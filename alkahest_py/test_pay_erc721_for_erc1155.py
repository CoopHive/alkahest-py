import asyncio
from alkahest_py import PyTestEnvManager, PyMockERC721, PyMockERC1155


async def test_pay_erc721_for_erc1155():
    """
    Test paying ERC721 for ERC1155 tokens.
    This corresponds to test_pay_erc721_for_erc1155() in main.rs
    
    Flow: Bob escrows ERC1155, Alice pays ERC721 to get the ERC1155
    """
    try:
        env = PyTestEnvManager()
        
        # Setup mock tokens
        mock_erc721_a = PyMockERC721(env.mock_addresses.erc721_a, env.god_wallet_provider)
        mock_erc1155_a = PyMockERC1155(env.mock_addresses.erc1155_a, env.god_wallet_provider)
        
        # Mint an ERC721 token to Alice (she will fulfill with this)
        token_id = mock_erc721_a.mint(env.alice)
        print(f"Minted ERC721 token {token_id} to Alice")
        
        # Give Bob some ERC1155 tokens for escrow
        mock_erc1155_a.mint(env.bob, 1, 10)  # Mint 10 tokens of ID 1 to Bob
        bob_erc1155_balance = mock_erc1155_a.balance_of(env.bob, 1)
        print(f"Bob has {bob_erc1155_balance} ERC1155 tokens of ID 1")
        
        if bob_erc1155_balance != 10:
            raise Exception(f"Bob should have 10 ERC1155 tokens, got {bob_erc1155_balance}")
        
        # Create test data
        bid_data = {  # Bob's bid
            "address": env.mock_addresses.erc1155_a,
            "id": 1,
            "value": 10
        }
        ask_data = {  # Bob asks for Alice's ERC721
            "address": env.mock_addresses.erc721_a,
            "id": token_id
        }
        
        # Bob approves tokens for escrow and creates buy attestation
        await env.bob_client.erc1155.approve_all(env.mock_addresses.erc1155_a, "escrow")
        
        buy_result = await env.bob_client.erc1155.buy_erc721_with_erc1155(bid_data, ask_data, 0)
        
        if not buy_result['log']['uid'] or buy_result['log']['uid'] == "0x0000000000000000000000000000000000000000000000000000000000000000":
            raise Exception("Invalid buy attestation UID")
        
        buy_attestation_uid = buy_result['log']['uid']
        
        # Alice approves her token for payment
        erc721_data = {"address": env.mock_addresses.erc721_a, "id": token_id}
        await env.alice_client.erc721.approve(erc721_data, "payment")
        
        # Alice fulfills the buy attestation
        pay_result = await env.alice_client.erc721.pay_erc721_for_erc1155(buy_attestation_uid)
        
        if not pay_result['log']['uid'] or pay_result['log']['uid'] == "0x0000000000000000000000000000000000000000000000000000000000000000":
            raise Exception("Invalid payment attestation UID")
        
        # Verify token transfers
        # Alice should have received ERC1155 tokens
        alice_erc1155_balance = mock_erc1155_a.balance_of(env.alice, 1)
        if alice_erc1155_balance != 10:
            raise Exception(f"Alice should have received 10 ERC1155 tokens, got {alice_erc1155_balance}")
        
        # Bob should have received the ERC721 token
        token_owner = mock_erc721_a.owner_of(token_id)
        if token_owner.lower() != env.bob.lower():
            raise Exception(f"Bob should have received the ERC721 token, but it's owned by {token_owner}")
        
        print("✅ test_pay_erc721_for_erc1155 PASSED")
        return True
        
    except Exception as e:
        print(f"❌ test_pay_erc721_for_erc1155 FAILED: {e}")
        raise


if __name__ == "__main__":
    asyncio.run(test_pay_erc721_for_erc1155())
