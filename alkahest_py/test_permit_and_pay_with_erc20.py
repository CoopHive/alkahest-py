import asyncio
from alkahest_py import PyTestEnvManager, PyMockERC20


async def test_permit_and_pay_with_erc20():
   
    
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
        
        payment_allowance = mock_erc20.allowance(env.alice, env.addresses.erc20_addresses.payment_obligation)
        
        if payment_allowance != 0:
            print(f"⚠️  WARNING: Unexpected existing allowance of {payment_allowance} tokens")
        else:
            print("✅ PASS: No pre-approval exists, as expected for permit flow")
        
        payment_amount = 100
        price_data = {"address": env.mock_addresses.erc20_a, "value": payment_amount}
        
        payment_result = await env.alice_client.erc20.permit_and_pay_with_erc20(price_data, env.bob)
        print(f"✅ PASS: Permit and pay executed successfully")
        alice_final = mock_erc20.balance_of(env.alice)
        bob_final = mock_erc20.balance_of(env.bob)
        
        expected_alice_balance = alice_after_transfer - payment_amount
        if alice_final != expected_alice_balance:
            raise Exception(f"Alice balance incorrect. Expected {expected_alice_balance}, got {alice_final}")
        
        expected_bob_balance = bob_initial + payment_amount
        if bob_final != expected_bob_balance:
            raise Exception(f"Bob balance incorrect. Expected {expected_bob_balance}, got {bob_final}")
        
        print("✅ PASS: All balances verified correctly")
        
        return True
        
    except Exception as e:
        print(f"\n❌ FAIL: Test failed - {e}")
        return False


async def main():
    success = await test_permit_and_pay_with_erc20()
    
    # Final results
    print("\n" + "=" * 80)
    print("📊 TEST RESULTS")
    print("=" * 80)
    
    if success:
        print("🎉 SUCCESS! The permit_and_pay_with_erc20 test flow completed successfully!")
        return True
    else:
        print("💥 Test failed. Please check the error messages above.")
        return False


if __name__ == "__main__":
    asyncio.run(main())
