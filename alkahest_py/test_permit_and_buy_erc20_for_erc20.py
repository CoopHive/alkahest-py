#!/usr/bin/env python3
"""
Test flow for permit_and_buy_erc20_for_erc20 functionality.
This test demonstrates creating an escrow to trade ERC20 tokens using permit signature (no pre-approval needed).
"""

import asyncio
from alkahest_py import PyTestEnvManager, PyMockERC20


async def test_permit_and_buy_erc20_for_erc20():
    """Test the complete permit_and_buy_erc20_for_erc20 flow."""
    
    try:
        env = PyTestEnvManager()
        mock_erc20_a = PyMockERC20(env.mock_addresses.erc20_a, env.god_wallet_provider)
        
        transfer_amount = 100
        mock_erc20_a.transfer(env.alice, transfer_amount)
        
        alice_after_transfer = mock_erc20_a.balance_of(env.alice)
        if alice_after_transfer != transfer_amount:
            raise Exception(f"Transfer failed. Expected {transfer_amount}, got {alice_after_transfer}")
        
        bid_amount = 100
        ask_amount = 200
        bid_data = {"address": env.mock_addresses.erc20_a, "value": bid_amount}
        ask_data = {"address": env.mock_addresses.erc20_b, "value": ask_amount}
        expiration = 0
        
        escrow_result = await env.alice_client.erc20.permit_and_buy_erc20_for_erc20(bid_data, ask_data, expiration)
        
        alice_final_a = mock_erc20_a.balance_of(env.alice)
        escrow_balance_a = mock_erc20_a.balance_of(env.addresses.erc20_addresses.escrow_obligation)
        
        expected_alice_balance = alice_after_transfer - bid_amount
        if alice_final_a != expected_alice_balance:
            raise Exception(f"Alice balance incorrect. Expected {expected_alice_balance}, got {alice_final_a}")
        
        if escrow_balance_a != bid_amount:
            raise Exception(f"Escrow balance incorrect. Expected {bid_amount}, got {escrow_balance_a}")
        
        if not escrow_result['log']['uid'] or escrow_result['log']['uid'] == "0x0000000000000000000000000000000000000000000000000000000000000000":
            raise Exception("Invalid attestation UID")
        
        return True
        
    except Exception as e:
        print(f"\n❌ FAIL: Test failed - {e}")
        return False


async def main():
    success = await test_permit_and_buy_erc20_for_erc20()
    
    if success:
        print("🎉 SUCCESS! The permit_and_buy_erc20_for_erc20 test flow completed successfully!")
        return True
    else:
        print("💥 Test failed. Please check the error messages above.")
        return False


if __name__ == "__main__":
    asyncio.run(main())
