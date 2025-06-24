

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


async def test_listen_and_arbitrate_no_spawn():
    """Test complete listen_and_arbitrate_no_spawn flow: escrow → listen → fulfillment → arbitration → payment collection"""
    try:
        # Setup test environment
        env = PyTestEnvManager()
        
        # Setup escrow with proper oracle demand data
        mock_erc20 = PyMockERC20(env.mock_addresses.erc20_a, env.god_wallet_provider)
        mock_erc20.transfer(env.alice, 100)
        
        price = {"address": env.mock_addresses.erc20_a, "value": 100}
        trusted_oracle_arbiter = env.addresses.arbiters_addresses.trusted_oracle_arbiter
        
        # Create proper demand data with Bob as the oracle
        oracle_client = env.bob_client.oracle
        demand_data = PyTrustedOracleArbiterDemandData(env.bob, [])
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
        
        # Create filter and fulfillment params for listening
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
        
        options = PyArbitrateOptions(
            require_oracle=True,
            skip_arbitrated=False
        )
        
        # Decision function that approves "good" statements
        decisions_made = []
        def decision_function(statement_str):
            print(f"Decision function called with statement: {statement_str}")
            decision = statement_str == "good"
            decisions_made.append((statement_str, decision))
            return decision
        
        # Variables to store results
        listen_result = None
        listen_error = None
        
        # Create async tasks for both listening and making statement
        async def listen_task():
            nonlocal listen_result, listen_error
            try:
                print("Starting listen_and_arbitrate_no_spawn...")
                # Run in a thread since the method is synchronous
                loop = asyncio.get_event_loop()
                listen_result = await loop.run_in_executor(
                    None,
                    lambda: oracle_client.listen_and_arbitrate_no_spawn(
                        fulfillment_params,
                        decision_function,
                        options,
                        timeout_seconds=3  # 10 second timeout
                    )
                )
                print(f"Listen completed with {len(listen_result) if listen_result else 0} results")
            except Exception as e:
                print(f"Listen thread exception: {e}")
                listen_error = e
        
        async def make_statement_task():
            # Small delay to let listener start
            await asyncio.sleep(0.1)
            print("Making fulfillment statement...")
            string_client = env.bob_client.string_obligation
            statement_data = PyStringObligationStatementData(item="good")
            fulfillment_uid = await string_client.make_statement(statement_data, escrow_uid)
            print(f"Made statement with UID: {fulfillment_uid}")
            return fulfillment_uid
        
        # Run both tasks concurrently
        print("Starting concurrent listen and statement...")
        listen_coro = listen_task()
        statement_coro = make_statement_task()
        
        # Wait for statement to complete, and then wait a bit more for listen to process
        fulfillment_uid = await statement_coro
        await asyncio.sleep(0.2)  # Give listener time to process
        
        # The listen task should complete soon after the statement
        try:
            await asyncio.wait_for(listen_coro, timeout=15)
        except asyncio.TimeoutError:
            raise Exception("Listen task timed out")
        
        if listen_error:
            raise Exception(f"Listen thread failed: {listen_error}")
        
        if not listen_result:
            raise Exception("No result from listen_and_arbitrate_no_spawn")
        
        if not listen_result:
            raise Exception("No result from listen_and_arbitrate_no_spawn")
        
        # Verify decisions were made
        if len(decisions_made) != 1:
            raise Exception(f"Expected 1 decision, got {len(decisions_made)}")
        
        statement_str, decision = decisions_made[0]
        if statement_str != "good" or not decision:
            raise Exception(f"Wrong decision: statement='{statement_str}', decision={decision}")
        
        # Verify listen result
        if len(listen_result) != 1:
            raise Exception(f"Expected 1 decision in result, got {len(listen_result)}")
        
        result_decision = listen_result[0]
        if not result_decision.decision or result_decision.statement_data != "good":
            raise Exception(f"Result decision incorrect: {result_decision.decision}, statement: {result_decision.statement_data}")
        
        # Wait for arbitration transactions to be processed
        time.sleep(2)
        
        # Collect payment
        collection_receipt = await env.bob_client.erc20.collect_payment(
            escrow_uid, fulfillment_uid
        )
        
        # Verify collection receipt
        if not collection_receipt or not collection_receipt.startswith('0x'):
            raise Exception(f"Invalid collection receipt: {collection_receipt}")
        
        print("✅ test_listen_and_arbitrate_no_spawn PASSED")
        return True
        
    except Exception as e:
        print(f"❌ test_listen_and_arbitrate_no_spawn FAILED: {e}")
        return False





if __name__ == "__main__":
    async def run_tests():
        print("Running listen_and_arbitrate_no_spawn tests...")
        success1 = await test_listen_and_arbitrate_no_spawn()
        return success1 
    
    success = asyncio.run(run_tests())
    exit(0 if success else 1)
