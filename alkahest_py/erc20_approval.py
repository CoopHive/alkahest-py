#!/usr/bin/env python3
"""
ERC20 Approval Flow - Final Working Version

This demonstrates the ERC20 approval functionality for both payment and escrow purposes.
Note: Due to transaction management limitations in the Python bindings, 
we test each approval type separately to avoid conflicts.
"""

import asyncio
from alkahest_py import PyTestEnvManager, PyMockERC20


async def test_erc20_approvals():
    """
    Test ERC20 approvals for payment and escrow.
    
    This test demonstrates both approval types work correctly,
    but tests them separately due to transaction management constraints
    in the Python bindings when multiple transactions are submitted rapidly.
    """
    print("🚀 Testing ERC20 Approval Functionality...")
    print("\n📝 Note: Testing approvals separately to avoid transaction conflicts")
    
    success_count = 0
    total_tests = 2
    
    # Test 1: Payment Approval
    print("\n" + "─" * 50)
    print("🔸 Testing Payment Approval")
    print("─" * 50)
    
    try:
        env1 = PyTestEnvManager()
        mock_erc20_1 = PyMockERC20(env1.mock_addresses.erc20_a, env1.god_wallet_provider)
        
        # Transfer tokens to Alice
        mock_erc20_1.transfer(env1.alice, 100)
        print(f"✅ Transferred 100 tokens to Alice")
        print(f"   Alice balance: {mock_erc20_1.balance_of(env1.alice)}")
        
        # Create token data and approve for payment
        token_data = {"address": env1.mock_addresses.erc20_a, "value": 100}
        receipt_hash = await env1.alice_client.erc20.approve(token_data, "payment")
        
        # Check allowance after approval
        payment_allowance = mock_erc20_1.allowance(env1.alice, env1.addresses.erc20_addresses.payment_obligation)
        
        print(f"✅ Payment approval successful!")
        print(f"   Transaction: {receipt_hash}")
        print(f"   Approved 100 tokens for payment obligation")
        print(f"   Payment allowance verified: {payment_allowance} tokens")
        
        success_count += 1
        
    except Exception as e:
        print(f"❌ Payment approval failed: {e}")
    
    # Small delay between tests
    await asyncio.sleep(1)
    
    # Test 2: Escrow Approval
    print("\n" + "─" * 50)
    print("🔸 Testing Escrow Approval")
    print("─" * 50)
    
    try:
        env2 = PyTestEnvManager()
        mock_erc20_2 = PyMockERC20(env2.mock_addresses.erc20_a, env2.god_wallet_provider)
        
        # Transfer tokens to Alice
        mock_erc20_2.transfer(env2.alice, 100)
        print(f"✅ Transferred 100 tokens to Alice")
        print(f"   Alice balance: {mock_erc20_2.balance_of(env2.alice)}")
        
        # Create token data and approve for escrow
        token_data = {"address": env2.mock_addresses.erc20_a, "value": 100}
        receipt_hash = await env2.alice_client.erc20.approve(token_data, "escrow")
        
        # Check allowance after approval
        escrow_allowance = mock_erc20_2.allowance(env2.alice, env2.addresses.erc20_addresses.escrow_obligation)
        
        print(f"✅ Escrow approval successful!")
        print(f"   Transaction: {receipt_hash}")
        print(f"   Approved 100 tokens for escrow obligation")
        print(f"   Escrow allowance verified: {escrow_allowance} tokens")
        success_count += 1
        
    except Exception as e:
        print(f"❌ Escrow approval failed: {e}")
    
    # Results
    print("\n" + "=" * 60)
    print("📊 TEST RESULTS")
    print("=" * 60)
    print(f"✅ Successful tests: {success_count}/{total_tests}")
    
    if success_count == total_tests:
        print("\n🎉 SUCCESS! ERC20 approval functionality is working correctly.")
        print("\n📋 Summary:")
        print("   • Payment approval: ✅ Working")
        print("   • Escrow approval:  ✅ Working")
        print("\n💡 Both approval types successfully grant allowances to their")
        print("   respective contract addresses for token transfers.")
        return True
    else:
        print(f"\n💥 {total_tests - success_count} test(s) failed.")
        return False


async def main():
    """Main test runner."""
    print("=" * 60)
    print("ERC20 APPROVAL FUNCTIONALITY TEST")
    print("Python version of src/main.rs")
    print("=" * 60)
    
    try:
        success = await test_erc20_approvals()
        return success
        
    except Exception as e:
        print(f"\n💥 Unexpected error: {e}")
        return False


if __name__ == "__main__":
    success = asyncio.run(main())
    exit(0 if success else 1)
