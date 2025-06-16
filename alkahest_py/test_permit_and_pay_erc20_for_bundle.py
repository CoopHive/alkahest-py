import asyncio
from alkahest_py import PyTestEnvManager, PyMockERC20


async def test_permit_and_pay_erc20_for_bundle():
    """
    Test paying ERC20 tokens to fulfill a token bundle escrow using permit (no pre-approval needed).
    This corresponds to test_permit_and_pay_erc20_for_bundle() in main.rs
    
    Flow: Bob escrows a bundle (ERC20 + ERC721 + ERC1155), Alice pays ERC20 using permit to get the bundle
    """
    try:
        env = PyTestEnvManager()
        
        # Setup mock tokens
        mock_erc20_a = PyMockERC20(env.mock_addresses.erc20_a, env.god_wallet_provider)  # Alice's payment token
        mock_erc20_b = PyMockERC20(env.mock_addresses.erc20_b, env.god_wallet_provider)  # Bob's bundle token
        # Note: PyMockERC721 and PyMockERC1155 not available yet, but this shows the intended flow
        
        # Give Alice ERC20 tokens for payment
        alice_initial_erc20 = mock_erc20_a.balance_of(env.alice)
        mock_erc20_a.transfer(env.alice, 100)
        alice_after_transfer = mock_erc20_a.balance_of(env.alice)
        
        if alice_after_transfer != alice_initial_erc20 + 100:
            raise Exception(f"Alice ERC20 transfer failed. Expected {alice_initial_erc20 + 100}, got {alice_after_transfer}")
        
        # Give Bob bundle tokens
        mock_erc20_b.transfer(env.bob, 50)  # Bob gets ERC20 tokens for the bundle
        # Bob would also mint/own ERC721 and ERC1155 tokens
        # mock_erc721_a.mint(env.bob)  # ERC721 token ID 1
        # mock_erc1155_a.mint(env.bob, token_id=1, amount=20)  # ERC1155 tokens
        
        # Create test data
        erc20_amount = 50  # Alice pays this much
        bob_erc20_amount = 25  # Half of Bob's ERC20 tokens go into bundle
        erc721_token_id = 1
        erc1155_token_id = 1
        erc1155_bundle_amount = 10  # Half of Bob's ERC1155 tokens go into bundle
        expiration = 3600  # 1 hour from now
        
        # Create token bundle
        bundle_data = {
            "erc20s": [{"address": env.mock_addresses.erc20_b, "value": bob_erc20_amount}],
            "erc721s": [{"address": env.mock_addresses.erc721_a, "id": erc721_token_id}],
            "erc1155s": [{"address": env.mock_addresses.erc1155_a, "id": erc1155_token_id, "value": erc1155_bundle_amount}]
        }
        
        # Step 1: Bob approves his tokens for the bundle escrow
        await env.bob_client.token_bundle.approve(bundle_data, "escrow")
        
        # Step 2: Bob creates bundle escrow demanding ERC20 from Alice
        # First encode the payment statement data as the demand
        payment_statement_data = {
            "token": env.mock_addresses.erc20_a,
            "amount": erc20_amount,
            "payee": env.bob
        }
        
        arbiter_data = {
            "arbiter": env.addresses.erc20_addresses.payment_obligation,
            "demand": payment_statement_data  # This would be ABI encoded in real implementation
        }
        
        buy_result = await env.bob_client.token_bundle.buy_with_bundle(
            bundle_data, arbiter_data, expiration
        )
        
        if not buy_result['log']['uid'] or buy_result['log']['uid'] == "0x0000000000000000000000000000000000000000000000000000000000000000":
            raise Exception("Invalid buy attestation UID")
        
        buy_attestation_uid = buy_result['log']['uid']
        
        # Check balances before the exchange
        initial_alice_erc20_balance = mock_erc20_a.balance_of(env.alice)
        initial_alice_bob_erc20_balance = mock_erc20_b.balance_of(env.alice)
        # initial_alice_erc1155_balance = mock_erc1155_a.balance_of(env.alice, erc1155_token_id)  # When available
        
        # Step 3: Alice fulfills Bob's bundle escrow using permit (no pre-approval needed)
        pay_result = await env.alice_client.erc20.permit_and_pay_erc20_for_bundle(buy_attestation_uid)
        
        if not pay_result['log']['uid'] or pay_result['log']['uid'] == "0x0000000000000000000000000000000000000000000000000000000000000000":
            raise Exception("Invalid payment attestation UID")
        
        # Verify token transfers
        # 1. Alice should now own ERC721 (would check with mock_erc721_a.owner_of when available)
        # final_erc721_owner = mock_erc721_a.owner_of(erc721_token_id)
        # assert final_erc721_owner == env.alice
        
        # 2. Alice should have received Bob's ERC20
        final_alice_bob_erc20_balance = mock_erc20_b.balance_of(env.alice)
        alice_received_bob_erc20 = final_alice_bob_erc20_balance - initial_alice_bob_erc20_balance
        if alice_received_bob_erc20 != bob_erc20_amount:
            raise Exception(f"Alice should have received {bob_erc20_amount} ERC20 tokens from Bob, got {alice_received_bob_erc20}")
        
        # 3. Alice should have received Bob's ERC1155 (would check with mock_erc1155_a.balance_of when available)
        # final_alice_erc1155_balance = mock_erc1155_a.balance_of(env.alice, erc1155_token_id)
        # alice_received_erc1155 = final_alice_erc1155_balance - initial_alice_erc1155_balance
        # assert alice_received_erc1155 == erc1155_bundle_amount
        
        # 4. Alice should have spent her ERC20
        final_alice_erc20_balance = mock_erc20_a.balance_of(env.alice)
        alice_spent = initial_alice_erc20_balance - final_alice_erc20_balance
        if alice_spent != erc20_amount:
            raise Exception(f"Alice should have spent {erc20_amount} ERC20 tokens, spent {alice_spent}")
        
        # 5. Bob should have received Alice's ERC20
        bob_erc20_balance = mock_erc20_a.balance_of(env.bob)
        if bob_erc20_balance != erc20_amount:
            raise Exception(f"Bob should have received {erc20_amount} ERC20 tokens, got {bob_erc20_balance}")
        
        print("✅ test_permit_and_pay_erc20_for_bundle PASSED")
        
    except Exception as e:
        print(f"❌ test_permit_and_pay_erc20_for_bundle FAILED: {e}")
        raise


if __name__ == "__main__":
    asyncio.run(test_permit_and_pay_erc20_for_bundle())
