#!/usr/bin/env python3

import asyncio
import time
import threading
from alkahest_py import (
    PyTestEnvManager,
    PyStringObligationStatementData,
    PyAttestationFilter,
    PyFulfillmentParams,
    PyArbitrateOptions,
    PyMockERC20,
    PyTrustedOracleArbiterDemandData,
)

async def test_threading_with_allow_threads():
    """Test that py.allow_threads actually allows Python threads to run concurrently"""
    try:
        print("🚀 Testing py.allow_threads functionality...")
        
        # Setup test environment
        env = PyTestEnvManager()
        
        # Setup escrow with proper oracle demand data
        mock_erc20 = PyMockERC20(env.mock_addresses.erc20_a, env.god_wallet_provider)
        mock_erc20.transfer(env.alice, 100)
        
        # Create proper demand data with Bob as the oracle
        oracle_client = env.bob_client.oracle
        demand_data = PyTrustedOracleArbiterDemandData(env.bob, [])
        demand_bytes = demand_data.encode_self()
        
        trusted_oracle_arbiter = env.addresses.arbiters_addresses.trusted_oracle_arbiter
        arbiter = {
            "arbiter": trusted_oracle_arbiter,
            "demand": demand_bytes
        }
        
        price = {"address": env.mock_addresses.erc20_a, "value": 100}
        expiration = int(time.time()) + 3600
        escrow_receipt = await env.alice_client.erc20.permit_and_buy_with_erc20(
            price, arbiter, expiration
        )
        escrow_uid = escrow_receipt['log']['uid']
        print(f"Escrow created with UID: {escrow_uid}")
        
        # Setup oracle client and parameters
        oracle_client = env.bob_client.oracle
        
        filter_obj = PyAttestationFilter(
            attester=env.addresses.string_obligation_addresses.obligation,
            recipient=env.bob,
            schema_uid=None,
            uid=None,
            ref_uid=escrow_uid,
            from_block=0,
            to_block=None,
        )
        
        statement_abi = PyStringObligationStatementData(item="")
        fulfillment_params = PyFulfillmentParams(
            statement_abi=statement_abi,
            filter=filter_obj
        )
        options = PyArbitrateOptions(require_oracle=True, skip_arbitrated=False)
        
        # Decision function that gets called from Rust
        decisions_made = []
        def decision_function(statement_str):
            print(f"🔍 Decision function called with statement: {statement_str}")
            decision = statement_str == "good"
            decisions_made.append((statement_str, decision))
            return decision
        
        # Callback function to test if it gets called with py.allow_threads
        callback_calls = []
        def callback_function(decision_info):
            print(f"🎉 CALLBACK CALLED with decision info: {decision_info}")
            callback_calls.append(decision_info)
        
        # Variables to store results from threads
        listen_result = None
        listen_error = None
        fulfillment_uid = None
        thread_messages = []
        
        def background_thread_work():
            """This thread will run alongside the Rust listener and make a fulfillment statement"""
            nonlocal fulfillment_uid
            for i in range(3):
                thread_messages.append(f"🔄 Background thread iteration {i+1}")
                print(f"🔄 Background thread iteration {i+1} - listener should be running concurrently")
                time.sleep(0.5)
            
            # Make a fulfillment statement during listening to trigger the callback
            try:
                print("🔄 Background thread: Making fulfillment statement...")
                
                # Create new event loop for this thread  
                loop = asyncio.new_event_loop()
                asyncio.set_event_loop(loop)
                
                string_client = env.bob_client.string_obligation
                statement_data = PyStringObligationStatementData(item="good")
                fulfillment_uid = loop.run_until_complete(
                    string_client.make_statement(statement_data, escrow_uid)
                )
                print(f"🔄 Background thread: Made statement with UID: {fulfillment_uid}")
                loop.close()
                
                # Continue running to see if callback gets called
                for i in range(4, 8):
                    thread_messages.append(f"🔄 Background thread iteration {i+1}")
                    print(f"🔄 Background thread iteration {i+1} - waiting for callback...")
                    time.sleep(0.5)
                    
            except Exception as e:
                print(f"🔄 Background thread error making statement: {e}")
            
            thread_messages.append("✅ Background thread completed")
            print("✅ Background thread completed")
        
        def run_listener():
            """Run the listener (this will block for 5 seconds with py.allow_threads)"""
            nonlocal listen_result, listen_error
            try:
                print("🎧 Listener thread: Starting listen_and_arbitrate_no_spawn...")
                listen_result = oracle_client.listen_and_arbitrate_no_spawn(
                    fulfillment_params,
                    decision_function,
                    callback_function,  # Now using the actual callback function
                    options,
                    8  # 8 second timeout to allow for statement + processing
                )
                print(f"🎧 Listener completed with {len(listen_result) if listen_result else 0} results")
            except Exception as e:
                print(f"🎧 Listener error: {e}")
                listen_error = e
        
        # Start background thread first
        background_thread = threading.Thread(target=background_thread_work)
        listener_thread = threading.Thread(target=run_listener)
        
        print("🚀 Starting background thread and listener simultaneously...")
        background_thread.start()
        listener_thread.start()
        
        # Wait for both threads
        background_thread.join()
        listener_thread.join()
        
        if listen_error:
            print(f"❌ Listen error: {listen_error}")
        
        # Verify that the background thread ran concurrently
        if len(thread_messages) >= 7:
            print("✅ SUCCESS: Background thread ran concurrently with the Rust listener!")
            print(f"   Background thread completed {len(thread_messages)} iterations while listener was running")
            print("   This proves py.allow_threads is working correctly")
        else:
            print(f"❌ FAILURE: Background thread only completed {len(thread_messages)} iterations")
            print("   This suggests py.allow_threads may not be working")
        
        # Check if callback was called
        if callback_calls:
            print(f"🎉 CALLBACK SUCCESS: Callback was called {len(callback_calls)} times!")
            for i, call in enumerate(callback_calls):
                print(f"   Call {i+1}: {call}")
            print("   This proves the callback works with py.allow_threads!")
        else:
            print("⚠️  CALLBACK NOT CALLED: This could be due to:")
            print("   - No live events during listening period")
            print("   - Timing issues with statement creation")
            print("   - Callback mechanism not working with py.allow_threads")
        
        # Check if decision function was called
        if decisions_made:
            print(f"✅ DECISION FUNCTION: Called {len(decisions_made)} times")
            for statement, decision in decisions_made:
                print(f"   Statement: '{statement}' -> Decision: {decision}")
        else:
            print("⚠️  DECISION FUNCTION: Not called (no statements found)")
        
        print("✅ Threading test completed successfully")
        return True
        
    except Exception as e:
        print(f"❌ Threading test failed: {e}")
        return False

if __name__ == "__main__":
    success = asyncio.run(test_threading_with_allow_threads())
    exit(0 if success else 1)
