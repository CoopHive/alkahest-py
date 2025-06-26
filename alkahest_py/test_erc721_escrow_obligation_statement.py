import pytest
from alkahest_py import PyTestEnvManager, PyERC721EscrowObligationStatement

@pytest.mark.asyncio
async def test_basic_encode_decode():
    env = PyTestEnvManager()
    
    obligation = PyERC721EscrowObligationStatement(
    token=env.mock_addresses.erc721_a,
    token_id="12345",
    arbiter=env.addresses.erc721_addresses.escrow_obligation,
    demand=[1, 2, 3, 4, 5]
    )
    
    encoded_data = PyERC721EscrowObligationStatement.encode(obligation)
    decoded_obligation = PyERC721EscrowObligationStatement.decode(encoded_data)

    assert obligation.token_id == decoded_obligation.token_id, "Token ID mismatch"
    assert obligation.token.lower() == decoded_obligation.token.lower(), "Token mismatch"
    assert obligation.arbiter.lower() == decoded_obligation.arbiter.lower(), "Arbiter mismatch"
    assert obligation.demand == decoded_obligation.demand, "Demand mismatch"
