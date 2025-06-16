import asyncio
from alkahest_py import PyTestEnvManager, PyMockERC20


async def test_buy_erc721_for_erc20():
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
        
        # Alice creates buy order: offers 50 ERC20 tokens for NFT token ID 1
        # Note: We're testing the escrow creation, not the full NFT exchange
        bid_data = {"address": env.mock_addresses.erc20_a, "value": 50}  # Alice offers ERC20
        ask_data = {"address": env.mock_addresses.erc721_a, "id": 1}  # Alice wants NFT ID 1
        
        # Alice approves tokens for escrow
        await env.alice_client.erc20.approve(bid_data, "escrow")
        
        # Alice creates the buy order for ERC721
        buy_result = await env.alice_client.erc20.buy_erc721_for_erc20(bid_data, ask_data, 0)
        
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
        
        print("✅ test_buy_erc721_for_erc20 PASSED")
        return True
        
    except Exception as e:
        print(f"Test failed: {e}")
        return False
        
    except Exception as e:
        print(f"Test failed: {e}")
        return False


async def main():
    success = await test_buy_erc721_for_erc20()
    return 0 if success else 1


if __name__ == "__main__":
    exit_code = asyncio.run(main())
    exit(exit_code)
