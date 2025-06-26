import pytest
from alkahest_py import EnvTestManager, ERC721PaymentObligationStatement

@pytest.mark.asyncio
async def test_basic_encode_decode():
    env = EnvTestManager()
    
    obligation = ERC721PaymentObligationStatement(
    token=env.mock_addresses.erc721_a,
    token_id="67890",
    payee=env.addresses.erc721_addresses.payment_obligation
    )
    
    # Test encoding
    encoded_data = ERC721PaymentObligationStatement.encode(obligation)
    
    # Verify encoded data is bytes
    assert isinstance(encoded_data, bytes), "Encoded data should be bytes"
    assert len(encoded_data) > 0, "Encoded data should have content"
    
    # Test decode functionality
    decoded_obligation = ERC721PaymentObligationStatement.decode(encoded_data)
    
    # Verify decoded data matches original
    assert obligation.token_id == decoded_obligation.token_id, "Token ID mismatch"
    assert obligation.token.lower() == decoded_obligation.token.lower(), "Token mismatch"
    assert obligation.payee.lower() == decoded_obligation.payee.lower(), "Payee mismatch"
    
    # Test __repr__ method
    repr_str = repr(obligation)
    assert "ERC721PaymentObligationStatement" in repr_str, "Repr should contain class name"
    assert obligation.token in repr_str, "Repr should contain token address"
    assert obligation.token_id in repr_str, "Repr should contain token ID"
