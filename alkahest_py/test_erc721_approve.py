import asyncio
from alkahest_py import PyTestEnvManager, PyMockERC721


async def test_erc721_approve():
    """
    Test ERC721 token approval for both payment and escrow purposes.
    This corresponds to test_approve() in main.rs (lines 81-135)
    
    Flow:
    1. Mint an ERC721 token to Alice
    2. Test approve for payment purpose and verify approval
    3. Test approve for escrow purpose and verify approval
    """
    try:
        env = PyTestEnvManager()
        
        # Setup mock ERC721 token
        mock_erc721_a = PyMockERC721(env.mock_addresses.erc721_a, env.god_wallet_provider)
        
        # Mint an ERC721 token to Alice (token ID 1)
        mock_erc721_a.mint(env.alice)
        
        # Create token data for the minted token
        token_data = {"address": env.mock_addresses.erc721_a, "id": 1}
        
        # Test approve for payment
        await env.alice_client.erc721.approve(token_data, "payment")
        
        # Verify approval for payment obligation
        payment_approved = mock_erc721_a.get_approved(1)
        expected_payment_approval = env.addresses.erc721_addresses.payment_obligation
        
        if payment_approved.lower() != expected_payment_approval.lower():
            raise Exception(f"Payment approval should be set to {expected_payment_approval}, got {payment_approved}")
        
        print(f"✓ Payment approval verified: token 1 approved for {payment_approved}")
        
        # Test approve for escrow
        await env.alice_client.erc721.approve(token_data, "escrow")
        
        # Verify approval for escrow obligation
        escrow_approved = mock_erc721_a.get_approved(1)
        expected_escrow_approval = env.addresses.erc721_addresses.escrow_obligation
        
        if escrow_approved.lower() != expected_escrow_approval.lower():
            raise Exception(f"Escrow approval should be set to {expected_escrow_approval}, got {escrow_approved}")
        
        print(f"✓ Escrow approval verified: token 1 approved for {escrow_approved}")
        
        print("✅ test_erc721_approve PASSED")
        return True
        
    except Exception as e:
        print(f"❌ test_erc721_approve FAILED: {e}")
        raise


async def main():
    try:
        success = await test_erc721_approve()
        return success
    except Exception as e:
        print(f"Unexpected error: {e}")
        return False


if __name__ == "__main__":
    exit_code = asyncio.run(main())
    exit(0 if exit_code else 1)
