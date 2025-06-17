import asyncio
from alkahest_py import PyTestEnvManager, PyERC1155PaymentObligationStatement


async def test_basic_encode_decode():
    try:
        env = PyTestEnvManager()
        
        obligation = PyERC1155PaymentObligationStatement(
            token=env.mock_addresses.erc1155_a,
            token_id="98765",
            amount="500",
            payee=env.addresses.erc1155_addresses.payment_obligation
        )
        
        # Test encoding
        encoded_data = PyERC1155PaymentObligationStatement.encode(obligation)
        
        # Verify encoded data is bytes
        assert isinstance(encoded_data, bytes), "Encoded data should be bytes"
        assert len(encoded_data) > 0, "Encoded data should have content"
        
        # Test decoding
        decoded_obligation = PyERC1155PaymentObligationStatement.decode(encoded_data)
        
        assert obligation.token_id == decoded_obligation.token_id, "Token ID mismatch"
        assert obligation.amount == decoded_obligation.amount, "Amount mismatch"
        assert obligation.token.lower() == decoded_obligation.token.lower(), "Token mismatch"
        assert obligation.payee.lower() == decoded_obligation.payee.lower(), "Payee mismatch"
        
        # Test __repr__ method
        repr_str = repr(obligation)
        assert "PyERC1155PaymentObligationStatement" in repr_str, "Repr should contain class name"
        assert obligation.token in repr_str, "Repr should contain token address"
        assert obligation.token_id in repr_str, "Repr should contain token ID"
        assert obligation.amount in repr_str, "Repr should contain amount"
        
        print("✅ ERC1155 Payment Obligation Statement encode/decode test passed!")
        print(f"Original: {obligation}")
        print(f"Decoded: {decoded_obligation}")
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
