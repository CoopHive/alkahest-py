
import asyncio
from alkahest_py import PyTestEnvManager, PyERC20EscrowObligationStatement


async def test_basic_encode_decode():
    print("🚀 Testing PyERC20EscrowObligationStatement basic functionality...")
    
    try:
        # Setup test environment
        env = PyTestEnvManager()
        
        # Create test obligation
        obligation = PyERC20EscrowObligationStatement(
            token=env.mock_addresses.erc20_a,
            amount=100,
            arbiter=env.addresses.erc20_addresses.payment_obligation,
            demand=[1, 2, 3, 4, 5]
        )
        print(f"✅ Created obligation: {repr(obligation)}")
        
        print(f"✅ Created obligation successfully")
        
        # Test encoding
        encoded_data = PyERC20EscrowObligationStatement.encode(obligation)
        print(f"✅ Encoding successful - Generated {len(encoded_data)} bytes")
        
        # Test decoding
        decoded_obligation = PyERC20EscrowObligationStatement.decode(encoded_data)
        print(f"✅ Decoding successful")
        print(f"Decoded obligation: {repr(decoded_obligation)}")

        # Basic verification
        assert obligation.amount == decoded_obligation.amount, "Amount mismatch"
        assert obligation.token.lower() == decoded_obligation.token.lower(), "Token mismatch"
        assert obligation.arbiter.lower() == decoded_obligation.arbiter.lower(), "Arbiter mismatch"
        assert obligation.demand == decoded_obligation.demand, "Demand mismatch"
        
        print("✅ Round-trip verification successful")
        return True
        
    except Exception as e:
        print(f"❌ Test failed: {e}")
        return False


async def main():
    success = await test_basic_encode_decode()
    
    print("\n" + "=" * 50)
    if success:
        print("🎉 TEST PASSED! Python SDK is working correctly.")
    else:
        print("💥 TEST FAILED! Check the error above.")
    print("=" * 50)


if __name__ == "__main__":
    asyncio.run(main())