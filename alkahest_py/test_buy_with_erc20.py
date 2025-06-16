import asyncio
import time
from alkahest_py import PyTestEnvManager, PyMockERC20


async def test_buy_with_erc20():
    
    try:
        env = PyTestEnvManager()
        mock_erc20 = PyMockERC20(env.mock_addresses.erc20_a, env.god_wallet_provider)
        alice_initial = mock_erc20.balance_of(env.alice)

        # Step 2: Transfer tokens to Alice
        transfer_amount = 100
        mock_erc20.transfer(env.alice, transfer_amount)

        alice_after_transfer = mock_erc20.balance_of(env.alice)
        expected_alice_balance = alice_initial + transfer_amount

        if alice_after_transfer == expected_alice_balance:
            print(f"✅ PASS: Transfer successful. Alice balance: {alice_after_transfer}")
        else:
            print(f"❌ FAIL: Expected {expected_alice_balance}, got {alice_after_transfer}")
            return False
        
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
        
        alice_final_balance = mock_erc20.balance_of(env.alice)
        escrow_balance = mock_erc20.balance_of(env.addresses.erc20_addresses.escrow_obligation)
        
        if alice_final_balance == 0 and escrow_balance == 100:
            print("✅ PASS: All tokens successfully moved to escrow")
        else:
            print(f"❌ FAIL: Token balances incorrect. Alice: {alice_final_balance}, Escrow: {escrow_balance}")
            return False
        
        print("\n" + "=" * 60)
        print("🎉 buy_with_erc20 test flow completed successfully!")
        
        return True
        
    except Exception as e:
        print(f"\n💥 UNEXPECTED ERROR: {e}")
        return False


async def main():
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
