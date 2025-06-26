"""
Single comprehensive test for String Obligation with PyDecodedAttestation<T>
"""
import pytest
from alkahest_py import (
    PyTestEnvManager,
    PyStringObligationStatementData,
)

@pytest.mark.asyncio
async def test_string_obligation():
    env = PyTestEnvManager()
    string_client = env.alice_client.string_obligation
    
    statement_data = PyStringObligationStatementData(item="Test statement for PyDecodedAttestation<T>")
    tx_hash = await string_client.make_statement(statement_data, None)
    
    # Verify transaction hash format
    assert tx_hash.startswith('0x') and len(tx_hash) == 66
