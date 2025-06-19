import asyncio
from alkahest_py import PyTestEnvManager, PyStringObligationStatementData


async def test_basic_encode_decode():
    try:
        env = PyTestEnvManager()
        
        obligation = PyStringObligationStatementData(
            item="Hello, String Obligation World!"
        )
        
        encoded_data = PyStringObligationStatementData.encode(obligation)
        decoded_obligation = PyStringObligationStatementData.decode(encoded_data)

        assert obligation.item == decoded_obligation.item, "Item mismatch"
        
        print("✅ String Obligation Statement encode/decode test passed!")
        
    except Exception as e:
        print(f"❌ Test failed: {e}")
        raise


if __name__ == "__main__":
    asyncio.run(test_basic_encode_decode())
