import asyncio
from alkahest_py import PyTestEnvManager, PyMockERC20


async def test_pay_erc20_for_erc721():
    """
    Test paying ERC20 tokens to fulfill an ERC721 escrow.
    This corresponds to test_pay_erc20_for_erc721() in main.rs
    
    Flow: Bob escrows ERC721, Alice pays ERC20 to get the ERC721
    """
    try:
        env = PyTestEnvManager()
        
        # Setup mock tokens
        mock_erc20_a = PyMockERC20(env.mock_addresses.erc20_a, env.god_wallet_provider)
        # Note: PyMockERC721 is not available yet, but this shows the intended flow
        
        # Give Alice ERC20 tokens and Bob an ERC721 token
        alice_initial_erc20 = mock_erc20_a.balance_of(env.alice)
        mock_erc20_a.transfer(env.alice, 100)
        alice_after_transfer = mock_erc20_a.balance_of(env.alice)
        
        if alice_after_transfer != alice_initial_erc20 + 100:
            raise Exception(f"Alice ERC20 transfer failed. Expected {alice_initial_erc20 + 100}, got {alice_after_transfer}")
        
        # Bob would mint/own an ERC721 token (ID 1)
        # mock_erc721_a.mint(env.bob)  # When PyMockERC721 is available
        
        # Create test data
        erc20_amount = 50
        erc721_token_id = 1
        expiration = 3600  # 1 hour from now
        
        # Step 1: Bob approves his ERC721 for escrow  
        erc721_data = {"address": env.mock_addresses.erc721_a, "id": erc721_token_id}
        await env.bob_client.erc721.approve(erc721_data, "escrow")
        
        # Step 2: Bob creates ERC721 escrow requesting ERC20
        erc20_data = {"address": env.mock_addresses.erc20_a, "value": erc20_amount}
        buy_result = await env.bob_client.erc721.buy_erc20_with_erc721(
            erc721_data, erc20_data, expiration
        )
        
        if not buy_result['log']['uid'] or buy_result['log']['uid'] == "0x0000000000000000000000000000000000000000000000000000000000000000":
            raise Exception("Invalid buy attestation UID")
        
        buy_attestation_uid = buy_result['log']['uid']
        
        # Verify ERC721 is in escrow (would check with mock_erc721_a.owner_of when available)
        # erc721_owner = mock_erc721_a.owner_of(erc721_token_id)
        # assert erc721_owner == env.addresses.erc721_addresses.escrow_obligation
        
        initial_alice_erc20_balance = mock_erc20_a.balance_of(env.alice)
        
        # Step 3: Alice approves her ERC20 tokens for payment
        await env.alice_client.erc20.approve(erc20_data, "payment")
        
        # Step 4: Alice fulfills Bob's escrow
        pay_result = await env.alice_client.erc20.pay_erc20_for_erc721(buy_attestation_uid)
        
        if not pay_result['log']['uid'] or pay_result['log']['uid'] == "0x0000000000000000000000000000000000000000000000000000000000000000":
            raise Exception("Invalid payment attestation UID")
        
        # Verify token transfers
        # Alice should now own the ERC721 token (would check with mock_erc721_a.owner_of when available)
        # final_erc721_owner = mock_erc721_a.owner_of(erc721_token_id)
        # assert final_erc721_owner == env.alice
        
        # Alice spent erc20_amount tokens
        final_alice_erc20_balance = mock_erc20_a.balance_of(env.alice)
        alice_spent = initial_alice_erc20_balance - final_alice_erc20_balance
        if alice_spent != erc20_amount:
            raise Exception(f"Alice should have spent {erc20_amount} ERC20 tokens, spent {alice_spent}")
        
        # Bob received erc20_amount tokens
        bob_erc20_balance = mock_erc20_a.balance_of(env.bob)
        if bob_erc20_balance != erc20_amount:
            raise Exception(f"Bob should have received {erc20_amount} ERC20 tokens, got {bob_erc20_balance}")
        
        print("✅ test_pay_erc20_for_erc721 PASSED")
        
    except Exception as e:
        print(f"❌ test_pay_erc20_for_erc721 FAILED: {e}")
        raise


if __name__ == "__main__":
    asyncio.run(test_pay_erc20_for_erc721())
