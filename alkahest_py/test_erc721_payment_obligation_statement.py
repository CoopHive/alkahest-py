import asyncio
from alkahest_py import PyTestEnvManager, PyERC721PaymentObligationStatement


async def test_basic_encode_decode():
    try:
        env = PyTestEnvManager()
        
        obligation = PyERC721PaymentObligationStatement(
            token=env.mock_addresses.erc721_a,
            token_id="67890",
            payee=env.addresses.erc721_addresses.payment_obligation
        )
        
        # Test encoding
        encoded_data = PyERC721PaymentObligationStatement.encode(obligation)
        
        # Verify encoded data is bytes
        assert isinstance(encoded_data, bytes), "Encoded data should be bytes"
        assert len(encoded_data) > 0, "Encoded data should have content"
        
        # Test decode functionality
        decoded_obligation = PyERC721PaymentObligationStatement.decode(encoded_data)
        
        # Verify decoded data matches original
        assert obligation.token_id == decoded_obligation.token_id, "Token ID mismatch"
        assert obligation.token.lower() == decoded_obligation.token.lower(), "Token mismatch"
        assert obligation.payee.lower() == decoded_obligation.payee.lower(), "Payee mismatch"
        
        # Test __repr__ method
        repr_str = repr(obligation)
        assert "PyERC721PaymentObligationStatement" in repr_str, "Repr should contain class name"
        assert obligation.token in repr_str, "Repr should contain token address"
        assert obligation.token_id in repr_str, "Repr should contain token ID"
        
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
