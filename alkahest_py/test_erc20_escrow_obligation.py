"""
Test encode/decode functionality for PyERC20EscrowObligation
"""
import asyncio
from alkahest_py import PyTestEnvManager, PyERC20EscrowObligation


async def test_encode_decode_erc20_escrow_obligation():
    """Test encoding and decoding PyERC20EscrowObligation"""
    print("🚀 Testing PyERC20EscrowObligation encode/decode functionality...")
    
    # Setup test environment
    env = PyTestEnvManager()
    
    # Create test data
    token_address = env.mock_addresses.erc20_a
    amount = 100
    arbiter_address = env.addresses.erc20_addresses.payment_obligation
    demand_data = [1, 2, 3, 4, 5]
    
    print(f"📝 Original data:")
    print(f"  Token: {token_address}")
    print(f"  Amount: {amount}")
    print(f"  Arbiter: {arbiter_address}")
    print(f"  Demand: {demand_data}")
    
    # Create PyERC20EscrowObligation instance
    original_obligation = PyERC20EscrowObligation(
        token=token_address,
        amount=amount,
        arbiter=arbiter_address,
        demand=demand_data
    )
    
    print(f"\n📦 Created obligation: {repr(original_obligation)}")
    
    # Test encoding
    try:
        encoded_data = env.alice_client.erc20.encode_escrow_statement(original_obligation)
        print(f"✅ ENCODE SUCCESS: Generated {len(encoded_data)} bytes")
        print(f"   Encoded (hex): {encoded_data[:20].hex()}..." if len(encoded_data) > 20 else f"   Encoded (hex): {encoded_data.hex()}")
    except Exception as e:
        print(f"❌ ENCODE FAILED: {e}")
        return False
    
    # Test decoding
    try:
        decoded_obligation = env.alice_client.erc20.decode_escrow_statement(encoded_data)
        print(f"✅ DECODE SUCCESS: {repr(decoded_obligation)}")
    except Exception as e:
        print(f"❌ DECODE FAILED: {e}")
        return False
    
    # Verify round-trip integrity
    print(f"\n🔍 Verifying round-trip integrity...")
    
    tests_passed = 0
    total_tests = 4
    
    # Check token address
    if original_obligation.token.lower() == decoded_obligation.token.lower():
        print(f"✅ Token address matches: {decoded_obligation.token}")
        tests_passed += 1
    else:
        print(f"❌ Token mismatch: {original_obligation.token} != {decoded_obligation.token}")
    
    # Check amount
    if original_obligation.amount == decoded_obligation.amount:
        print(f"✅ Amount matches: {decoded_obligation.amount}")
        tests_passed += 1
    else:
        print(f"❌ Amount mismatch: {original_obligation.amount} != {decoded_obligation.amount}")
    
    # Check arbiter address
    if original_obligation.arbiter.lower() == decoded_obligation.arbiter.lower():
        print(f"✅ Arbiter address matches: {decoded_obligation.arbiter}")
        tests_passed += 1
    else:
        print(f"❌ Arbiter mismatch: {original_obligation.arbiter} != {decoded_obligation.arbiter}")
    
    # Check demand data
    if original_obligation.demand == decoded_obligation.demand:
        print(f"✅ Demand data matches: {decoded_obligation.demand}")
        tests_passed += 1
    else:
        print(f"❌ Demand mismatch: {original_obligation.demand} != {decoded_obligation.demand}")
    
    print(f"\n📊 Test Results: {tests_passed}/{total_tests} tests passed")
    
    if tests_passed == total_tests:
        print("🎉 ALL TESTS PASSED! Round-trip encoding/decoding works correctly.")
        return True
    else:
        print("💥 SOME TESTS FAILED! Check the output above for details.")
        return False


async def test_edge_cases():
    """Test edge cases for PyERC20EscrowObligation"""
    print("\n🧪 Testing edge cases...")
    
    env = PyTestEnvManager()
    
   
    
    # Test with large amount
    print("\n📝 Test 2: Large amount")
    try:
        large_amount = 2**32 - 1  # Maximum uint32
        obligation = PyERC20EscrowObligation(
            token=env.mock_addresses.erc20_a,
            amount=large_amount,
            arbiter=env.addresses.erc20_addresses.payment_obligation,
            demand=[255, 254, 253]
        )
        
        encoded = env.alice_client.erc20.encode_escrow_statement(obligation)
        decoded = env.alice_client.erc20.decode_escrow_statement(encoded)
        
        if decoded.amount == large_amount:
            print(f"✅ Large amount handled correctly: {decoded.amount}")
        else:
            print(f"❌ Large amount failed: expected {large_amount}, got {decoded.amount}")
            
    except Exception as e:
        print(f"❌ Large amount test failed: {e}")

async def main():
    """Run all tests"""
    print("=" * 60)
    print("🧪 PyERC20EscrowObligation Encode/Decode Test Suite")
    print("=" * 60)
    
    success = True
    
    # Run main encode/decode test
    try:
        main_test_passed = await test_encode_decode_erc20_escrow_obligation()
        success = success and main_test_passed
    except Exception as e:
        print(f"❌ Main test failed with exception: {e}")
        success = False
    
    # Run edge case tests
    try:
        await test_edge_cases()
    except Exception as e:
        print(f"❌ Edge case tests failed with exception: {e}")
        success = False
    
    print("\n" + "=" * 60)
    if success:
        print("🎉 ALL TESTS COMPLETED! Check individual test results above.")
    else:
        print("💥 SOME TESTS HAD ISSUES! Check the output above for details.")
    print("=" * 60)


if __name__ == "__main__":
    asyncio.run(main())
