import asyncio
from alkahest_py import PyTestEnvManager, PyMockERC20


async def test_pay_erc20_for_erc1155():
    """
    Test paying ERC20 tokens to fulfill an ERC1155 escrow.
    This corresponds to test_pay_erc20_for_erc1155() in main.rs
    
    Flow: Bob escrows ERC1155, Alice pays ERC20 to get the ERC1155
    """
    try:
        env = PyTestEnvManager()
        
        # Setup mock tokens
        mock_erc20_a = PyMockERC20(env.mock_addresses.erc20_a, env.god_wallet_provider)
        # Note: PyMockERC1155 is not available yet, but this shows the intended flow
        
        # Give Alice ERC20 tokens and Bob ERC1155 tokens
        alice_initial_erc20 = mock_erc20_a.balance_of(env.alice)
        mock_erc20_a.transfer(env.alice, 100)
        alice_after_transfer = mock_erc20_a.balance_of(env.alice)
        
        if alice_after_transfer != alice_initial_erc20 + 100:
            raise Exception(f"Alice ERC20 transfer failed. Expected {alice_initial_erc20 + 100}, got {alice_after_transfer}")
        
        # Bob would mint/own ERC1155 tokens (ID 1, amount 50)
        # mock_erc1155_a.mint(env.bob, token_id=1, amount=50)  # When PyMockERC1155 is available
        
        # Create test data
        erc20_amount = 50
        token_id = 1
        token_amount = 50  # Amount of ERC1155 tokens Bob has
        expiration = 3600  # 1 hour from now
        
        # Step 1: Bob approves his ERC1155 for escrow
        await env.bob_client.erc1155.approve_all(env.mock_addresses.erc1155_a, "escrow")
        
        # Step 2: Bob creates ERC1155 escrow requesting ERC20
        erc1155_data = {"address": env.mock_addresses.erc1155_a, "id": token_id, "value": token_amount}
        erc20_data = {"address": env.mock_addresses.erc20_a, "value": erc20_amount}
        buy_result = await env.bob_client.erc1155.buy_erc20_with_erc1155(
            erc1155_data, erc20_data, expiration
        )
        
        if not buy_result['log']['uid'] or buy_result['log']['uid'] == "0x0000000000000000000000000000000000000000000000000000000000000000":
            raise Exception("Invalid buy attestation UID")
        
        buy_attestation_uid = buy_result['log']['uid']
        
        # Check initial balances before the exchange
        # Alice should start with 0 ERC1155 tokens (would check with mock_erc1155_a.balance_of when available)
        # initial_alice_erc1155_balance = mock_erc1155_a.balance_of(env.alice, token_id)
        # assert initial_alice_erc1155_balance == 0
        
        initial_alice_erc20_balance = mock_erc20_a.balance_of(env.alice)
        
        # Step 3: Alice approves her ERC20 tokens for payment
        await env.alice_client.erc20.approve(erc20_data, "payment")
        
        # Step 4: Alice fulfills Bob's escrow
        pay_result = await env.alice_client.erc20.pay_erc20_for_erc1155(buy_attestation_uid)
        
        if not pay_result['log']['uid'] or pay_result['log']['uid'] == "0x0000000000000000000000000000000000000000000000000000000000000000":
            raise Exception("Invalid payment attestation UID")
        
        # Verify token transfers
        # Alice should have received the ERC1155 tokens (would check with mock_erc1155_a.balance_of when available)
        # final_alice_erc1155_balance = mock_erc1155_a.balance_of(env.alice, token_id)
        # assert final_alice_erc1155_balance == token_amount
        
        # Alice spent erc20_amount tokens
        final_alice_erc20_balance = mock_erc20_a.balance_of(env.alice)
        alice_spent = initial_alice_erc20_balance - final_alice_erc20_balance
        if alice_spent != erc20_amount:
            raise Exception(f"Alice should have spent {erc20_amount} ERC20 tokens, spent {alice_spent}")
        
        # Bob received erc20_amount tokens
        bob_erc20_balance = mock_erc20_a.balance_of(env.bob)
        if bob_erc20_balance != erc20_amount:
            raise Exception(f"Bob should have received {erc20_amount} ERC20 tokens, got {bob_erc20_balance}")
        
        print("✅ test_pay_erc20_for_erc1155 PASSED")
        
    except Exception as e:
        print(f"❌ test_pay_erc20_for_erc1155 FAILED: {e}")
        raise


if __name__ == "__main__":
    asyncio.run(test_pay_erc20_for_erc1155())
