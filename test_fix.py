#!/usr/bin/env python3
"""
Test script to verify that the "failed to register pending transaction to watch" error is fixed.
This test calls approve() followed by approve_if_less() which should reproduce the original issue.
"""

import asyncio
from alkahest_py.alkahest_py import PyTestEnvManager, PyMockERC20

async def test_sequential_calls():
    """Test multiple sequential function calls to verify the runtime fix."""
    print("🧪 Testing sequential function calls...")
    print("="*60)
    
    # Setup test environment
    env = PyTestEnvManager()
    mock_erc20 = PyMockERC20(env.mock_addresses.erc20_a, env.god_wallet_provider)
    
    # Transfer tokens to Alice
    mock_erc20.transfer(env.alice, 200)
    print(f"✅ Transferred 200 tokens to Alice")
    print(f"   Alice balance: {mock_erc20.balance_of(env.alice)}")
    
    token_data = {"address": env.mock_addresses.erc20_a, "value": 100}
    
    try:
        print("\n📝 Step 1: First approve() call...")
        receipt1 = await env.alice_client.erc20.approve(token_data, "payment")
        print(f"✅ First approve() successful: {receipt1}")
        
        print("\n📝 Step 2: Second approve_if_less() call...")
        receipt2 = await env.alice_client.erc20.approve_if_less(token_data, "payment")
        print(f"✅ Second approve_if_less() successful: {receipt2}")
        
        print("\n📝 Step 3: Third approve() call with different purpose...")
        receipt3 = await env.alice_client.erc20.approve(token_data, "escrow")
        print(f"✅ Third approve() successful: {receipt3}")
        
        print("\n📝 Step 4: Fourth approve_if_less() call...")
        larger_token = {"address": env.mock_addresses.erc20_a, "value": 150}
        receipt4 = await env.alice_client.erc20.approve_if_less(larger_token, "payment")
        print(f"✅ Fourth approve_if_less() successful: {receipt4}")
        
        print("\n🎉 SUCCESS! All sequential calls completed without errors!")
        print("✅ The 'failed to register pending transaction to watch' issue has been fixed.")
        return True
        
    except Exception as e:
        print(f"\n❌ ERROR: {e}")
        print("💥 The fix may not be working correctly.")
        return False

async def main():
    """Main test runner."""
    print("🚀 Testing Runtime Fix for 'failed to register pending transaction to watch'")
    print("="*80)
    
    success = await test_sequential_calls()
    
    print("\n" + "="*80)
    print("📊 TEST RESULTS")
    print("="*80)
    
    if success:
        print("🎉 ALL TESTS PASSED!")
        print("✅ The shared runtime fix is working correctly.")
        print("✅ Multiple sequential function calls now work without errors.")
    else:
        print("💥 TESTS FAILED!")
        print("❌ The fix needs further investigation.")
    
    return success

if __name__ == "__main__":
    asyncio.run(main())
