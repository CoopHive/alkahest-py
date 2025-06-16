import asyncio
from alkahest_py import PyTestEnvManager, PyMockERC20


async def test_buy_erc20_for_erc20():
    try:
        env = PyTestEnvManager()
        mock_erc20_a = PyMockERC20(env.mock_addresses.erc20_a, env.god_wallet_provider)
        
        transfer_amount = 100
        mock_erc20_a.transfer(env.alice, transfer_amount)
        
        alice_after_transfer = mock_erc20_a.balance_of(env.alice)
        if alice_after_transfer != transfer_amount:
            raise Exception(f"Transfer failed. Expected {transfer_amount}, got {alice_after_transfer}")
        
        bid_amount = 100
        bid_data = {"address": env.mock_addresses.erc20_a, "value": bid_amount}
        
        await env.alice_client.erc20.approve(bid_data, "escrow")
        
        escrow_allowance = mock_erc20_a.allowance(env.alice, env.addresses.erc20_addresses.escrow_obligation)
        if escrow_allowance < bid_amount:
            raise Exception(f"Insufficient allowance. Expected >= {bid_amount}, got {escrow_allowance}")
        
        ask_amount = 200
        ask_data = {"address": env.mock_addresses.erc20_b, "value": ask_amount}
        expiration = 0
        
        escrow_result = await env.alice_client.erc20.buy_erc20_for_erc20(bid_data, ask_data, expiration)
        
        alice_final_a = mock_erc20_a.balance_of(env.alice)
        escrow_balance_a = mock_erc20_a.balance_of(env.addresses.erc20_addresses.escrow_obligation)
        
        expected_alice_balance = alice_after_transfer - bid_amount
        if alice_final_a != expected_alice_balance:
            raise Exception(f"Alice balance incorrect. Expected {expected_alice_balance}, got {alice_final_a}")
        
        if escrow_balance_a != bid_amount:
            raise Exception(f"Escrow balance incorrect. Expected {bid_amount}, got {escrow_balance_a}")
        
        if not escrow_result['log']['uid'] or escrow_result['log']['uid'] == "0x0000000000000000000000000000000000000000000000000000000000000000":
            raise Exception("Invalid attestation UID")
        
        print("✅ test_buy_erc20_for_erc20 PASSED")
        return True
        
    except Exception as e:
        print(f"Test failed: {e}")
        return False


async def main():
    success = await test_buy_erc20_for_erc20()
    return 0 if success else 1


if __name__ == "__main__":
    exit_code = asyncio.run(main())
    exit(exit_code)
