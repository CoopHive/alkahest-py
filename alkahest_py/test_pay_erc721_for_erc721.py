import asyncio
from alkahest_py import PyTestEnvManager, PyMockERC721


async def test_pay_erc721_for_erc721():
    """
    Test paying ERC721 for ERC721 tokens.
    This corresponds to test_pay_erc721_for_erc721() in main.rs
    
    Flow: Alice creates ERC721 escrow, Bob fulfills with his ERC721
    """
    try:
        env = PyTestEnvManager()
        
        # Setup mock ERC721 tokens
        mock_erc721_a = PyMockERC721(env.mock_addresses.erc721_a, env.god_wallet_provider)
        mock_erc721_b = PyMockERC721(env.mock_addresses.erc721_b, env.god_wallet_provider)
        
        # Mint ERC721 tokens to Alice and Bob
        token_id_a = mock_erc721_a.mint(env.alice)
        token_id_b = mock_erc721_b.mint(env.bob)
        print(f"Minted ERC721 token {token_id_a} to Alice")
        print(f"Minted ERC721 token {token_id_b} to Bob")
        
        # Verify ownership
        alice_token_owner = mock_erc721_a.owner_of(token_id_a)
        bob_token_owner = mock_erc721_b.owner_of(token_id_b)
        if alice_token_owner.lower() != env.alice.lower():
            raise Exception(f"Alice token ownership verification failed. Expected {env.alice}, got {alice_token_owner}")
        if bob_token_owner.lower() != env.bob.lower():
            raise Exception(f"Bob token ownership verification failed. Expected {env.bob}, got {bob_token_owner}")
        
        # Create bid and ask data
        bid_data = {
            "address": env.mock_addresses.erc721_a,
            "id": token_id_a
        }
        ask_data = {
            "address": env.mock_addresses.erc721_b,
            "id": token_id_b
        }
        
        # Alice approves token for escrow and creates buy attestation
        await env.alice_client.erc721.approve(bid_data, "escrow")
        
        buy_result = await env.alice_client.erc721.buy_erc_721_for_erc_721(bid_data, ask_data, 0)
        
        if not buy_result['log']['uid'] or buy_result['log']['uid'] == "0x0000000000000000000000000000000000000000000000000000000000000000":
            raise Exception("Invalid buy attestation UID")
        
        buy_attestation_uid = buy_result['log']['uid']
        
        # Bob approves token for payment
        await env.bob_client.erc721.approve(ask_data, "payment")
        
        # Bob fulfills the buy attestation
        pay_result = await env.bob_client.erc721.pay_erc_721_for_erc_721(buy_attestation_uid)
        
        if not pay_result['log']['uid'] or pay_result['log']['uid'] == "0x0000000000000000000000000000000000000000000000000000000000000000":
            raise Exception("Invalid payment attestation UID")
        
        # Verify token transfers
        alice_final_owner = mock_erc721_b.owner_of(token_id_b)
        bob_final_owner = mock_erc721_a.owner_of(token_id_a)
        
        print(f"Alice now owns ERC721 B token {token_id_b}: {alice_final_owner}")
        print(f"Bob now owns ERC721 A token {token_id_a}: {bob_final_owner}")
        
        # Both sides should have received the tokens
        if alice_final_owner.lower() != env.alice.lower():
            raise Exception(f"Alice should have received ERC721 B token {token_id_b}, but it's owned by {alice_final_owner}")
        if bob_final_owner.lower() != env.bob.lower():
            raise Exception(f"Bob should have received ERC721 A token {token_id_a}, but it's owned by {bob_final_owner}")
        
        print("✅ test_pay_erc721_for_erc721 PASSED")
        return True
        
    except Exception as e:
        print(f"❌ test_pay_erc721_for_erc721 FAILED: {e}")
        raise


if __name__ == "__main__":
    asyncio.run(test_pay_erc721_for_erc721())
