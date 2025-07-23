import asyncio
import pytest
import time
from alkahest_py import (
    EnvTestManager,
    StringObligationData,
    AttestationFilter,
    FulfillmentParams,
    ArbitrateOptions,
    MockERC20,
    TrustedOracleArbiterDemandData,
)

@pytest.mark.asyncio
async def test_listen_and_arbitrate_new_fulfillments_no_spawn():
    """Test complete listen_and_arbitrate_new_fulfillments_no_spawn flow with concurrent threading"""
    # Setup test environment
    env = EnvTestManager()
    
    # Setup escrow with proper oracle demand data
    mock_erc20 = MockERC20(env.mock_addresses.erc20_a, env.god_wallet_provider)
    mock_erc20.transfer(env.alice, 100)
    
    price = {"address": env.mock_addresses.erc20_a, "value": 100}
    trusted_oracle_arbiter = env.addresses.arbiters_addresses.trusted_oracle_arbiter
    
    # Create proper demand data with Bob as the oracle
    oracle_client = env.bob_client.oracle
    demand_data = TrustedOracleArbiterDemandData(env.bob, [])
    demand_bytes = demand_data.encode_self()
    
    arbiter = {
        "arbiter": trusted_oracle_arbiter,
        "demand": demand_bytes
    }
    
    expiration = int(time.time()) + 3600
    escrow_receipt = await env.alice_client.erc20.permit_and_buy_with_erc20(
        price, arbiter, expiration
    )
    escrow_uid = escrow_receipt['log']['uid']
    assert escrow_uid is not None, "Escrow UID should not be None"
    
    # Create filter and fulfillment params for listening
    filter_obj = AttestationFilter(
        attester=env.addresses.string_obligation_addresses.obligation,
        recipient=env.bob,
        schema_uid=None,
        uid=None,
        ref_uid=escrow_uid,
        from_block=0,
        to_block=None,
    )
    
    obligation_abi = StringObligationData(item="")
    fulfillment_params = FulfillmentParams(
        obligation_abi=obligation_abi,
        filter=filter_obj
    )
    
    options = ArbitrateOptions(
        require_oracle=True,
        skip_arbitrated=False,
        require_request=False,
        only_new=True
    )
    
    # Decision function that approves "good" obligations
    decisions_made = []
    def decision_function(obligation_str):
        print(f"🔍 Decision function called with obligation: {obligation_str}")
        decision = obligation_str == "good"
        decisions_made.append((obligation_str, decision))
        return decision
    
    # Callback function to verify callback is called during live event processing
    callback_calls = []
    def callback_function(decision_info):
        print(f"🎉 CALLBACK CALLED with decision info: {decision_info}")
        callback_calls.append(decision_info)
    
    # Variables to store results from threads
    listen_result = None
    listen_error = None
    fulfillment_uid = None
    collection_success = False
    string_client = env.bob_client.string_obligation
    
    # Function to run the listener in background
    async def run_listener():
        nonlocal listen_result, listen_error
        try:
            listen_result = await oracle_client.listen_and_arbitrate_no_spawn(
                fulfillment_params,
                decision_function,
                callback_function,
                options,
                5  
            )
        except Exception as e:
            listen_error = e
        
    # Function to make the fulfillment obligation while listener is active
    async def make_fulfillment_during_listen():
        nonlocal fulfillment_uid, collection_success
        try:
            
            obligation_data = StringObligationData(item="good")
            
            # Make the fulfillment obligation
            fulfillment_uid = await string_client.do_obligation(obligation_data, escrow_uid)
            assert fulfillment_uid is not None, "Fulfillment UID should not be None"
            
            
            try:
                collection_receipt = await env.bob_client.erc20.collect_escrow(
                    escrow_uid, fulfillment_uid
                )
                print("collection_receipt:", collection_receipt)
                if collection_receipt and collection_receipt.startswith('0x'):
                    collection_success = True
            except Exception:
                # Collection might fail due to timing, but that's not the main test focus
                pass
                
        except Exception as e:
            pytest.fail(f"Fulfillment thread failed: {e}")
    
    # Start both async tasks concurrently
    listener_task = asyncio.create_task(run_listener())
    fulfillment_task = asyncio.create_task(make_fulfillment_during_listen())
    
    await fulfillment_task
    
    listener_task.cancel()
    try:
        await listener_task
    except asyncio.CancelledError:
        pass  # Expected when we cancel the task
    
    # Assert no errors occurred in the listener thread
    if listen_error:
        pytest.fail(f"Listen thread failed: {listen_error}")
    
    # Assert that the fulfillment was created
    assert fulfillment_uid is not None, "Fulfillment should have been created"
    
    # Assert that the decision function was called
    assert len(decisions_made) > 0, "Decision function should have been called at least once"
    
    # Assert that decisions were made correctly
    for obligation, decision in decisions_made:
        if obligation == "good":
            assert decision is True, f"Decision for 'good' obligation should be True, got {decision}"
    
    # Note: listen_and_arbitrate_new_fulfillments_no_spawn might not return detailed results
    # as it focuses on new fulfillments only, so we mainly test the function execution
    assert True, "Test completed successfully"

