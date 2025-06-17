import asyncio
from alkahest_py import PyTestEnvManager, PyMockERC1155


async def test_erc1155_approve_all():
    """
    Test ERC1155 approve_all functionality for both payment and escrow purposes.
    This corresponds to test_erc1155_approve_all() in main.rs
    
    Flow: 
    1. Mint ERC1155 tokens to Alice
    2. Test approve_all for payment purpose and verify with isApprovedForAll
    3. Test approve_all for escrow purpose and verify with isApprovedForAll
    """
    try:
        env = PyTestEnvManager()
        
        # Setup mock ERC1155 token
        mock_erc1155_a = PyMockERC1155(env.mock_addresses.erc1155_a, env.god_wallet_provider)
        
        # Mint ERC1155 tokens to Alice
        mock_erc1155_a.mint(env.alice, 1, 10)
        print(f"Minted 10 ERC1155 tokens (ID: 1) to Alice")
        
        # Verify Alice owns the tokens
        alice_balance = mock_erc1155_a.balance_of(env.alice, 1)
        if alice_balance != 10:
            raise Exception(f"Token ownership verification failed. Expected 10, got {alice_balance}")
        
        # Test approve_all for payment
        print("Testing approve_all for payment purpose...")
        await env.alice_client.erc1155.approve_all(env.mock_addresses.erc1155_a, "payment")
        
        # Verify approval for payment obligation using isApprovedForAll
        payment_approved = mock_erc1155_a.is_approved_for_all(
            env.alice, 
            env.addresses.erc1155_addresses.payment_obligation
        )
        
        if not payment_approved:
            raise Exception("Payment approval for all should be set correctly")
        
        print("✅ Payment approve_all verified successfully")
        
        # Test approve_all for escrow
        print("Testing approve_all for escrow purpose...")
        await env.alice_client.erc1155.approve_all(env.mock_addresses.erc1155_a, "escrow")
        
        # Verify approval for escrow obligation using isApprovedForAll
        escrow_approved = mock_erc1155_a.is_approved_for_all(
            env.alice, 
            env.addresses.erc1155_addresses.escrow_obligation
        )
        
        if not escrow_approved:
            raise Exception("Escrow approval for all should be set correctly")
        
        print("✅ Escrow approve_all verified successfully")
        
        print("✅ test_erc1155_approve_all PASSED")
        return True
        
    except Exception as e:
        print(f"❌ test_erc1155_approve_all FAILED: {e}")
        raise


async def main():
    try:
        success = await test_erc1155_approve_all()
        return 0 if success else 1
    except Exception as e:
        print(f"Test execution failed: {e}")
        return 1


if __name__ == "__main__":
    exit_code = asyncio.run(main())
    exit(exit_code)
