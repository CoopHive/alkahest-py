#!/usr/bin/env python3
"""
Test runner for ERC20 Escrow Obligation Statement tests.
This file imports and runs the test_erc20_escrow_obligation_statement module.
"""

import asyncio
import sys
import os


async def main():
    """Main test runner that executes all ERC20-related tests."""
    print("🚀 Starting ERC20 Test Suite...")
    print("=" * 60)
    
    # Import the test module
    try:
        from test_erc20_escrow_obligation_statement import test_basic_encode_decode
    except ImportError as e:
        print(f"❌ Failed to import test module: {e}")
        return False
    
    # List of tests to run
    tests = [
        ("ERC20 Escrow Obligation Statement - Basic Encode/Decode", test_basic_encode_decode),
    ]
    
    passed = 0
    failed = 0
    
    # Run each test
    for test_name, test_func in tests:
        print(f"\n📋 Running: {test_name}")
        print("-" * 40)
        
        try:
            result = await test_func()
            if result:
                print(f"✅ PASSED: {test_name}")
                passed += 1
            else:
                print(f"❌ FAILED: {test_name}")
                failed += 1
        except Exception as e:
            print(f"💥 ERROR in {test_name}: {e}")
            failed += 1
    
    # Print summary
    print("\n" + "=" * 60)
    print("📊 TEST SUMMARY")
    print("=" * 60)
    print(f"✅ Passed: {passed}")
    print(f"❌ Failed: {failed}")
    print(f"📈 Total:  {passed + failed}")
    
    if failed == 0:
        print("\n🎉 ALL TESTS PASSED! ERC20 functionality is working correctly.")
        return True
    else:
        print(f"\n💔 {failed} test(s) failed. Please check the errors above.")
        return False


if __name__ == "__main__":
    # Run the test suite
    success = asyncio.run(main())
    
    # Exit with appropriate code
    sys.exit(0 if success else 1)
