import asyncio
import time
from alkahest_py import PyTestEnvManager, PyMockERC1155


async def test_erc1155_collect_expired():
    """
    Test collecting expired escrowed ERC1155 tokens with time-based expiration.
    This corresponds to test_collect_expired() in main.rs for ERC1155
    
    Flow: 
    1. Mint ERC1155 tokens to Alice
    2. Alice creates escrow with short expiration
    3. Wait for escrow to expire
    4. Alice collects expired tokens back
    5. Verify tokens returned to Alice
    """
    try:
        env = PyTestEnvManager()
        
        # Setup mock ERC1155 token
        mock_erc1155_a = PyMockERC1155(env.mock_addresses.erc1155_a, env.god_wallet_provider)
        
        # Mint ERC1155 tokens to Alice
        mock_erc1155_a.mint(env.alice, 1, 10)
        print(f"Minted 10 ERC1155 tokens (ID: 1) to Alice")
        
        # Create trade data
        bid_data = {
            "address": env.mock_addresses.erc1155_a,
            "id": 1,
            "value": 5
        }
        
        ask_data = {
            "address": env.mock_addresses.erc1155_b,
            "id": 2,
            "value": 3
        }
        
        # Alice approves tokens for escrow
        await env.alice_client.erc1155.approve_all(env.mock_addresses.erc1155_a, "escrow")
        
        # Check initial balance
        initial_alice_balance = mock_erc1155_a.balance_of(env.alice, 1)
        
        # Alice makes escrow with a short expiration (current time + 15 seconds)
        expiration = int(time.time()) + 15
        buy_result = await env.alice_client.erc1155.buy_erc1155_for_erc1155(bid_data, ask_data, expiration)
        
        if not buy_result['log']['uid'] or buy_result['log']['uid'] == "0x0000000000000000000000000000000000000000000000000000000000000000":
            raise Exception("Invalid buy attestation UID")
        
        buy_attestation_uid = buy_result['log']['uid']
        
        # Verify tokens are in escrow
        escrow_balance = mock_erc1155_a.balance_of(env.addresses.erc1155_addresses.escrow_obligation, 1)
        alice_balance_after_escrow = mock_erc1155_a.balance_of(env.alice, 1)
        
        if escrow_balance != 5:
            raise Exception(f"5 tokens should be in escrow, got {escrow_balance}")
        
        if alice_balance_after_escrow != 5:
            raise Exception(f"Alice should have 5 tokens remaining, got {alice_balance_after_escrow}")
        
        print(f"ERC1155 tokens {bid_data['value']} in escrow at: {env.addresses.erc1155_addresses.escrow_obligation}")
        
        # Wait for expiration (wait 20 seconds to be safe)
        print("Waiting for escrow to expire...")
        time.sleep(20)
        
        # Alice collects expired funds
        collect_result = await env.alice_client.erc1155.collect_expired(buy_attestation_uid)
        print(f"Collected expired escrow, transaction: {collect_result}")
        
        # Verify tokens returned to Alice
        final_alice_balance = mock_erc1155_a.balance_of(env.alice, 1)
        final_escrow_balance = mock_erc1155_a.balance_of(env.addresses.erc1155_addresses.escrow_obligation, 1)
        
        if final_alice_balance != initial_alice_balance:
            raise Exception(f"All tokens should be returned to Alice. Expected {initial_alice_balance}, got {final_alice_balance}")
        
        if final_escrow_balance != 0:
            raise Exception(f"Escrow should be empty after collection. Got {final_escrow_balance}")
        
        print(f"ERC1155 tokens finally returned to Alice: {final_alice_balance}")
        
        print("✅ test_erc1155_collect_expired PASSED")
        return True
        
    except Exception as e:
        print(f"❌ test_erc1155_collect_expired FAILED: {e}")
        raise


async def main():
    try:
        success = await test_erc1155_collect_expired()
        return 0 if success else 1
    except Exception as e:
        print(f"Test execution failed: {e}")
        return 1


if __name__ == "__main__":
    exit_code = asyncio.run(main())
    exit(exit_code)
