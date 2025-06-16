import asyncio
import time
from alkahest_py import PyTestEnvManager, PyMockERC20


async def test_permit_and_buy_with_erc20():
    try:
        env = PyTestEnvManager()
        mock_erc20 = PyMockERC20(env.mock_addresses.erc20_a, env.god_wallet_provider)
        
        god_initial = mock_erc20.balance_of(env.god)
        alice_initial = mock_erc20.balance_of(env.alice)
        
        transfer_amount = 100
        mock_erc20.transfer(env.alice, transfer_amount)
        
        alice_after_transfer = mock_erc20.balance_of(env.alice)
        expected_alice_balance = alice_initial + transfer_amount
        
        if alice_after_transfer != expected_alice_balance:
            print(f"Transfer failed: Expected {expected_alice_balance}, got {alice_after_transfer}")
            return False
        
        price_data = {"address": env.mock_addresses.erc20_a, "value": 50}
        arbiter_data = {
            "arbiter": env.addresses.erc20_addresses.payment_obligation,
            "demand": b"test"
        }
        expiration = int(time.time()) + 3600
        
        result = await env.alice_client.erc20.permit_and_buy_with_erc20(
            price_data, arbiter_data, expiration
        )
        
        alice_final = mock_erc20.balance_of(env.alice)
        print("✅ test_permit_and_buy_with_erc20 PASSED")
        return True
        
    except Exception as e:
        print(f"Test failed: {e}")
        return False


async def main():
    success = await test_permit_and_buy_with_erc20()
    return 0 if success else 1


if __name__ == "__main__":
    exit_code = asyncio.run(main())
    exit(exit_code)
