import asyncio
import time
from alkahest_py import PyTestEnvManager, PyMockERC721


async def test_collect_expired():
    """
    Test collecting expired escrowed tokens.
    This corresponds to test_collect_expired() in main.rs
    
    Flow: Alice creates escrow with short expiration, waits for expiration, then collects
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
        
        # Create bid and ask data
        bid_data = {
            "address": env.mock_addresses.erc721_a,
            "id": token_id
        }
        ask_data = {
            "address": env.mock_addresses.erc721_b,
            "id": 2  # Ask for ERC721 B token ID 2
        }
        
        # Alice approves token for escrow
        await env.alice_client.erc721.approve(bid_data, "escrow")
        
        # Alice makes escrow with a short expiration (current time + 10 seconds)
        expiration = int(time.time()) + 10
        buy_result = await env.alice_client.erc721.buy_erc_721_for_erc_721(bid_data, ask_data, expiration)
        
        if not buy_result['log']['uid'] or buy_result['log']['uid'] == "0x0000000000000000000000000000000000000000000000000000000000000000":
            raise Exception("Invalid buy attestation UID")
        
        buy_attestation_uid = buy_result['log']['uid']
        
        # Verify token is in escrow
        current_owner = mock_erc721_a.owner_of(token_id)
        escrow_address = env.addresses.erc721_addresses.escrow_obligation
        print(f"ERC721 token {token_id} in escrow at: {current_owner}")
        
        if current_owner.lower() != escrow_address.lower():
            raise Exception(f"Token should be in escrow at {escrow_address}, but owned by {current_owner}")
        
        # Wait for expiration (adding buffer time)
        print("Waiting for escrow to expire...")
        time.sleep(12)
        
        # Alice collects expired funds
        collect_result = await env.alice_client.erc721.collect_expired(buy_attestation_uid)
        print(f"Collected expired escrow, transaction: {collect_result}")
        
        # Verify token returned to Alice
        final_owner = mock_erc721_a.owner_of(token_id)
        print(f"ERC721 token {token_id} finally owned by: {final_owner}")
        
        if final_owner.lower() != env.alice.lower():
            raise Exception(f"Token should be returned to Alice, but it's owned by {final_owner}")
        
        print("✅ test_collect_expired PASSED")
        return True
        
    except Exception as e:
        print(f"❌ test_collect_expired FAILED: {e}")
        raise


if __name__ == "__main__":
    asyncio.run(test_collect_expired())
