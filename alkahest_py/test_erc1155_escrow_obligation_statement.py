import asyncio
from alkahest_py import PyTestEnvManager, PyERC1155EscrowObligationStatement


async def test_basic_encode_decode():
    try:
        env = PyTestEnvManager()
        
        obligation = PyERC1155EscrowObligationStatement(
            token=env.mock_addresses.erc1155_a,
            token_id="54321",
            amount="250",
            arbiter=env.addresses.erc1155_addresses.escrow_obligation,
            demand=[1, 2, 3, 4, 5]
        )
        
        encoded_data = PyERC1155EscrowObligationStatement.encode(obligation)
        decoded_obligation = PyERC1155EscrowObligationStatement.decode(encoded_data)

        assert obligation.token_id == decoded_obligation.token_id, "Token ID mismatch"
        assert obligation.amount == decoded_obligation.amount, "Amount mismatch"
        assert obligation.token.lower() == decoded_obligation.token.lower(), "Token mismatch"
        assert obligation.arbiter.lower() == decoded_obligation.arbiter.lower(), "Arbiter mismatch"
        assert obligation.demand == decoded_obligation.demand, "Demand mismatch"
        print("✅ test_basic_encode_decode PASSED")
        return True
        
    except Exception as e:
        print(f"Test failed: {e}")
        return False


async def main():
    success = await test_basic_encode_decode()
    return 0 if success else 1


if __name__ == "__main__":
    exit_code = asyncio.run(main())
    exit(exit_code)