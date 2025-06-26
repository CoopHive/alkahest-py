"""
Single comprehensive test for String Obligation with PyDecodedAttestation<T>
"""
import pytest
from alkahest_py import (
    EnvTestManager,
    StringObligationStatementData,
)

@pytest.mark.asyncio
async def test_string_obligation():
    env = EnvTestManager()
    string_client = env.alice_client.string_obligation
    
    statement_data = StringObligationStatementData(item="Test statement for PyDecodedAttestation<T>")
    tx_hash = string_client.make_statement(statement_data, None)
    
    # Verify transaction hash format
    assert tx_hash.startswith('0x') and len(tx_hash) == 66

@pytest.mark.asyncio
async def test_make_statement_json():
    """
    Test the make_statement_json function with various Python objects
    """
    env = EnvTestManager()
    string_client = env.alice_client.string_obligation
    
    # Test with dictionary
    dict_data = {
        "message": "Hello from dictionary",
        "number": 42,
        "nested": {
            "key": "value",
            "array": [1, 2, 3]
        }
    }
    
    tx_hash_dict = string_client.make_statement_json(dict_data, None)
    assert tx_hash_dict.startswith('0x') and len(tx_hash_dict) == 66
    print(f"✅ Dictionary test passed: {tx_hash_dict}")
    
    # Test with list
    list_data = ["item1", "item2", {"nested": "object"}, 123, True]
    
    tx_hash_list = string_client.make_statement_json(list_data, None)
    assert tx_hash_list.startswith('0x') and len(tx_hash_list) == 66
    print(f"✅ List test passed: {tx_hash_list}")
    
    # Test with simple string
    string_data = "Simple string message"
    
    tx_hash_string = string_client.make_statement_json(string_data, None)
    assert tx_hash_string.startswith('0x') and len(tx_hash_string) == 66
    print(f"✅ String test passed: {tx_hash_string}")
    
    # Test with number
    number_data = 12345
    
    tx_hash_number = string_client.make_statement_json(number_data, None)
    assert tx_hash_number.startswith('0x') and len(tx_hash_number) == 66
    print(f"✅ Number test passed: {tx_hash_number}")
    
    # Test with boolean
    bool_data = True
    
    tx_hash_bool = string_client.make_statement_json(bool_data, None)
    assert tx_hash_bool.startswith('0x') and len(tx_hash_bool) == 66
    print(f"✅ Boolean test passed: {tx_hash_bool}")
    
    # Test with None
    none_data = None
    
    tx_hash_none = string_client.make_statement_json(none_data, None)
    assert tx_hash_none.startswith('0x') and len(tx_hash_none) == 66
    print(f"✅ None test passed: {tx_hash_none}")
    
    # Test with complex nested structure
    complex_data = {
        "user": {
            "id": 123,
            "name": "Alice",
            "active": True,
            "preferences": {
                "theme": "dark",
                "notifications": True,
                "languages": ["en", "es", "fr"]
            }
        },
        "timestamp": "2025-06-26T12:00:00Z",
        "metadata": {
            "version": "1.0",
            "source": "test",
            "tags": ["test", "json", "alkahest"]
        }
    }
    
    tx_hash_complex = string_client.make_statement_json(complex_data, None)
    assert tx_hash_complex.startswith('0x') and len(tx_hash_complex) == 66
    print(f"✅ Complex nested structure test passed: {tx_hash_complex}")
    
    # All transaction hashes should be different
    all_hashes = [tx_hash_dict, tx_hash_list, tx_hash_string, tx_hash_number, 
                  tx_hash_bool, tx_hash_none, tx_hash_complex]
    assert len(set(all_hashes)) == len(all_hashes), "All transaction hashes should be unique"
    
    print("✅ All make_statement_json tests passed successfully!")

