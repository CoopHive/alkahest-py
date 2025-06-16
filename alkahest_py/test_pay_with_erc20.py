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
        if alice_after_transfer != transfer_amount:
            raise Exception(f"Transfer failed. Expected {transfer_amount}, got {alice_after_transfer}")
        
        payment_amount = 50
        price_data = {"address": env.mock_addresses.erc20_a, "value": payment_amount}
        
        await env.alice_client.erc20.approve(price_data, "payment")
        
        payment_allowance = mock_erc20.allowance(env.alice, env.addresses.erc20_addresses.payment_obligation)
        if payment_allowance < payment_amount:
            raise Exception(f"Insufficient allowance. Expected >= {payment_amount}, got {payment_allowance}")
        
        payment_result = await env.alice_client.erc20.pay_with_erc20(price_data, env.bob)
        
        alice_final = mock_erc20.balance_of(env.alice)
        bob_final = mock_erc20.balance_of(env.bob)
        
        expected_alice_balance = alice_after_transfer - payment_amount
        if alice_final != expected_alice_balance:
            raise Exception(f"Alice balance incorrect. Expected {expected_alice_balance}, got {alice_final}")
        
        expected_bob_balance = bob_initial + payment_amount
        if bob_final != expected_bob_balance:
            raise Exception(f"Bob balance incorrect. Expected {expected_bob_balance}, got {bob_final}")
        
        if not payment_result['log']['uid'] or payment_result['log']['uid'] == "0x0000000000000000000000000000000000000000000000000000000000000000":
            raise Exception("Invalid attestation UID")
        
        return True
        
    except Exception as e:
        print(f"Test failed: {e}")
        return False


async def main():
    success = await test_pay_with_erc20()
    return 0 if success else 1


if __name__ == "__main__":
    exit_code = asyncio.run(main())
    exit(exit_code)
