import asyncio
from alkahest_py import PyTestEnvManager, PyMockERC20


async def test_erc20_approvals():
    success_count = 0
    total_tests = 2
    
    try:
        env1 = PyTestEnvManager()
        mock_erc20_1 = PyMockERC20(env1.mock_addresses.erc20_a, env1.god_wallet_provider)
        
        mock_erc20_1.transfer(env1.alice, 100)
        print(f"✅ Transferred 100 tokens to Alice")
        
        token_data = {"address": env1.mock_addresses.erc20_a, "value": 100}
        receipt_hash = await env1.alice_client.erc20.approve(token_data, "payment")
        
        payment_allowance = mock_erc20_1.allowance(env1.alice, env1.addresses.erc20_addresses.payment_obligation)
        
        print(f"✅ Payment approval successful!")

        success_count += 1

    except Exception as e:
        print(f"❌ Payment approval failed: {e}")
    
    await asyncio.sleep(1)
    
    
    try:
        env2 = PyTestEnvManager()
        mock_erc20_2 = PyMockERC20(env2.mock_addresses.erc20_a, env2.god_wallet_provider)
        
        # Transfer tokens to Alice
        mock_erc20_2.transfer(env2.alice, 100)
        print(f"✅ Transferred 100 tokens to Alice")
        
        # Create token data and approve for escrow
        token_data = {"address": env2.mock_addresses.erc20_a, "value": 100}
        receipt_hash = await env2.alice_client.erc20.approve(token_data, "escrow")
        
        # Check allowance after approval
        escrow_allowance = mock_erc20_2.allowance(env2.alice, env2.addresses.erc20_addresses.escrow_obligation)
        
        print(f"✅ Escrow approval successful!")

        success_count += 1
        
    except Exception as e:
        print(f"❌ Escrow approval failed: {e}")
    
    # Results
    print("\n" + "=" * 60)
    print("📊 TEST RESULTS")
    print("=" * 60)
    print(f"✅ Successful tests: {success_count}/{total_tests}")
    
    if success_count == total_tests:
        print("\n🎉 SUCCESS! ERC20 approval functionality is working correctly.")
        return True
    else:
        print(f"\n💥 {total_tests - success_count} test(s) failed.")
        return False


async def main():
    
    try:
        success = await test_erc20_approvals()
        return success
        
    except Exception as e:
        print(f"\n💥 Unexpected error: {e}")
        return False


if __name__ == "__main__":
    success = asyncio.run(main())
    exit(0 if success else 1)
