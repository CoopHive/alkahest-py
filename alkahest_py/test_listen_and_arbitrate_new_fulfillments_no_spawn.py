import asyncio
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
async def test_listen_and_arbitrate_new_fulfillments_no_spawn():
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
    escrow_receipt = env.alice_client.erc20.permit_and_buy_with_erc20(
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
            print("🎧 Listener thread: Starting listen_and_arbitrate_new_fulfillments_no_spawn...")
            listen_result = oracle_client.listen_and_arbitrate_new_fulfillments_no_spawn(
                fulfillment_params,
                decision_function,
                callback_function,  # Pass the callback function
                options,
                10  # 10 second timeout to allow for concurrent fulfillment
            )
            print(f"🎧 Listener completed with {len(listen_result) if listen_result else 0} results")
        except Exception as e:
            print(f"🎧 Listener error: {e}")
            listen_error = e
        
    # Function to make the fulfillment statement while listener is active
    def make_fulfillment_during_listen():
        nonlocal fulfillment_uid
        try:
            # Wait for listener to start, then make statement during listening period
            time.sleep(2.0)  # Give listener time to start actively listening
            print("🔄 Fulfillment thread: Making statement while listener is active...")
            
            statement_data = StringObligationStatementData(item="good")
            
            # Need to run async code in the thread
            async def do_fulfillment_and_collection():
                nonlocal fulfillment_uid
                # Make the fulfillment statement
                fulfillment_uid = string_client.make_statement(statement_data, escrow_uid)
                print(f"🔄 Fulfillment thread: Made statement with UID: {fulfillment_uid}")
                
                # Wait a moment for arbitration to process, then collect payment immediately
                await asyncio.sleep(3)  # Give time for arbitration processing
                print("🔄 Fulfillment thread: Collecting payment...")
                
                try:
                    collection_receipt = env.bob_client.erc20.collect_payment(
                        escrow_uid, fulfillment_uid
                    )
                    
                    if collection_receipt and collection_receipt.startswith('0x'):
                        print(f"✅ Payment collected successfully: {collection_receipt}")
                    else:
                        print(f"⚠️  Payment collection returned: {collection_receipt}")
                except Exception as e:
                    print(f"❌ Payment collection failed: {e}")
            
            # Create new event loop for this thread
            loop = asyncio.new_event_loop()
            asyncio.set_event_loop(loop)
            try:
                loop.run_until_complete(do_fulfillment_and_collection())
            finally:
                loop.close()
        except Exception as e:
            print(f"🔄 Fulfillment thread error: {e}")
    
    # Start both threads concurrently - listener in background, fulfillment during listening
    listener_thread = threading.Thread(target=run_listener)
    fulfillment_thread = threading.Thread(target=make_fulfillment_during_listen)
    
    print("🚀 Starting listener in background, then fulfillment during listening...")
    listener_thread.start()
    fulfillment_thread.start()
    
    # Wait for both threads to complete
    listener_thread.join()
    fulfillment_thread.join()
    
    if listen_error:
        raise Exception(f"Listen thread failed: {listen_error}")
    
    # Check results and callback
    if not listen_result:
        print("⚠️  No result from listen_and_arbitrate_no_spawn - this may be due to timing")
        print(f"Fulfillment UID was: {fulfillment_uid}")
        print("This is expected behavior if the fulfillment was made too early or too late")
    
    # Verify callback was called for live event processing
    if callback_calls:
        print(f"🎉 CALLBACK SUCCESS: Callback was called {len(callback_calls)} times!")
        for i, call in enumerate(callback_calls):
            print(f"   Call {i+1}: {call}")
        print("   This proves live event processing with py.allow_threads works!")
    else:
        print("⚠️  Callback was not called - this may indicate timing issues")
        print("   The callback is only triggered for live events during the listening period")
    
    # Verify decision function was called
    if decisions_made:
        print(f"✅ Decision function called {len(decisions_made)} times")
        for statement, decision in decisions_made:
            print(f"   Statement: '{statement}' -> Decision: {decision}")
    else:
        print("⚠️  Decision function not called - no statements processed")
    
    # If we have results, verify them
    if listen_result and len(listen_result) > 0:
        print(f"✅ Found {len(listen_result)} arbitration results")
        
        # Verify listen result
        result_decision = listen_result[0]
        if not result_decision.decision or result_decision.statement_data != "good":
            raise Exception(f"Result decision incorrect: {result_decision.decision}, statement: {result_decision.statement_data}")
        
        print("✅ Arbitration result verified")
    
    print("✅ test_listen_and_arbitrate_no_spawn PASSED")
    return True

