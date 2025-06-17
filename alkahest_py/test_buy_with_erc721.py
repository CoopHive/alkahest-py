import asyncio
from alkahest_py import PyTestEnvManager, PyMockERC721


async def test_buy_with_erc721():
    """
    Test buying with ERC721 tokens.
    This corresponds to test_buy_with_erc721() in main.rs
    
    Flow: Alice uses ERC721 to create a buy order with custom arbiter data
    """
    try:
        env = PyTestEnvManager()
        
        # Setup mock ERC721 token
        mock_erc721_a = PyMockERC721(env.mock_addresses.erc721_a, env.god_wallet_provider)
        
        # Mint an ERC721 token to Alice
        token_id = mock_erc721_a.mint(env.alice)
        print(f"Minted ERC721 token {token_id} to Alice")
        
        # Verify Alice owns the token
        token_owner = mock_erc721_a.owner_of(token_id)
        if token_owner.lower() != env.alice.lower():
            raise Exception(f"Token ownership verification failed. Expected {env.alice}, got {token_owner}")
        
        # Create test data
        price_data = {
            "address": env.mock_addresses.erc721_a,
            "id": token_id
        }
        
        # Create custom arbiter data
        arbiter_data = {
            "arbiter": env.addresses.erc721_addresses.payment_obligation,
            "demand": b"custom demand data"
        }
        
        # Alice approves token for escrow
        await env.alice_client.erc721.approve(price_data, "escrow")
        
        # Alice creates buy order with ERC721
        buy_result = await env.alice_client.erc721.buy_with_erc721(price_data, arbiter_data, 0)
        
        if not buy_result['log']['uid'] or buy_result['log']['uid'] == "0x0000000000000000000000000000000000000000000000000000000000000000":
            raise Exception("Invalid buy attestation UID")
        
        buy_attestation_uid = buy_result['log']['uid']
        
        # Verify ERC721 is in escrow
        current_owner = mock_erc721_a.owner_of(token_id)
        escrow_address = env.addresses.erc721_addresses.escrow_obligation
        print(f"ERC721 token {token_id} now owned by: {current_owner}")
        print(f"Expected escrow address: {escrow_address}")
        
        # The token should now be owned by the escrow contract
        if current_owner.lower() != escrow_address.lower():
            raise Exception(f"Token should be in escrow at {escrow_address}, but owned by {current_owner}")
        
        # Verify the attestation was created (buy order is live)
        if not buy_attestation_uid:
            raise Exception("Buy attestation UID should be valid")
        
        print("✅ test_buy_with_erc721 PASSED")
        return True
        
    except Exception as e:
        print(f"❌ test_buy_with_erc721 FAILED: {e}")
        raise


if __name__ == "__main__":
    asyncio.run(test_buy_with_erc721())
