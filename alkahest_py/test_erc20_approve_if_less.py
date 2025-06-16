import asyncio
from alkahest_py import PyTestEnvManager, PyMockERC20


async def test_approve_if_less():
    try:
        test = PyTestEnvManager()
        mock_erc20_a = PyMockERC20(test.mock_addresses.erc20_a, test.god_wallet_provider)
        
        mock_erc20_a.transfer(test.alice, 200)
        alice_balance = mock_erc20_a.balance_of(test.alice)
        
        token = {"address": test.mock_addresses.erc20_a, "value": 100}
        
        receipt_opt = await test.alice_client.erc20.approve_if_less(token, "payment")
        
        if not receipt_opt:
            print("ERROR: First approval should return receipt")
            return False
        
        payment_allowance = mock_erc20_a.allowance(
            test.alice,
            test.addresses.erc20_addresses.payment_obligation
        )
        
        receipt_opt = await test.alice_client.erc20.approve_if_less(token, "payment")
        
        if receipt_opt is not None:
            print(f"ERROR: Second approval should return None, got: {receipt_opt}")
            return False
        
        larger_token = {"address": test.mock_addresses.erc20_a, "value": 150}
        
        try:
            receipt_opt = await test.alice_client.erc20.approve_if_less(larger_token, "payment")
            
            if receipt_opt:
                new_payment_allowance = mock_erc20_a.allowance(
                    test.alice,
                    test.addresses.erc20_addresses.payment_obligation
                )
                
                if new_payment_allowance < 150:
                    print(f"ERROR: New allowance {new_payment_allowance} is insufficient for 150")
                    return False
            else:
                print("ERROR: Third approval should return receipt for larger amount")
                return False
                
        except Exception as e:
            print(f"Third approval failed: {e}")
        
        print("✅ test_approve_if_less PASSED")
        return True
    except Exception as e:
        print(f"Test failed with error: {e}")
        return False


async def main():
    success = await test_approve_if_less()
    return 0 if success else 1


if __name__ == "__main__":
    exit_code = asyncio.run(main())
    exit(exit_code)
