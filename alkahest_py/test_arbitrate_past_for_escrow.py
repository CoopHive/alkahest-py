#!/usr/bin/env python3
"""
Test the Oracle arbitrate_past_for_escrow functionality with complete escrow, arbitration, and payment flow
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
    PyEscrowParams,
    PyEscrowArbitrationResult,
)


async def test_arbitrate_past_for_escrow():
    """Test complete arbitrate_past_for_escrow flow: escrow → fulfillment → arbitration → payment collection"""
    try:
        # Setup test environment
        env = PyTestEnvManager()
        
        # Setup escrow with proper oracle demand data
        mock_erc20 = PyMockERC20(env.mock_addresses.erc20_a, env.god_wallet_provider)
        mock_erc20.transfer(env.alice, 100)
        
        price = {"address": env.mock_addresses.erc20_a, "value": 100}
        trusted_oracle_arbiter = env.addresses.arbiters_addresses.trusted_oracle_arbiter
        
        # Create proper demand data with Bob as the oracle
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
        
        # Setup escrow parameters for arbitration
        escrow_filter = PyAttestationFilter(
            attester=env.addresses.erc20_addresses.escrow_obligation,
            recipient=None,
            schema_uid=None,
            uid=None,
            ref_uid=None,
            from_block=0,
            to_block=None
        )
        escrow_params = PyEscrowParams(demand_bytes, escrow_filter)
        
        # Setup fulfillment parameters for arbitration
        fulfillment_filter = PyAttestationFilter(
            attester=env.addresses.string_obligation_addresses.obligation,
            recipient=env.bob,
            schema_uid=None,
            uid=None,
            ref_uid=escrow_uid,
            from_block=0,
            to_block=None
        )
        fulfillment_params = PyFulfillmentParams(statement_data, fulfillment_filter)
        
        # Setup arbitration options
        options = PyArbitrateOptions(require_oracle=True, skip_arbitrated=False)
        
        # Define decision function that receives both statement and demand data
        def decision_function(statement_str, demand_data):
            print(f"Decision function called with statement: '{statement_str}' and oracle: {demand_data.oracle}")
            return statement_str == "good"
        
        # Call arbitrate_past_for_escrow
        oracle_client = env.bob_client.oracle
        result = oracle_client.arbitrate_past_for_escrow(
            escrow_params,
            fulfillment_params,
            decision_function,
            options
        )
        
        print(f"Arbitration result: {result}")
        print(f"Decisions made: {len(result.decisions)}")
        print(f"Escrow attestations: {len(result.escrow_attestations)}")
        print(f"Escrow demands: {len(result.escrow_demands)}")
        
        # Verify we got positive arbitration decisions
        assert len(result.decisions) > 0, "Should have arbitration decisions"
        positive_decisions = [d for d in result.decisions if d.decision]
        assert len(positive_decisions) > 0, "Should have positive arbitration decisions"
        
        # Collect payment for Bob
        collection_receipt = await env.bob_client.erc20.collect_payment(escrow_uid, fulfillment_uid)
        print(f"Payment collected successfully: {collection_receipt}")
        
        # Verify Bob received payment
        final_balance = mock_erc20.balance_of(env.bob)
        print(f"Bob's final balance: {final_balance}")
        assert final_balance == 100, f"Bob should have received 100 tokens, but has {final_balance}"
        
        print("✅ test_arbitrate_past_for_escrow PASSED")
        return True
        
    except Exception as e:
        print(f"❌ test_arbitrate_past_for_escrow FAILED: {e}")
        import traceback
        traceback.print_exc()
        return False


def main():
    """Main function to run the test"""
    result = asyncio.run(test_arbitrate_past_for_escrow())
    if result:
        print("✅ Oracle Arbitrate Past For Escrow PASSED")
    else:
        print("❌ Oracle Arbitrate Past For Escrow FAILED")
        exit(1)


if __name__ == "__main__":
    main()
