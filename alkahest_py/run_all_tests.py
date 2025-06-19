#!/usr/bin/env python3
"""
Comprehensive test runner for all alkahest-py test flows.
This script runs all the Python test flows that correspond to the Rust test functions in main.rs.
"""

import asyncio
import sys
import traceback
from typing import List, Callable

from test_erc20_escrow_obligation_statement import test_basic_encode_decode as test_erc20_escrow_encode_decode
from test_erc20_payment_obligation_statement import test_basic_encode_decode as test_erc20_payment_encode_decode
from test_erc721_escrow_obligation_statement import test_basic_encode_decode as test_erc721_escrow_encode_decode
from test_erc721_payment_obligation_statement import test_basic_encode_decode as test_erc721_payment_encode_decode
from test_erc1155_escrow_obligation_statement import test_basic_encode_decode as test_erc1155_escrow_encode_decode
from test_erc1155_payment_obligation_statement import test_basic_encode_decode as test_erc1155_payment_encode_decode
from test_string_obligation_statement import test_basic_encode_decode as test_string_obligation_encode_decode
from test_erc20_approval import test_erc20_approvals
from test_erc20_approve_if_less import test_approve_if_less
from test_buy_with_erc20 import test_buy_with_erc20
from test_pay_with_erc20 import test_pay_with_erc20
from test_permit_and_buy_with_erc20 import test_permit_and_buy_with_erc20
from test_permit_and_pay_with_erc20 import test_permit_and_pay_with_erc20
from test_buy_erc20_for_erc20 import test_buy_erc20_for_erc20
from test_permit_and_buy_erc20_for_erc20 import test_permit_and_buy_erc20_for_erc20
from test_pay_erc20_for_erc20 import test_pay_erc20_for_erc20
from test_permit_and_pay_erc20_for_erc20 import test_permit_and_pay_erc20_for_erc20
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
from test_erc721_approve import test_erc721_approve
from test_erc721_approve_all import test_erc721_approve_all
from test_erc721_revoke_all import test_erc721_revoke_all
from test_buy_with_erc721 import test_buy_with_erc721
from test_pay_with_erc721 import test_pay_with_erc721
from test_buy_erc721_for_erc721 import test_buy_erc721_for_erc721
from test_pay_erc721_for_erc721 import test_pay_erc721_for_erc721
from test_collect_expired import test_collect_expired
from test_buy_erc20_with_erc721 import test_buy_erc20_with_erc721
from test_buy_erc1155_with_erc721 import test_buy_erc1155_with_erc721
from test_buy_bundle_with_erc721 import test_buy_bundle_with_erc721
from test_pay_erc721_for_erc20 import test_pay_erc721_for_erc20
from test_pay_erc721_for_erc1155 import test_pay_erc721_for_erc1155
from test_pay_erc721_for_bundle import test_pay_erc721_for_bundle

# ERC1155 Tests
from test_erc1155_approve_all import test_erc1155_approve_all
from test_erc1155_revoke_all import test_erc1155_revoke_all
from test_buy_with_erc1155 import test_buy_with_erc1155
from test_pay_with_erc1155 import test_pay_with_erc1155
from test_buy_erc1155_for_erc1155 import test_buy_erc1155_for_erc1155
from test_pay_erc1155_for_erc1155 import test_pay_erc1155_for_erc1155
from test_buy_erc20_with_erc1155 import test_buy_erc20_with_erc1155
from test_buy_erc721_with_erc1155 import test_buy_erc721_with_erc1155
from test_buy_bundle_with_erc1155 import test_buy_bundle_with_erc1155
from test_pay_erc1155_for_erc20 import test_pay_erc1155_for_erc20
from test_pay_erc1155_for_erc721 import test_pay_erc1155_for_erc721
from test_pay_erc1155_for_bundle import test_pay_erc1155_for_bundle
from test_erc1155_collect_expired import test_erc1155_collect_expired

# IEAS Types Test
from test_ieas_types import test_ieas_types


async def run_test(test_func: Callable, test_name: str) -> bool:
    try:
        await test_func()
        print(f"✅ {test_name.split(' - ')[0]} PASSED")
        return True
    except Exception as e:
        print(f"❌ {test_name.split(' - ')[0]} FAILED:")
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
        # IEAS Types Test
        (test_ieas_types, "IEAS Types - Python Bindings Test"),
        
        # Obligation Statement Encode/Decode Tests
        (test_erc20_escrow_encode_decode, "ERC20 Escrow Obligation Statement - Basic Encode/Decode"),
        (test_erc20_payment_encode_decode, "ERC20 Payment Obligation Statement - Basic Encode/Decode"),
        (test_erc721_escrow_encode_decode, "ERC721 Escrow Obligation Statement - Basic Encode/Decode"),
        (test_erc721_payment_encode_decode, "ERC721 Payment Obligation Statement - Basic Encode/Decode"),
        (test_erc1155_escrow_encode_decode, "ERC1155 Escrow Obligation Statement - Basic Encode/Decode"),
        (test_erc1155_payment_encode_decode, "ERC1155 Payment Obligation Statement - Basic Encode/Decode"),
        (test_string_obligation_encode_decode, "String Obligation Statement - Basic Encode/Decode"),
        
        # ERC20 Core Tests
        (test_erc20_approvals, "ERC20 Approvals - Payment and Escrow"),
        (test_approve_if_less, "ERC20 Approve If Less - Conditional Approval"),
        
        # ERC721 Core Tests
        (test_erc721_approve, "ERC721 Approve - Payment and Escrow Token Approval"),
        (test_erc721_approve_all, "ERC721 Approve All - Payment and Escrow Operator Approval"),
        (test_erc721_revoke_all, "ERC721 Revoke All - Revoke Operator Approval"),
        
        # ERC721 Transaction Tests
        (test_buy_with_erc721, "ERC721 Buy with ERC721 - Custom Arbiter Purchase"),
        (test_pay_with_erc721, "ERC721 Pay with ERC721 - Direct NFT Payment"),
        (test_buy_erc721_for_erc721, "ERC721 Buy ERC721 for ERC721 - NFT to NFT Exchange Escrow"),
        (test_pay_erc721_for_erc721, "ERC721 Pay ERC721 for ERC721 - NFT to NFT Exchange Fulfillment"),
        (test_collect_expired, "ERC721 Collect Expired - Reclaim Expired Escrow"),
        (test_buy_erc20_with_erc721, "ERC721 Buy ERC20 with ERC721 - Use NFT to Buy Tokens"),
        (test_buy_erc1155_with_erc721, "ERC721 Buy ERC1155 with ERC721 - Use NFT to Buy Multi-tokens"),
        (test_buy_bundle_with_erc721, "ERC721 Buy Bundle with ERC721 - Use NFT to Buy Token Bundle"),
        (test_pay_erc721_for_erc20, "ERC721 Pay ERC721 for ERC20 - Use NFT to Fulfill Token Escrow"),
        (test_pay_erc721_for_erc1155, "ERC721 Pay ERC721 for ERC1155 - Use NFT to Fulfill Multi-token Escrow"),
        (test_pay_erc721_for_bundle, "ERC721 Pay ERC721 for Bundle - Use NFT to Fulfill Bundle Escrow"),
        
        # ERC1155 Core Tests
        (test_erc1155_approve_all, "ERC1155 Approve All - Payment and Escrow Operator Approval"),
        (test_erc1155_revoke_all, "ERC1155 Revoke All - Revoke Operator Approval"),
        
        # ERC1155 Transaction Tests
        (test_buy_with_erc1155, "ERC1155 Buy with ERC1155 - Custom Arbiter Purchase"),
        (test_pay_with_erc1155, "ERC1155 Pay with ERC1155 - Direct Multi-token Payment"),
        (test_buy_erc1155_for_erc1155, "ERC1155 Buy ERC1155 for ERC1155 - Multi-token to Multi-token Exchange Escrow"),
        (test_pay_erc1155_for_erc1155, "ERC1155 Pay ERC1155 for ERC1155 - Multi-token to Multi-token Exchange Fulfillment"),
        (test_erc1155_collect_expired, "ERC1155 Collect Expired - Reclaim Expired Multi-token Escrow"),
        (test_buy_erc20_with_erc1155, "ERC1155 Buy ERC20 with ERC1155 - Use Multi-tokens to Buy Tokens"),
        (test_buy_erc721_with_erc1155, "ERC1155 Buy ERC721 with ERC1155 - Use Multi-tokens to Buy NFT"),
        (test_buy_bundle_with_erc1155, "ERC1155 Buy Bundle with ERC1155 - Use Multi-tokens to Buy Token Bundle"),
        (test_pay_erc1155_for_erc20, "ERC1155 Pay ERC1155 for ERC20 - Use Multi-tokens to Fulfill Token Escrow"),
        (test_pay_erc1155_for_erc721, "ERC1155 Pay ERC1155 for ERC721 - Use Multi-tokens to Fulfill NFT Escrow"),
        (test_pay_erc1155_for_bundle, "ERC1155 Pay ERC1155 for Bundle - Use Multi-tokens to Fulfill Bundle Escrow"),
        
        # Transaction Flow Tests
        (test_buy_with_erc20, "ERC20 Buy with ERC20 - Escrow Creation"),
        (test_pay_with_erc20, "ERC20 Pay with ERC20 - Direct Payment"),
        (test_permit_and_pay_with_erc20, "ERC20 Permit and Pay - Signature-based Payment"),
        (test_buy_erc20_for_erc20, "ERC20 Buy ERC20 for ERC20 - Token Exchange Escrow"),
        (test_permit_and_buy_erc20_for_erc20, "ERC20 Permit and Buy ERC20 for ERC20 - Signature-based Exchange"),
        (test_permit_and_buy_with_erc20, "ERC20 Permit and Buy with ERC20 - Signature-based Purchase"),
        (test_pay_erc20_for_erc20, "ERC20 Pay ERC20 for ERC20 - Order Fulfillment"),
        (test_permit_and_pay_erc20_for_erc20, "ERC20 Permit and Pay ERC20 for ERC20 - Signature-based Order Fulfillment"),
        
        # Cross-token Buy flows (Alice creates escrow to buy tokens)
        (test_buy_erc721_for_erc20, "ERC20 Buy ERC721 for ERC20 - NFT Purchase Order"),
        (test_permit_and_buy_erc721_for_erc20, "ERC20 Permit and Buy ERC721 for ERC20 - Signature-based NFT Purchase"),
        (test_buy_erc1155_for_erc20, "ERC20 Buy ERC1155 for ERC20 - Multi-token Purchase Order"),
        (test_permit_and_buy_erc1155_for_erc20, "ERC20 Permit and Buy ERC1155 for ERC20 - Signature-based Multi-token Purchase"),
        (test_buy_bundle_for_erc20, "ERC20 Buy Bundle for ERC20 - Token Bundle Purchase Order"),
        (test_permit_and_buy_bundle_for_erc20, "ERC20 Permit and Buy Bundle for ERC20 - Signature-based Bundle Purchase"),
        
        # Cross-token Pay flows (Alice fulfills Bob's escrow)
        (test_pay_erc20_for_erc721, "ERC20 Pay ERC20 for ERC721 - NFT Order Fulfillment"),
        (test_permit_and_pay_erc20_for_erc721, "ERC20 Permit and Pay ERC20 for ERC721 - Signature-based NFT Fulfillment"),
        (test_pay_erc20_for_erc1155, "ERC20 Pay ERC20 for ERC1155 - Multi-token Order Fulfillment"),
        (test_permit_and_pay_erc20_for_erc1155, "ERC20 Permit and Pay ERC20 for ERC1155 - Signature-based Multi-token Fulfillment"),
        (test_pay_erc20_for_bundle, "ERC20 Pay ERC20 for Bundle - Bundle Order Fulfillment"),
        (test_permit_and_pay_erc20_for_bundle, "ERC20 Permit and Pay ERC20 for Bundle - Signature-based Bundle Fulfillment"),
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
  
    if failed == 0:
        print(f"\n🎉 ALL {passed} TESTS PASSED! Alkahest-py functionality is working correctly.")
    
    sys.exit(0 if failed == 0 else 1)


if __name__ == "__main__":
    asyncio.run(main())
