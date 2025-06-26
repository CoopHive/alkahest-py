import pytest
from alkahest_py import PyTestEnvManager, PyMockERC20

@pytest.mark.asyncio
async def test_permit_and_buy_erc1155_for_erc20():
    """
    Test buying ERC1155 tokens with ERC20 tokens using permit (no pre-approval needed).
    This corresponds to test_permit_and_buy_erc1155_for_erc20() in main.rs
    """
    env = PyTestEnvManager()
    
    # Setup mock ERC20 token  
    mock_erc20 = PyMockERC20(env.mock_addresses.erc20_a, env.god_wallet_provider)
    
    # Give Alice some ERC20 tokens (for bidding)
    alice_initial_erc20 = mock_erc20.balance_of(env.alice)
    mock_erc20.transfer(env.alice, 100)
    alice_after_transfer = mock_erc20.balance_of(env.alice)
    
    assert not (alice_after_transfer != alice_initial_erc20 + 100), "Alice ERC20 transfer failed. Expected {alice_initial_erc20 + 100}, got {alice_after_transfer}"
    
    # Alice creates buy order: offers 50 ERC20 tokens for ERC1155 tokens (ID 1, amount 10)
    # Using permit, so no pre-approval needed
    bid_data = {"address": env.mock_addresses.erc20_a, "value": 50}  # Alice offers ERC20
    ask_data = {"address": env.mock_addresses.erc1155_a, "id": 1, "value": 10}  # Alice wants ERC1155 ID 1, amount 10
    
    # Alice creates the buy order for ERC1155 with permit (no pre-approval needed)
    buy_result = await env.alice_client.erc20.permit_and_buy_erc1155_for_erc20(bid_data, ask_data, 0)
    
    assert not (not buy_result['log']['uid'] or buy_result['log']['uid'] == "0x0000000000000000000000000000000000000000000000000000000000000000"), "Invalid buy attestation UID"
    
    buy_attestation_uid = buy_result['log']['uid']
    
    # Verify Alice's ERC20 tokens are in escrow
    alice_balance_after_escrow = mock_erc20.balance_of(env.alice)
    escrow_balance = mock_erc20.balance_of(env.addresses.erc20_addresses.escrow_obligation)
    
    expected_alice_balance = alice_initial_erc20 + 100 - 50  # initial + transfer - escrowed
    assert not (alice_balance_after_escrow != expected_alice_balance), "Alice should have {expected_alice_balance} ERC20 after escrow, got {alice_balance_after_escrow}"
    
    assert not (escrow_balance != 50), "Escrow should have 50 ERC20 tokens, got {escrow_balance}"
    
    # Verify the attestation was created (buy order is live)
    assert not (not buy_attestation_uid), "Buy attestation UID should be valid"
