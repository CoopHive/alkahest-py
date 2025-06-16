#!/usr/bin/env python3
"""
Test runner for ERC20 Escrow Obligation Statement tests.
"""

import asyncio
import sys


async def main():
    """Main test runner that executes all ERC20-related tests."""
    
    try:
        from test_erc20_escrow_obligation_statement import test_basic_encode_decode
    except ImportError as e:
        print(f"Failed to import test module: {e}")
        return False
    
    tests = [
        ("ERC20 Escrow Obligation Statement - Basic Encode/Decode", test_basic_encode_decode),
    ]
    
    passed = 0
    failed = 0
    
    for test_name, test_func in tests:
        try:
            result = await test_func()
            if result:
                passed += 1
            else:
                print(f"FAILED: {test_name}")
                failed += 1
        except Exception as e:
            print(f"ERROR in {test_name}: {e}")
            failed += 1
    
    print(f"Passed: {passed}")
    print(f"Failed: {failed}")
    print(f"Total:  {passed + failed}")
    
    return 0 if failed == 0 else 1


if __name__ == "__main__":
    exit_code = asyncio.run(main())
    sys.exit(exit_code)
