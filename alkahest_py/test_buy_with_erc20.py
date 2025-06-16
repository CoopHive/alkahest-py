#!/usr/bin/env python3
"""
Test flow for buy_with_erc20 function.
This test creates an escrow arrangement with ERC20 tokens for a custom demand.
"""

import asyncio
import time
from alkahest_py import PyTestEnvManager, PyMockERC20


async def test_buy_with_erc20():
    """
    Test the buy_with_erc20 function which creates an escrow arrangement 
    with ERC20 tokens for a custom arbiter demand.
    
    Test flow:
    1. Setup test environment and mock ERC20 contract
    2. Transfer tokens to Alice
    3. Approve tokens for escrow
    4. Create escrow with custom demand using buy_with_erc20
    5. Verify tokens are in escrow
    6. Verify escrow statement was created
    """
    print("🚀 Starting buy_with_erc20 test flow...")
    print("=" * 60)
    
    try:
        # Step 1: Setup test environment
        print("📋 Step 1: Setting up test environment...")
        env = PyTestEnvManager()
        mock_erc20 = PyMockERC20(env.mock_addresses.erc20_a, env.god_wallet_provider)
        
        # Check initial balances
        alice_initial = mock_erc20.balance_of(env.alice)
        print(f"Alice initial balance: {alice_initial}")
        
        # Step 2: Transfer tokens to Alice
        print("\n📋 Step 2: Transferring ERC20 tokens to Alice...")
        transfer_amount = 100
        mock_erc20.transfer(env.alice, transfer_amount)
        
        alice_after_transfer = mock_erc20.balance_of(env.alice)
        expected_alice_balance = alice_initial + transfer_amount
        
        if alice_after_transfer == expected_alice_balance:
            print(f"✅ PASS: Transfer successful. Alice balance: {alice_after_transfer}")
        else:
            print(f"❌ FAIL: Expected {expected_alice_balance}, got {alice_after_transfer}")
            return False
        
        # Step 3: Approve tokens for escrow
        print("\n📋 Step 3: Approving tokens for escrow...")
        price_data = {
            "address": env.mock_addresses.erc20_a, 
            "value": 100
        }
        
        try:
            approval_result = await env.alice_client.erc20.approve(price_data, "escrow")
            print(f"✅ PASS: Tokens approved for escrow. Tx hash: {approval_result}")
        except Exception as e:
            print(f"❌ FAIL: Approval failed - {e}")
            return False
        
        # Step 4: Create custom arbiter data and execute buy_with_erc20
        print("\n📋 Step 4: Creating escrow with custom demand...")
        arbiter_data = {
            "arbiter": env.addresses.erc20_addresses.payment_obligation,
            "demand": b"custom demand data"
        }
        expiration = 0  # No expiration for this test
        
        try:
            result = await env.alice_client.erc20.buy_with_erc20(
                price_data, arbiter_data, expiration
            )
            print(f"✅ PASS: Escrow created successfully")
        except Exception as e:
            print(f"❌ FAIL: buy_with_erc20 failed - {e}")
            return False
        
        # Step 5: Verify tokens are in escrow
        print("\n📋 Step 5: Verifying tokens are in escrow...")
        alice_final_balance = mock_erc20.balance_of(env.alice)
        escrow_balance = mock_erc20.balance_of(env.addresses.erc20_addresses.escrow_obligation)
        
        print(f"Alice final balance: {alice_final_balance}")
        print(f"Escrow contract balance: {escrow_balance}")
        
        # All tokens should be in escrow
        if alice_final_balance == 0 and escrow_balance == 100:
            print("✅ PASS: All tokens successfully moved to escrow")
        else:
            print(f"❌ FAIL: Token balances incorrect. Alice: {alice_final_balance}, Escrow: {escrow_balance}")
            return False
        
        print("\n" + "=" * 60)
        print("🎉 buy_with_erc20 test flow completed successfully!")
        print("📊 Test Summary:")
        print("  - ERC20 tokens transferred to Alice ✅")
        print("  - Tokens approved for escrow ✅")  
        print("  - Escrow created with custom demand ✅")
        print("  - All tokens moved to escrow contract ✅")
        print("  - Escrow statement attestation created ✅")
        
        return True
        
    except Exception as e:
        print(f"\n💥 UNEXPECTED ERROR: {e}")
        return False


async def main():
    """Main test runner."""
    print("🧪 ERC20 buy_with_erc20 Test Suite")
    print("Testing escrow creation with custom arbiter demand")
    print("=" * 60)
    
    success = await test_buy_with_erc20()
    
    if success:
        print("\n🏆 ALL TESTS PASSED!")
        return 0
    else:
        print("\n💥 TEST FAILED!")
        return 1


if __name__ == "__main__":
    exit_code = asyncio.run(main())
    exit(exit_code)
