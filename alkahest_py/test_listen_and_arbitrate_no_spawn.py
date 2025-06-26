import pytest
import time
import threading
from alkahest_py import (
    EnvTestManager,
    StringObligationStatementData,
    AttestationFilter,
    FulfillmentParams,
    ArbitrateOptions,
    MockERC20,
    TrustedOracleArbiterDemandData,
)

@pytest.mark.asyncio
async def test_listen_and_arbitrate_no_spawn():
    """Test complete listen_and_arbitrate_no_spawn flow with concurrent threading and callback verification"""
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
    print(f"Escrow created with UID: {escrow_uid}")
    
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
    
    statement_abi = StringObligationStatementData(item="")
    fulfillment_params = FulfillmentParams(
        statement_abi=statement_abi,
        filter=filter_obj
    )
    
    options = ArbitrateOptions(
        require_oracle=True,
        skip_arbitrated=False
    )
    
    # Decision function that approves "good" statements
    decisions_made = []
    def decision_function(statement_str):
        print(f"🔍 Decision function called with statement: {statement_str}")
        decision = statement_str == "good"
        decisions_made.append((statement_str, decision))
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
    string_client = env.bob_client.string_obligation
    
    # Function to run the listener in background
    def run_listener():
        nonlocal listen_result, listen_error
        try:
            print("🎧 Listener thread: Starting listen_and_arbitrate_no_spawn...")
            listen_result = oracle_client.listen_and_arbitrate_no_spawn(
                fulfillment_params,
                decision_function,
                callback_function,  # Pass the callback function
                options,
                10  # 10 second timeout to allow for concurrent fulfillment
            )
            print(f"🎧 Listener completed with {len(listen_result) if listen_result else 0} results")
        except Exception as e:
            listen_error = e
            print(f"🎧 Listener error: {e}")
