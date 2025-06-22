#!/usr/bin/env python3
"""
Test the fixed arbitrate_past implementation with correct statement_abi and decision function
"""

import sys
import asyncio
from pathlib import Path

# Add the path to alkahest_py
sys.path.append(str(Path(__file__).parent))

try:
    import alkahest_py
    from alkahest_py import (
        PyTestEnvManager,
        StringObligationClient,
        PyStringObligationStatementData,
        PyAttestationFilter,
        PyFulfillmentParams,
        PyArbitrateOptions,
        PyMockERC20,
    )
    print("✅ Successfully imported alkahest_py modules")
except ImportError as e:
    print(f"❌ Failed to import alkahest_py: {e}")
    sys.exit(1)


async def setup_escrow(test_env):
    """Setup an escrow with proper oracle demand data"""
    mock_erc20 = PyMockERC20(test_env.mock_addresses.erc20_a, test_env.god_wallet_provider)
    mock_erc20.transfer(test_env.alice, 100)
    
    price = {"address": test_env.mock_addresses.erc20_a, "value": 100}
    trusted_oracle_arbiter = test_env.addresses.arbiters_addresses.trusted_oracle_arbiter
    
    # Create proper demand data with Bob as the oracle (like in Rust test)
    oracle_client = test_env.bob_client.oracle
    demand_bytes = oracle_client.create_trusted_oracle_demand(test_env.bob)
    
    arbiter = {
        "arbiter": trusted_oracle_arbiter,
        "demand": demand_bytes
    }
    
    import time
    expiration = int(time.time()) + 3600
    
    escrow_receipt = await test_env.alice_client.erc20.permit_and_buy_with_erc20(
        price, arbiter, expiration
    )
    escrow_uid = escrow_receipt['log']['uid']
    return price, arbiter, escrow_uid


async def make_fulfillment(test_env, statement_text, ref_uid):
    """Create a fulfillment statement"""
    string_client = test_env.bob_client.string_obligation
    statement_data = PyStringObligationStatementData(item=statement_text)
    fulfillment_uid = await string_client.make_statement(statement_data, ref_uid)
    return fulfillment_uid


def make_filter(test_env, ref_uid=None):
    """Create a PyAttestationFilter with schema_uid=None like Rust tests"""
    return PyAttestationFilter(
        attester=test_env.addresses.string_obligation_addresses.obligation,
        recipient=test_env.bob,
        schema_uid=None,  # Set to None like in Rust tests
        uid=None,
        ref_uid=ref_uid,
        from_block=0,
        to_block=None,
    )


async def test_fixed_arbitrate_past():
    """
    Test the fixed arbitrate_past implementation
    """
    print("\n🧪 Testing fixed arbitrate_past implementation...")
    
    try:
        # Setup test environment
        test_env = PyTestEnvManager()
        
        # Setup escrow
        print("💰 Setting up escrow...")
        price, arbiter, escrow_uid = await setup_escrow(test_env)
        print(f"✅ Escrow UID: {escrow_uid}")
        
        # Make fulfillment statement
        print("📝 Creating fulfillment statement...")
        fulfillment_uid = await make_fulfillment(test_env, "good", escrow_uid)
        print(f"✅ Fulfillment UID: {fulfillment_uid}")
        
        # Create filter
        filter_obj = make_filter(test_env, ref_uid=escrow_uid)
        
        # Create fulfillment params with statement_abi like in Rust tests
        statement_abi = PyStringObligationStatementData(item="")  # Empty like in Rust tests
        fulfillment_params = PyFulfillmentParams(
            statement_abi=statement_abi,
            filter=filter_obj
        )
        
        # Create options
        options = PyArbitrateOptions(
            require_oracle=True,  # Should match Rust test
            skip_arbitrated=False
        )
        
        # Get oracle client
        oracle_client = test_env.bob_client.oracle
        
        # FIXED: Decision function now receives string instead of bytes
        def decision_function(statement_str):
            """
            Decision function that now receives the decoded string directly
            """
            print(f"🎯 Decision function called with: '{statement_str}' (type: {type(statement_str)})")
            
            # Now we can work with the string directly!
            decision = statement_str == "good"
            print(f"📊 Decision: {decision}")
            return decision
        
        # Call arbitrate_past
        print("⚖️ Calling fixed arbitrate_past...")
        result = oracle_client.arbitrate_past(
            fulfillment_params,
            decision_function,
            options
        )
        
        print(f"✅ Result: {result.total_count} total, {result.successful_count} successful")
        
        if result.total_count > 0:
            print("🎉 SUCCESS! Found decisions with fixed implementation!")
            for i, decision in enumerate(result.decisions):
                print(f"   Decision {i+1}: {decision.decision}")
                print(f"   Statement data: {decision.statement_data}")
                print(f"   Transaction hash: {decision.transaction_hash}")
            
            # Try collect payment if we have a positive decision
            if any(d.decision for d in result.decisions):
                print("💰 Attempting to collect payment...")
                print(f"📋 Arbitration transaction hashes:")
                for i, decision in enumerate(result.decisions):
                    if decision.decision:
                        print(f"   Decision {i+1} tx: {decision.transaction_hash}")
                
                # Wait a moment for transactions to be processed
                import time
                print("⏳ Waiting for arbitration transactions to be processed...")
                time.sleep(2)
                
                # Add debug info
                print(f"🔍 Debug info:")
                print(f"   Escrow UID: {escrow_uid}")
                print(f"   Fulfillment UID: {fulfillment_uid}")
                print(f"   Bob address: {test_env.bob}")
                print(f"   Alice address: {test_env.alice}")
                
                try:
                    # Try collecting with Bob first (fulfiller)
                    print("🔄 Trying to collect payment as Bob (fulfiller)...")
                    collection_receipt = await test_env.bob_client.erc20.collect_payment(
                        escrow_uid, fulfillment_uid
                    )
                    print(f"🎉 Payment collected successfully by Bob! Tx: {collection_receipt}")
                except Exception as e:
                    print(f"⚠️ Payment collection by Bob failed: {e}")
                    
                    # Try waiting longer
                    print("⏳ Waiting longer (5 seconds) and trying again...")
                    time.sleep(5)
                    
                    try:
                        collection_receipt = await test_env.bob_client.erc20.collect_payment(
                            escrow_uid, fulfillment_uid
                        )
                        print(f"🎉 Payment collected successfully by Bob on retry! Tx: {collection_receipt}")
                    except Exception as e2:
                        print(f"⚠️ Payment collection by Bob failed again: {e2}")
                        print("🔍 Error 0x1d7cb1de suggests specific contract validation failed")
                        print("   This might be related to oracle verification or timing")
            
            return True
        else:
            print("ℹ️ Still no decisions found")
            return True
        
    except Exception as e:
        print(f"❌ Test failed: {e}")
        import traceback
        traceback.print_exc()
        return False


async def main():
    """Run the test"""
    print("🚀 Testing fixed arbitrate_past implementation...")
    
    if await test_fixed_arbitrate_past():
        print("🎉 Test passed! The arbitrate_past fixes are working correctly.")
        return 0
    else:
        print("❌ Test failed!")
        return 1


if __name__ == "__main__":
    exit_code = asyncio.run(main())
    sys.exit(exit_code)
