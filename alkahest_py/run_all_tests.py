#!/usr/bin/env python3
"""
Comprehensive test runner for all alkahest-py test flows.
This script runs all the Python test flows that correspond to the Rust test functions in main.rs.
"""

import asyncio
import sys
import traceback
from typing import List, Callable

# Import all test functions
from test_buy_erc721_for_erc20 import test_buy_erc721_for_erc20
from test_buy_erc1155_for_erc20 import test_buy_erc1155_for_erc20
from test_buy_bundle_for_erc20 import test_buy_bundle_for_erc20
from test_permit_and_buy_erc721_for_erc20 import test_permit_and_buy_erc721_for_erc20
from test_permit_and_buy_erc1155_for_erc20 import test_permit_and_buy_erc1155_for_erc20
from test_permit_and_buy_bundle_for_erc20 import test_permit_and_buy_bundle_for_erc20
from test_pay_erc20_for_erc721 import test_pay_erc20_for_erc721
from test_permit_and_pay_erc20_for_erc721 import test_permit_and_pay_erc20_for_erc721
from test_pay_erc20_for_erc1155 import test_pay_erc20_for_erc1155
from test_permit_and_pay_erc20_for_erc1155 import test_permit_and_pay_erc20_for_erc1155
from test_pay_erc20_for_bundle import test_pay_erc20_for_bundle
from test_permit_and_pay_erc20_for_bundle import test_permit_and_pay_erc20_for_bundle


async def run_test(test_func: Callable, test_name: str) -> bool:
    """Run a single test function and return True if it passes, False if it fails."""
    try:
        print(f"\n🧪 Running {test_name}...")
        await test_func()
        return True
    except Exception as e:
        print(f"❌ {test_name} FAILED:")
        print(f"   Error: {str(e)}")
        if "--verbose" in sys.argv:
            print(f"   Traceback:")
            traceback.print_exc(limit=3)
        return False


async def main():
    """Run all test flows and report results."""
    print("🚀 Starting Alkahest-py Test Suite")
    print("=" * 60)
    
    # Define all test cases
    test_cases = [
        # Buy flows (Alice creates escrow to buy tokens)
        (test_buy_erc721_for_erc20, "test_buy_erc721_for_erc20"),
        (test_buy_erc1155_for_erc20, "test_buy_erc1155_for_erc20"),
        (test_buy_bundle_for_erc20, "test_buy_bundle_for_erc20"),
        
        # Permit + Buy flows (Alice creates escrow with permit)
        (test_permit_and_buy_erc721_for_erc20, "test_permit_and_buy_erc721_for_erc20"),
        (test_permit_and_buy_erc1155_for_erc20, "test_permit_and_buy_erc1155_for_erc20"),
        (test_permit_and_buy_bundle_for_erc20, "test_permit_and_buy_bundle_for_erc20"),
        
        # Pay flows (Alice fulfills Bob's escrow)
        (test_pay_erc20_for_erc721, "test_pay_erc20_for_erc721"),
        (test_pay_erc20_for_erc1155, "test_pay_erc20_for_erc1155"),
        (test_pay_erc20_for_bundle, "test_pay_erc20_for_bundle"),
        
        # Permit + Pay flows (Alice fulfills Bob's escrow with permit)
        (test_permit_and_pay_erc20_for_erc721, "test_permit_and_pay_erc20_for_erc721"),
        (test_permit_and_pay_erc20_for_erc1155, "test_permit_and_pay_erc20_for_erc1155"),
        (test_permit_and_pay_erc20_for_bundle, "test_permit_and_pay_erc20_for_bundle"),
    ]
    
    # Run all tests
    passed = 0
    failed = 0
    failed_tests = []
    
    for test_func, test_name in test_cases:
        success = await run_test(test_func, test_name)
        if success:
            passed += 1
        else:
            failed += 1
            failed_tests.append(test_name)
    
    # Print summary
    print("\n" + "=" * 60)
    print("📊 TEST SUMMARY")
    print("=" * 60)
    print(f"✅ Passed: {passed}")
    print(f"❌ Failed: {failed}")
    print(f"📈 Total:  {passed + failed}")
    
    if failed > 0:
        print(f"\n❌ Failed tests:")
        for test_name in failed_tests:
            print(f"   - {test_name}")
    
    # Print test descriptions
    print(f"\n📝 TEST DESCRIPTIONS")
    print("=" * 60)
    print("🔵 Buy Flows - User creates escrow to buy tokens:")
    print("   • test_buy_erc721_for_erc20 - Buy NFT with ERC20 tokens")
    print("   • test_buy_erc1155_for_erc20 - Buy ERC1155 tokens with ERC20")
    print("   • test_buy_bundle_for_erc20 - Buy token bundle with ERC20")
    
    print("\n🟡 Permit + Buy Flows - User creates escrow with permit:")
    print("   • test_permit_and_buy_erc721_for_erc20 - Buy NFT with permit")
    print("   • test_permit_and_buy_erc1155_for_erc20 - Buy ERC1155 with permit")
    print("   • test_permit_and_buy_bundle_for_erc20 - Buy bundle with permit")
    
    print("\n🟢 Pay Flows - User fulfills existing escrow:")
    print("   • test_pay_erc20_for_erc721 - Pay ERC20 to get NFT from escrow")
    print("   • test_pay_erc20_for_erc1155 - Pay ERC20 to get ERC1155 from escrow")
    print("   • test_pay_erc20_for_bundle - Pay ERC20 to get bundle from escrow")
    
    print("\n🟣 Permit + Pay Flows - User fulfills escrow with permit:")
    print("   • test_permit_and_pay_erc20_for_erc721 - Pay with permit for NFT")
    print("   • test_permit_and_pay_erc20_for_erc1155 - Pay with permit for ERC1155")
    print("   • test_permit_and_pay_erc20_for_bundle - Pay with permit for bundle")
    
    print(f"\n💡 Note: Some tests use PyMockERC721 and PyMockERC1155 which are")
    print(f"   currently not exported from the Rust module. Once those are")
    print(f"   available, the tests can be enhanced with full token verification.")
    
    # Exit with appropriate code
    sys.exit(0 if failed == 0 else 1)


if __name__ == "__main__":
    asyncio.run(main())
