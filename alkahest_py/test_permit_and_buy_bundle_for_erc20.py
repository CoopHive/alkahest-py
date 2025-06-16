import asyncio
from alkahest_py import PyTestEnvManager, PyMockERC20


async def test_permit_and_buy_bundle_for_erc20():
    """
    Test buying a token bundle with ERC20 tokens using permit (no pre-approval needed).
    This corresponds to test_permit_and_buy_bundle_for_erc20() in main.rs
    """
    try:
        env = PyTestEnvManager()
        
        # Setup mock ERC20 token  
        mock_erc20 = PyMockERC20(env.mock_addresses.erc20_a, env.god_wallet_provider)
        
        # Give Alice some ERC20 tokens (for bidding)
        alice_initial_erc20 = mock_erc20.balance_of(env.alice)
        mock_erc20.transfer(env.alice, 100)
        alice_after_transfer = mock_erc20.balance_of(env.alice)
        
        if alice_after_transfer != alice_initial_erc20 + 100:
            raise Exception(f"Alice ERC20 transfer failed. Expected {alice_initial_erc20 + 100}, got {alice_after_transfer}")
        
        # Alice creates buy order: offers 50 ERC20 tokens for a token bundle
        # Using permit, so no pre-approval needed
        bid_data = {"address": env.mock_addresses.erc20_a, "value": 50}  # Alice offers ERC20
        
        # Create bundle data (ERC20 + ERC721 + ERC1155)
        bundle_data = {
            "erc20s": [{"address": env.mock_addresses.erc20_b, "value": 20}],
            "erc721s": [{"address": env.mock_addresses.erc721_a, "id": 1}],
            "erc1155s": [{"address": env.mock_addresses.erc1155_a, "id": 1, "value": 5}]
        }
        
        # Alice creates the buy order for token bundle with permit (no pre-approval needed)
        buy_result = await env.alice_client.erc20.permit_and_buy_bundle_for_erc20(bid_data, bundle_data, 0)
        
        if not buy_result['log']['uid'] or buy_result['log']['uid'] == "0x0000000000000000000000000000000000000000000000000000000000000000":
            raise Exception("Invalid buy attestation UID")
        
        buy_attestation_uid = buy_result['log']['uid']
        
        # Verify Alice's ERC20 tokens are in escrow
        alice_balance_after_escrow = mock_erc20.balance_of(env.alice)
        escrow_balance = mock_erc20.balance_of(env.addresses.erc20_addresses.escrow_obligation)
        
        expected_alice_balance = alice_initial_erc20 + 100 - 50  # initial + transfer - escrowed
        if alice_balance_after_escrow != expected_alice_balance:
            raise Exception(f"Alice should have {expected_alice_balance} ERC20 after escrow, got {alice_balance_after_escrow}")
        
        if escrow_balance != 50:
            raise Exception(f"Escrow should have 50 ERC20 tokens, got {escrow_balance}")
        
        # Verify the attestation was created (buy order is live)
        if not buy_attestation_uid:
            raise Exception("Buy attestation UID should be valid")
        
        print("✅ test_permit_and_buy_bundle_for_erc20 PASSED")
        
    except Exception as e:
        print(f"❌ test_permit_and_buy_bundle_for_erc20 FAILED: {e}")
        raise


if __name__ == "__main__":
    asyncio.run(test_permit_and_buy_bundle_for_erc20())
