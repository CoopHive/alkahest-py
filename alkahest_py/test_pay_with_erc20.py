#!/usr/bin/env python3
"""
Test flow for pay_with_erc20 functionality.
This test demonstrates the complete payment flow using ERC20 tokens.
"""

import asyncio
from alkahest_py import PyTestEnvManager, PyMockERC20


async def test_pay_with_erc20():
    
    try:
        env = PyTestEnvManager()
        mock_erc20 = PyMockERC20(env.mock_addresses.erc20_a, env.god_wallet_provider)
        
        alice_initial = mock_erc20.balance_of(env.alice)
        bob_initial = mock_erc20.balance_of(env.bob)
        
        transfer_amount = 100
        mock_erc20.transfer(env.alice, transfer_amount)
        
        alice_after_transfer = mock_erc20.balance_of(env.alice)
        print(f"✅ PASS: Transfer successful. Alice balance: {alice_after_transfer}")
        
        if alice_after_transfer != transfer_amount:
            raise Exception(f"Transfer failed. Expected {transfer_amount}, got {alice_after_transfer}")
        
        payment_amount = 50
        price_data = {"address": env.mock_addresses.erc20_a, "value": payment_amount}
        
        approval_tx = await env.alice_client.erc20.approve(price_data, "payment")
        print(f"✅ PASS: Tokens approved for payment. Tx hash: {approval_tx}")
        
        payment_allowance = mock_erc20.allowance(env.alice, env.addresses.erc20_addresses.payment_obligation)
        print(f"Payment allowance verified: {payment_allowance} tokens")
        
        if payment_allowance < payment_amount:
            raise Exception(f"Insufficient allowance. Expected >= {payment_amount}, got {payment_allowance}")
        
        
        payment_result = await env.alice_client.erc20.pay_with_erc20(price_data, env.bob)
        print(f"✅ PASS: Payment executed successfully")
        
        alice_final = mock_erc20.balance_of(env.alice)
        bob_final = mock_erc20.balance_of(env.bob)
        
        print(f"Alice final balance: {alice_final}")
        print(f"Bob final balance: {bob_final}")
        
        # Verify Alice's balance decreased by payment amount
        expected_alice_balance = alice_after_transfer - payment_amount
        if alice_final != expected_alice_balance:
            raise Exception(f"Alice balance incorrect. Expected {expected_alice_balance}, got {alice_final}")
        
        # Verify Bob's balance increased by payment amount
        expected_bob_balance = bob_initial + payment_amount
        if bob_final != expected_bob_balance:
            raise Exception(f"Bob balance incorrect. Expected {expected_bob_balance}, got {bob_final}")
        
        print("✅ PASS: All balances verified correctly")
        
        return True
        
    except Exception as e:
        print(f"\n❌ FAIL: Test failed - {e}")
        return False



async def main():
    print("🚀 ERC20 pay_with_erc20 Test Flow")
    print("=" * 80)
    
    success = await test_pay_with_erc20()
    
    print("\n" + "=" * 80)
    print("📊 TEST RESULTS")
    print("=" * 80)
    
    if success:
        print("🎉 SUCCESS! The pay_with_erc20 test flow completed successfully!")
        return True
    else:
        print("💥 Test failed. Please check the error messages above.")
        return False


if __name__ == "__main__":
    asyncio.run(main())
