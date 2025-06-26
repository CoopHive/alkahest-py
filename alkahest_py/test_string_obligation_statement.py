import pytest
from alkahest_py import EnvTestManager, StringObligationStatementData

@pytest.mark.asyncio
async def test_basic_encode_decode():
    env = EnvTestManager()
    
    obligation = StringObligationStatementData(
        item="test string obligation data"
    )
    
    encoded_data = StringObligationStatementData.encode(obligation)
    decoded_obligation = StringObligationStatementData.decode(encoded_data)

    assert obligation.item == decoded_obligation.item, "Item mismatch"
