import asyncio
from alkahest_py import PyTestEnvManager, PyMockERC20


async def test_buy_with_erc20():
    try:
        env = PyTestEnvManager()
        mock_erc20 = PyMockERC20(env.mock_addresses.erc20_a, env.god_wallet_provider)
        alice_initial = mock_erc20.balance_of(env.alice)

        transfer_amount = 100
        mock_erc20.transfer(env.alice, transfer_amount)

        alice_after_transfer = mock_erc20.balance_of(env.alice)
        expected_alice_balance = alice_initial + transfer_amount

        if alice_after_transfer != expected_alice_balance:
            raise Exception(f"Transfer failed. Expected {expected_alice_balance}, got {alice_after_transfer}")
        
        price_data = {
            "address": env.mock_addresses.erc20_a, 
            "value": 100
        }
        
        await env.alice_client.erc20.approve(price_data, "escrow")
        
        arbiter_data = {
            "arbiter": env.addresses.erc20_addresses.payment_obligation,
            "demand": b"custom demand data"
        }
        expiration = 0
        
        result = await env.alice_client.erc20.buy_with_erc20(
            price_data, arbiter_data, expiration
        )
        
        alice_final_balance = mock_erc20.balance_of(env.alice)
        escrow_balance = mock_erc20.balance_of(env.addresses.erc20_addresses.escrow_obligation)
        
        if alice_final_balance == 0 and escrow_balance == 100:
            print("✅ test_buy_with_erc20 PASSED")
            return True
        else:
            print(f"Token balances incorrect. Alice: {alice_final_balance}, Escrow: {escrow_balance}")
            return False
        
    except Exception as e:
        print(f"Test failed: {e}")
        return False


async def main():
    success = await test_buy_with_erc20()
    return 0 if success else 1


if __name__ == "__main__":
    exit_code = asyncio.run(main())
    exit(exit_code)
