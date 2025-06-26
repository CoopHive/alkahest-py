import pytest
from alkahest_py import PyTestEnvManager, PyStringObligationStatementData

@pytest.mark.asyncio
async def test_basic_encode_decode():
    env = PyTestEnvManager()
    
    obligation = PyStringObligationStatementData(
        item="test string obligation data"
    )
    
    encoded_data = PyStringObligationStatementData.encode(obligation)
    decoded_obligation = PyStringObligationStatementData.decode(encoded_data)

    assert obligation.item == decoded_obligation.item, "Item mismatch"
