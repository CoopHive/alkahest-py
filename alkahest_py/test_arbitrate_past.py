#!/usr/bin/env python3
"""
Test the Oracle arbitrate_past functionality with complete escrow, arbitration, and payment flow
"""

import asyncio
import time
from alkahest_py import (
    PyTestEnvManager,
    PyStringObligationStatementData,
    PyAttestationFilter,
    PyFulfillmentParams,
    PyArbitrateOptions,
    PyMockERC20,
    PyTrustedOracleArbiterDemandData,
)


async def test_arbitrate_past():
    """Test complete arbitrate_past flow: escrow → fulfillment → arbitration → payment collection"""
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
        
        # Make fulfillment statement
        string_client = env.bob_client.string_obligation
        statement_data = PyStringObligationStatementData(item="good")
        fulfillment_uid = await string_client.make_statement(statement_data, escrow_uid)
        
        # Create filter and fulfillment params
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
        def decision_function(statement_str):
            print(f"Decision function called with statement: {statement_str}")
            return statement_str == "good"
        
        # Call arbitrate_past
        result = oracle_client.arbitrate_past(
            fulfillment_params,
            decision_function,
            options
        )
        
        # Verify arbitration found decisions
        if result.total_count != 1 or result.successful_count != 1:
            raise Exception(f"Expected 1 successful decision, got {result.successful_count}/{result.total_count}")
        
        # Verify decision details
        decision = result.decisions[0]
        if not decision.decision or decision.statement_data != "good":
            raise Exception(f"Decision incorrect: {decision.decision}, statement: {decision.statement_data}")
        
        # Wait for arbitration transactions to be processed
        time.sleep(2)
        
        # Collect payment
        collection_receipt = await env.bob_client.erc20.collect_payment(
            escrow_uid, fulfillment_uid
        )
        
        # Verify collection receipt
        if not collection_receipt or not collection_receipt.startswith('0x'):
            raise Exception(f"Invalid collection receipt: {collection_receipt}")
        
        print("✅ test_arbitrate_past PASSED")
        return True
        
    except Exception as e:
        print(f"❌ test_arbitrate_past FAILED: {e}")
        return False


# For compatibility with run_all_tests.py
async def test_fixed_arbitrate_past():
    """Alias for the main test function to maintain compatibility"""
    return await test_arbitrate_past()


if __name__ == "__main__":
    success = asyncio.run(test_arbitrate_past())
    exit(0 if success else 1)
