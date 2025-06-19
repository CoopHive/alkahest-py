"""
Single comprehensive test for String Obligation with PyDecodedAttestation<T>
"""
import asyncio
from alkahest_py import (
    PyTestEnvManager,
    PyStringObligationStatementData,
)


async def test_string_obligation():
    try:
        env = PyTestEnvManager()
        string_client = env.alice_client.string_obligation
        
        statement_data = PyStringObligationStatementData(item="Test statement for PyDecodedAttestation<T>")
        tx_hash = await string_client.make_statement(statement_data, None)
        
        # Verify transaction hash format
        assert tx_hash.startswith('0x') and len(tx_hash) == 66
        
        print("✅ test_string_obligation PASSED")
        return True
        
    except Exception as e:
        print(f"❌ test_string_obligation FAILED: {e}")
        return False


if __name__ == "__main__":
    success = asyncio.run(test_string_obligation())
    exit(0 if success else 1)
