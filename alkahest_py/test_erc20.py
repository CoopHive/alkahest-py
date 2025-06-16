#!/usr/bin/env python3
"""
Test runner for all ERC20-related tests.
"""

import asyncio
import sys


async def main():
    """Main test runner that executes all ERC20-related tests."""
    
    try:
        from test_erc20_escrow_obligation_statement import test_basic_encode_decode
        from test_erc20_approval import test_erc20_approvals
        from test_erc20_approve_if_less import test_approve_if_less
        from test_buy_with_erc20 import test_buy_with_erc20
        from test_pay_with_erc20 import test_pay_with_erc20
        from test_permit_and_pay_with_erc20 import test_permit_and_pay_with_erc20
        from test_buy_erc20_for_erc20 import test_buy_erc20_for_erc20
        from test_permit_and_buy_erc20_for_erc20 import test_permit_and_buy_erc20_for_erc20
        from test_permit_and_buy_with_erc20 import test_permit_and_buy_with_erc20
        from test_pay_erc20_for_erc20 import test_pay_erc20_for_erc20
        from test_permit_and_pay_erc20_for_erc20 import test_permit_and_pay_erc20_for_erc20
        from test_buy_erc721_for_erc20 import test_buy_erc721_for_erc20
        from test_permit_and_buy_erc721_for_erc20 import test_permit_and_buy_erc721_for_erc20
        from test_pay_erc20_for_erc721 import test_pay_erc20_for_erc721
        from test_permit_and_pay_erc20_for_erc721 import test_permit_and_pay_erc20_for_erc721
        from test_buy_erc1155_for_erc20 import test_buy_erc1155_for_erc20
        from test_permit_and_buy_erc1155_for_erc20 import test_permit_and_buy_erc1155_for_erc20
        from test_pay_erc20_for_erc1155 import test_pay_erc20_for_erc1155
        from test_permit_and_pay_erc20_for_erc1155 import test_permit_and_pay_erc20_for_erc1155
        from test_buy_bundle_for_erc20 import test_buy_bundle_for_erc20
        from test_permit_and_buy_bundle_for_erc20 import test_permit_and_buy_bundle_for_erc20
        from test_pay_erc20_for_bundle import test_pay_erc20_for_bundle
        from test_permit_and_pay_erc20_for_bundle import test_permit_and_pay_erc20_for_bundle
    except ImportError as e:
        print(f"Failed to import test module: {e}")
        return 1
    
    tests = [
        ("ERC20 Escrow Obligation Statement - Basic Encode/Decode", test_basic_encode_decode),
        ("ERC20 Approvals - Payment and Escrow", test_erc20_approvals),
        ("ERC20 Approve If Less - Conditional Approval", test_approve_if_less),
        ("ERC20 Buy with ERC20 - Escrow Creation", test_buy_with_erc20),
        ("ERC20 Pay with ERC20 - Direct Payment", test_pay_with_erc20),
        ("ERC20 Permit and Pay - Signature-based Payment", test_permit_and_pay_with_erc20),
        ("ERC20 Buy ERC20 for ERC20 - Token Exchange Escrow", test_buy_erc20_for_erc20),
        ("ERC20 Permit and Buy ERC20 for ERC20 - Signature-based Exchange", test_permit_and_buy_erc20_for_erc20),
        ("ERC20 Permit and Buy with ERC20 - Signature-based Purchase", test_permit_and_buy_with_erc20),
        ("ERC20 Pay ERC20 for ERC20 - Order Fulfillment", test_pay_erc20_for_erc20),
        ("ERC20 Permit and Pay ERC20 for ERC20 - Signature-based Order Fulfillment", test_permit_and_pay_erc20_for_erc20),
        ("ERC20 Buy ERC721 for ERC20 - NFT Purchase Order", test_buy_erc721_for_erc20),
        ("ERC20 Permit and Buy ERC721 for ERC20 - Signature-based NFT Purchase", test_permit_and_buy_erc721_for_erc20),
        ("ERC20 Buy ERC1155 for ERC20 - Multi-token Purchase Order", test_buy_erc1155_for_erc20),
        ("ERC20 Permit and Buy ERC1155 for ERC20 - Signature-based Multi-token Purchase", test_permit_and_buy_erc1155_for_erc20),
        ("ERC20 Buy Bundle for ERC20 - Token Bundle Purchase Order", test_buy_bundle_for_erc20),
        ("ERC20 Permit and Buy Bundle for ERC20 - Signature-based Bundle Purchase", test_permit_and_buy_bundle_for_erc20),
        ("ERC20 Pay ERC20 for ERC721 - NFT Order Fulfillment", test_pay_erc20_for_erc721),
        ("ERC20 Permit and Pay ERC20 for ERC721 - Signature-based NFT Fulfillment", test_permit_and_pay_erc20_for_erc721),
        ("ERC20 Pay ERC20 for ERC1155 - Multi-token Order Fulfillment", test_pay_erc20_for_erc1155),
        ("ERC20 Permit and Pay ERC20 for ERC1155 - Signature-based Multi-token Fulfillment", test_permit_and_pay_erc20_for_erc1155),
        ("ERC20 Pay ERC20 for Bundle - Bundle Order Fulfillment", test_pay_erc20_for_bundle),
        ("ERC20 Permit and Pay ERC20 for Bundle - Signature-based Bundle Fulfillment", test_permit_and_pay_erc20_for_bundle),
  ]
    
    passed = 0
    failed = 0
    
    print("🚀 Running ERC20 Test Suite...")
    print("=" * 60)
    
    for test_name, test_func in tests:
        print(f"\n📋 Running: {test_name}")
        print("-" * 40)
        
        try:
            result = await test_func()
            if result:
                passed += 1
            else:
                print(f"❌ FAILED: {test_name}")
                failed += 1
        except Exception as e:
            print(f"💥 ERROR in {test_name}: {e}")
            failed += 1
    
    print("\n" + "=" * 60)
    print("📊 ERC20 TEST SUITE SUMMARY")
    print("=" * 60)
    print(f"✅ Passed: {passed}")
    print(f"❌ Failed: {failed}")
    print(f"📈 Total:  {passed + failed}")
    
    if failed == 0:
        print("\n🎉 ALL ERC20 TESTS PASSED! ERC20 functionality is working correctly.")
        return 0
    else:
        print(f"\n💔 {failed} test(s) failed. Please check the errors above.")
        return 1


if __name__ == "__main__":
    exit_code = asyncio.run(main())
    sys.exit(exit_code)
