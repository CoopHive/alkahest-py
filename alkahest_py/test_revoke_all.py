import asyncio
from alkahest_py import PyTestEnvManager, PyMockERC721


async def test_revoke_all():
    """
    Test ERC721 revoke_all functionality for payment purpose.
    This corresponds to test_revoke_all() in main.rs
    
    Flow: 
    1. Mint ERC721 token to Alice
    2. First approve_all for payment purpose
    3. Then revoke_all for payment purpose
    4. Verify that approval has been revoked using isApprovedForAll
    """
    try:
        env = PyTestEnvManager()
        
        # Setup mock ERC721 token
        mock_erc721_a = PyMockERC721(env.mock_addresses.erc721_a, env.god_wallet_provider)
        
        # Mint ERC721 token to Alice
        token_id = mock_erc721_a.mint(env.alice)
        print(f"Minted ERC721 token {token_id} to Alice")
        
        # Verify Alice owns the token
        owner = mock_erc721_a.owner_of(token_id)
        if owner.lower() != env.alice.lower():
            raise Exception(f"Token ownership verification failed. Expected {env.alice}, got {owner}")
        
        # First approve_all for payment
        print("Setting approve_all for payment purpose...")
        await env.alice_client.erc721.approve_all(env.mock_addresses.erc721_a, "payment")
        
        # Verify approval was set
        payment_approved_before = mock_erc721_a.is_approved_for_all(
            env.alice, 
            env.addresses.erc721_addresses.payment_obligation
        )
        
        if not payment_approved_before:
            raise Exception("Payment approval should be set before revocation")
        
        print("✅ Payment approve_all verified as set")
        
        # Then revoke_all for payment
        print("Revoking all approvals for payment purpose...")
        await env.alice_client.erc721.revoke_all(env.mock_addresses.erc721_a, "payment")
        
        # Verify revocation
        payment_approved_after = mock_erc721_a.is_approved_for_all(
            env.alice, 
            env.addresses.erc721_addresses.payment_obligation
        )
        
        if payment_approved_after:
            raise Exception("Payment approval should be revoked")
        
        print("✅ Payment approval successfully revoked")
        
        print("✅ test_revoke_all PASSED")
        return True
        
    except Exception as e:
        print(f"❌ test_revoke_all FAILED: {e}")
        raise


async def main():
    try:
        success = await test_revoke_all()
        return 0 if success else 1
    except Exception as e:
        print(f"Test execution failed: {e}")
        return 1


if __name__ == "__main__":
    exit_code = asyncio.run(main())
    exit(exit_code)
