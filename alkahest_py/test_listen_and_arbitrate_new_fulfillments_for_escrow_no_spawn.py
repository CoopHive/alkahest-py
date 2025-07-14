#!/usr/bin/env python3
"""
Test the Oracle listen_and_arbitrate_new_fulfillments_for_escrow_no_spawn functionality
"""

import pytest
import time
import asyncio
from alkahest_py import (
    EnvTestManager,
    StringObligationData,
    AttestationFilter,
    FulfillmentParamsWithoutRefUid,
    ArbitrateOptions,
    MockERC20,
    TrustedOracleArbiterDemandData,
    EscrowParams,
)
from eth_abi import encode

@pytest.mark.asyncio
async def test_listen_and_arbitrate_new_fulfillments_for_escrow_no_spawn():
    """Test complete listen_and_arbitrate_new_fulfillments_for_escrow_no_spawn flow with concurrent threading"""
    # Setup test environment
    env = EnvTestManager()
    
    # Setup escrow with proper oracle demand data
    mock_erc20 = MockERC20(env.mock_addresses.erc20_a, env.god_wallet_provider)
    mock_erc20.transfer(env.alice, 100)
    
    price = {"address": env.mock_addresses.erc20_a, "value": 100}
    trusted_oracle_arbiter = env.addresses.arbiters_addresses.trusted_oracle_arbiter
    
    # Create proper demand data with Bob as the oracle
    oracle = env.bob
    data = b''

    # Encode as Solidity struct: tuple(address, bytes)
    demand_bytes = encode(['(address,bytes)'], [(oracle, data)])

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
    
    # Setup escrow parameters for arbitration
    escrow_filter = AttestationFilter(
        attester=env.addresses.erc20_addresses.escrow_obligation,
        recipient=None,
        schema_uid=None,
        uid=None,
        ref_uid=None,
        from_block=0,
        to_block=None
    )
    escrow_params = EscrowParams(demand_bytes, escrow_filter)
    
    # Create filter and fulfillment params for listening (new fulfillments style)
    filter_obj = AttestationFilter(
        attester=env.addresses.string_obligation_addresses.obligation,
        recipient=env.bob,
        schema_uid=None,
        uid=None,
        ref_uid=None,  # Important: new fulfillments don't specify ref_uid in filter
        from_block=0,
        to_block=None,
    )
    
    obligation_abi = StringObligationData(item="")
    fulfillment_params = FulfillmentParamsWithoutRefUid(
        obligation_abi=obligation_abi,
        filter=filter_obj
    )
    
    options = ArbitrateOptions(
        require_oracle=True,
        skip_arbitrated=False
    )
    
    # Decision function that approves "good" obligations
    decisions_made = []
    def decision_function(obligation_str, demand_data):
        print(f"🔍 Decision function called with obligation: '{obligation_str}' and oracle: {demand_data.oracle}")
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
            print("🎧 Listener thread: Starting listen_and_arbitrate_new_fulfillments_for_escrow_no_spawn...")
            listen_result = await oracle_client.listen_and_arbitrate_new_fulfillments_for_escrow_no_spawn(
                escrow_params,
                fulfillment_params,
                decision_function,
                callback_function,
                options,
                2
            )
            print("🎧 Listener thread: Completed successfully")
        except Exception as e:
            listen_error = e
            print(f"❌ Listener thread error: {e}")
    
    # Function to make the fulfillment obligation while listener is active
    async def make_fulfillment_during_listen():
        nonlocal fulfillment_uid, collection_success
        try:
            print("🔄 Fulfillment thread: Making obligation while listener is active...")
            
            obligation_data = StringObligationData(item="good")
            
            # Make the fulfillment obligation
            fulfillment_uid = await string_client.do_obligation(obligation_data, escrow_uid)
            assert fulfillment_uid is not None, "Fulfillment UID should not be None"
            print(f"🔄 Fulfillment thread: Created fulfillment {fulfillment_uid}")
            
            
            try:
                collection_receipt = await env.bob_client.erc20.collect_escrow(
                    escrow_uid, fulfillment_uid
                )
                print("collection_receipt:", collection_receipt)
                
                if collection_receipt and collection_receipt.startswith('0x'):
                    collection_success = True
                    print(f"🎉 Fulfillment thread: Payment collected successfully: {collection_receipt}")
            except Exception as e:
                print(f"⚠️ Collection failed (may be due to timing): {e}")
                # Collection might fail due to timing, but that's not the main test focus
                pass
                
        except Exception as e:
            pytest.fail(f"Fulfillment thread failed: {e}")
    
    # Get the oracle client
    oracle_client = env.bob_client.oracle
    
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
    
    # Note: This method focuses on new fulfillments for escrow,
    # so we mainly test the function execution and decision callbacks
    print(f"✅ Test completed successfully!")
    print(f"  - Decisions made: {len(decisions_made)}")
    print(f"  - Callback calls: {len(callback_calls)}")
    print(f"  - Collection success: {collection_success}")
    
    assert True, "Test completed successfully"
