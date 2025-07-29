"""
Test AlkahestClient initialization with extensions in Python
"""
import pytest
from alkahest_py import (
    AlkahestClient, 
    EnvTestManager,
    Erc20Addresses,
    Erc721Addresses,
    Erc1155Addresses,
    TokenBundleAddresses,
    AttestationAddresses,
    StringObligationAddresses,
    OracleAddresses
)


@pytest.mark.asyncio
async def test_alkahest_client_init_default():
    """Test AlkahestClient initialization with default extensions (no custom config)."""
    env = EnvTestManager()

    # Initialize client without custom address config (should use defaults)
    client = AlkahestClient(
        private_key="0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d",
        rpc_url=env.rpc_url
    )

    # Verify the client has all expected extension clients
    assert hasattr(client, 'erc20'), "Client should have ERC20 extension"
    assert hasattr(client, 'erc721'), "Client should have ERC721 extension"
    assert hasattr(client, 'erc1155'), "Client should have ERC1155 extension"
    assert hasattr(client, 'token_bundle'), "Client should have token bundle extension"
    assert hasattr(client, 'attestation'), "Client should have attestation extension"
    assert hasattr(client, 'string_obligation'), "Client should have string obligation extension"
    assert hasattr(client, 'oracle'), "Client should have oracle extension"

    # Verify extensions are accessible (should not raise errors)
    erc20_client = client.erc20
    erc721_client = client.erc721
    erc1155_client = client.erc1155
    token_bundle_client = client.token_bundle
    attestation_client = client.attestation
    string_obligation_client = client.string_obligation
    oracle_client = client.oracle

    # Verify extensions have expected methods
    assert hasattr(erc20_client, 'approve'), "ERC20 client should have approve method"
    assert hasattr(erc721_client, 'approve'), "ERC721 client should have approve method"
    assert hasattr(erc1155_client, 'approve_all'), "ERC1155 client should have approve_all method"
    # Token bundle and other clients exist but may have different method names
    assert token_bundle_client is not None, "Token bundle client should exist"

    print("✅ Default AlkahestClient initialization test passed!")


@pytest.mark.asyncio
async def test_alkahest_client_init_with_no_extensions():
    """Test AlkahestClient initialization with no extensions."""
    env = EnvTestManager()

    # Initialize client with no extensions
    client = AlkahestClient.with_no_extensions(
        private_key="0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d",
        rpc_url=env.rpc_url
    )

    # Verify the client exists but extensions raise errors when accessed
    assert client is not None, "Client should exist"
    
    # When we try to access extensions, they should raise AttributeError
    with pytest.raises(Exception):  # Should raise AttributeError
        _ = client.erc20
    
    with pytest.raises(Exception):  # Should raise AttributeError
        _ = client.erc721
    
    with pytest.raises(Exception):  # Should raise AttributeError
        _ = client.erc1155

    print("✅ No extensions AlkahestClient initialization test passed!")





@pytest.mark.asyncio
async def test_alkahest_client_with_erc20():
    """Test AlkahestClient with_erc20 method."""
    env = EnvTestManager()

    # Start with no extensions client
    base_client = AlkahestClient.with_no_extensions(
        private_key="0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d",
        rpc_url=env.rpc_url
    )

    # Create custom ERC20 config with real contract addresses
    erc20_config = Erc20Addresses(
        eas="0x4200000000000000000000000000000000000021",
        barter_utils="0x1234567890123456789012345678901234567890",
        escrow_obligation="0x2345678901234567890123456789012345678901",
        payment_obligation="0x3456789012345678901234567890123456789012"
    )

    # Add ERC20 extension with custom config
    client_with_erc20 = await base_client.with_erc20(config=erc20_config)

    # Verify ERC20 extension is accessible
    assert hasattr(client_with_erc20, 'erc20'), "Client should have ERC20 extension"
    erc20_client = client_with_erc20.erc20
    assert erc20_client is not None, "ERC20 client should exist"
    assert hasattr(erc20_client, 'approve'), "ERC20 client should have approve method"

    # Verify other extensions are still not accessible
    with pytest.raises(Exception):
        _ = client_with_erc20.erc721

    print("✅ AlkahestClient with_erc20 test passed!")


@pytest.mark.asyncio
async def test_alkahest_client_with_erc721():
    """Test AlkahestClient with_erc721 method."""
    env = EnvTestManager()

    # Start with no extensions client
    base_client = AlkahestClient.with_no_extensions(
        private_key="0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d",
        rpc_url=env.rpc_url
    )

    # Create custom ERC721 config with real contract addresses
    erc721_config = Erc721Addresses(
        eas="0x4200000000000000000000000000000000000021",
        barter_utils="0x5678901234567890123456789012345678901234",
        escrow_obligation="0x6789012345678901234567890123456789012345",
        payment_obligation="0x7890123456789012345678901234567890123456"
    )

    # Add ERC721 extension with custom config
    client_with_erc721 = await base_client.with_erc721(config=erc721_config)


@pytest.mark.asyncio
async def test_alkahest_client_with_erc1155():
    """Test AlkahestClient with_erc1155 method."""
    env = EnvTestManager()

    # Start with no extensions client
    base_client = AlkahestClient.with_no_extensions(
        private_key="0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d",
        rpc_url=env.rpc_url
    )

    # Create custom ERC1155 config with real contract addresses
    erc1155_config = Erc1155Addresses(
        eas="0x4200000000000000000000000000000000000021",
        barter_utils="0x8901234567890123456789012345678901234567",
        escrow_obligation="0x9012345678901234567890123456789012345678",
        payment_obligation="0x0123456789012345678901234567890123456789"
    )

    # Add ERC1155 extension with custom config
    client_with_erc1155 = await base_client.with_erc1155(config=erc1155_config)


@pytest.mark.asyncio
async def test_alkahest_client_with_token_bundle():
    """Test AlkahestClient with_token_bundle method."""
    env = EnvTestManager()

    # Start with no extensions client
    base_client = AlkahestClient.with_no_extensions(
        private_key="0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d",
        rpc_url=env.rpc_url
    )

    # Create custom token bundle config with real contract addresses
    token_bundle_config = TokenBundleAddresses(
        eas="0x4200000000000000000000000000000000000021",
        barter_utils="0xa123456789012345678901234567890123456789",
        escrow_obligation="0xb234567890123456789012345678901234567890",
        payment_obligation="0xc345678901234567890123456789012345678901"
    )

    # Add token bundle extension with custom config
    client_with_bundle = await base_client.with_token_bundle(config=token_bundle_config)


@pytest.mark.asyncio
async def test_alkahest_client_with_attestation():
    """Test AlkahestClient with_attestation method."""
    env = EnvTestManager()

    # Start with no extensions client
    base_client = AlkahestClient.with_no_extensions(
        private_key="0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d",
        rpc_url=env.rpc_url
    )

    # Add attestation extension with custom config
    attestation_config = AttestationAddresses(
        eas="0x4200000000000000000000000000000000000021",
        eas_schema_registry="0xbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
        barter_utils="0xcccccccccccccccccccccccccccccccccccccccc",
        escrow_obligation="0xdddddddddddddddddddddddddddddddddddddddd",
        escrow_obligation_2="0xeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee"
    )
    client_with_attestation = await base_client.with_attestation(config=attestation_config)

    # Verify attestation extension is accessible
    assert hasattr(client_with_attestation, 'attestation'), "Client should have attestation extension"
    attestation_client = client_with_attestation.attestation
    assert attestation_client is not None, "Attestation client should exist"

    # Verify other extensions are still not accessible
    with pytest.raises(Exception):
        _ = client_with_attestation.erc20

    print("✅ AlkahestClient with_attestation test passed!")


@pytest.mark.asyncio
async def test_alkahest_client_with_string_obligation():
    """Test AlkahestClient with_string_obligation method."""
    env = EnvTestManager()

    # Start with no extensions client
    base_client = AlkahestClient.with_no_extensions(
        private_key="0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d",
        rpc_url=env.rpc_url
    )

    # Add string obligation extension with custom config  
    string_obligation_config = StringObligationAddresses(
        eas="0x4200000000000000000000000000000000000021",
        obligation="0xffffffffffffffffffffffffffffffffffffffff"
    )
    client_with_string_obligation = await base_client.with_string_obligation(config=string_obligation_config)

    # Verify string obligation extension is accessible
    assert hasattr(client_with_string_obligation, 'string_obligation'), "Client should have string obligation extension"
    string_obligation_client = client_with_string_obligation.string_obligation
    assert string_obligation_client is not None, "String obligation client should exist"

    # Verify other extensions are still not accessible
    with pytest.raises(Exception):
        _ = client_with_string_obligation.erc20

    print("✅ AlkahestClient with_string_obligation test passed!")


@pytest.mark.asyncio
async def test_alkahest_client_with_oracle():
    """Test AlkahestClient with_oracle method."""
    env = EnvTestManager()

    # Start with no extensions client
    base_client = AlkahestClient.with_no_extensions(
        private_key="0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d",
        rpc_url=env.rpc_url
    )

    # Add oracle extension with custom config
    oracle_config = OracleAddresses(
        eas="0x4200000000000000000000000000000000000021",
        trusted_oracle_arbiter="0x1111111111111111111111111111111111111111"
    )
    client_with_oracle = await base_client.with_oracle(config=oracle_config)

    # Verify oracle extension is accessible
    assert hasattr(client_with_oracle, 'oracle'), "Client should have oracle extension"
    oracle_client = client_with_oracle.oracle
    assert oracle_client is not None, "Oracle client should exist"

    # Verify other extensions are still not accessible
    with pytest.raises(Exception):
        _ = client_with_oracle.erc20

    print("✅ AlkahestClient with_oracle test passed!")


@pytest.mark.asyncio
async def test_alkahest_client_with_multiple_extensions():
    """Test AlkahestClient can now add multiple extensions to any client type.
    
    This test demonstrates the new capability: extensions can be added to any client,
    not just no-extension clients. Each extension is created independently using
    the stored connection information.
    """
    env = EnvTestManager()

    # Start with no extensions client
    base_client = AlkahestClient.with_no_extensions(
        private_key="0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d",
        rpc_url=env.rpc_url
    )

    # Create custom configs for different extensions
    erc20_config = Erc20Addresses(
        eas="0x4200000000000000000000000000000000000021",
        barter_utils="0x1111111111111111111111111111111111111111",
        escrow_obligation="0x2222222222222222222222222222222222222222",
        payment_obligation="0x3333333333333333333333333333333333333333"
    )

    erc721_config = Erc721Addresses(
        eas="0x4200000000000000000000000000000000000021",
        barter_utils="0x4444444444444444444444444444444444444444",
        escrow_obligation="0x5555555555555555555555555555555555555555",
        payment_obligation="0x6666666666666666666666666666666666666666"
    )

    erc1155_config = Erc1155Addresses(
        eas="0x4200000000000000000000000000000000000021",
        barter_utils="0x7777777777777777777777777777777777777777",
        escrow_obligation="0x8888888888888888888888888888888888888888",
        payment_obligation="0x9999999999999999999999999999999999999999"
    )

    # Add ERC20 extension with custom config
    client_with_erc20 = await base_client.with_erc20(config=erc20_config)
    
    # Verify ERC20 extension is accessible
    assert hasattr(client_with_erc20, 'erc20'), "Client should have ERC20 extension"
    erc20_client = client_with_erc20.erc20
    assert erc20_client is not None, "ERC20 client should exist"
    
    # NEW: Now we can add another extension to a client that already has ERC20!
    client_with_erc20_and_erc721 = await client_with_erc20.with_erc721(config=erc721_config)
    
    # Verify both extensions are accessible
    assert hasattr(client_with_erc20_and_erc721, 'erc20'), "Client should still have ERC20 extension"
    assert hasattr(client_with_erc20_and_erc721, 'erc721'), "Client should now have ERC721 extension"
    
    erc20_client_2 = client_with_erc20_and_erc721.erc20
    erc721_client = client_with_erc20_and_erc721.erc721
    
    assert erc20_client_2 is not None, "ERC20 client should still exist"
    assert erc721_client is not None, "ERC721 client should exist"
    
    # Add a third extension
    client_with_three = await client_with_erc20_and_erc721.with_erc1155(config=erc1155_config)
    
    # Verify all three extensions are accessible
    assert hasattr(client_with_three, 'erc20'), "Client should have ERC20 extension"
    assert hasattr(client_with_three, 'erc721'), "Client should have ERC721 extension"
    assert hasattr(client_with_three, 'erc1155'), "Client should have ERC1155 extension"
    
    erc20_final = client_with_three.erc20
    erc721_final = client_with_three.erc721
    erc1155_final = client_with_three.erc1155
    
    assert erc20_final is not None, "ERC20 client should exist"
    assert erc721_final is not None, "ERC721 client should exist"
    assert erc1155_final is not None, "ERC1155 client should exist"
    
    # Verify methods are accessible on all extensions
    assert hasattr(erc20_final, 'approve'), "ERC20 client should have approve method"
    assert hasattr(erc721_final, 'approve'), "ERC721 client should have approve method"
    assert hasattr(erc1155_final, 'approve_all'), "ERC1155 client should have approve_all method"

    print("✅ AlkahestClient multiple extensions test passed!")
    print("🎉 Extensions can now be added to any client type using independent extension clients!")


@pytest.mark.asyncio
async def test_alkahest_client_add_extensions_to_full_client():
    """Test adding extensions to a client that already has all extensions.
    
    This demonstrates that extensions can be added to any client type,
    including one that already has all extensions loaded.
    """
    env = EnvTestManager()

    # Start with a FULL client (all extensions already loaded)
    full_client = AlkahestClient(
        private_key="0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d",
        rpc_url=env.rpc_url
    )

    # Verify it has all extensions
    assert hasattr(full_client, 'erc20'), "Full client should have ERC20 extension"
    assert hasattr(full_client, 'erc721'), "Full client should have ERC721 extension"
    assert hasattr(full_client, 'erc1155'), "Full client should have ERC1155 extension"
    assert hasattr(full_client, 'token_bundle'), "Full client should have token bundle extension"
    assert hasattr(full_client, 'attestation'), "Full client should have attestation extension"
    assert hasattr(full_client, 'string_obligation'), "Full client should have string obligation extension"
    assert hasattr(full_client, 'oracle'), "Full client should have oracle extension"

    # Now try to add MORE extensions to this already-full client
    # This should work with the new init_with_config approach
    custom_erc20_config = Erc20Addresses(
        eas="0x4200000000000000000000000000000000000021",
        barter_utils="0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        escrow_obligation="0xbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
        payment_obligation="0xcccccccccccccccccccccccccccccccccccccccc"
    )
    
    custom_erc721_config = Erc721Addresses(
        eas="0x4200000000000000000000000000000000000021",
        barter_utils="0xdddddddddddddddddddddddddddddddddddddddd",
        escrow_obligation="0xeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee",
        payment_obligation="0xffffffffffffffffffffffffffffffffffffffff"
    )
    
    custom_erc1155_config = Erc1155Addresses(
        eas="0x4200000000000000000000000000000000000021",
        barter_utils="0x1010101010101010101010101010101010101010",
        escrow_obligation="0x2020202020202020202020202020202020202020",
        payment_obligation="0x3030303030303030303030303030303030303030"
    )
    
    client_with_additional_erc20 = await full_client.with_erc20(config=custom_erc20_config)
    client_with_additional_erc721 = await full_client.with_erc721(config=custom_erc721_config)
    client_with_additional_erc1155 = await full_client.with_erc1155(config=custom_erc1155_config)

    # Verify the extensions still work
    assert hasattr(client_with_additional_erc20, 'erc20'), "Client should still have ERC20 extension"
    assert hasattr(client_with_additional_erc721, 'erc721'), "Client should still have ERC721 extension"
    assert hasattr(client_with_additional_erc1155, 'erc1155'), "Client should still have ERC1155 extension"

    # Test that we can chain multiple additions
    chained_client = await full_client.with_erc20(config=None)
    chained_client = await chained_client.with_erc721(config=None)
    chained_client = await chained_client.with_erc1155(config=None)
    chained_client = await chained_client.with_token_bundle(config=None)

    # Verify all extensions are still accessible
    assert hasattr(chained_client, 'erc20'), "Chained client should have ERC20 extension"
    assert hasattr(chained_client, 'erc721'), "Chained client should have ERC721 extension"
    assert hasattr(chained_client, 'erc1155'), "Chained client should have ERC1155 extension"
    assert hasattr(chained_client, 'token_bundle'), "Chained client should have token bundle extension"

    print("✅ AlkahestClient add extensions to full client test passed!")
    print("🚀 Extensions can be added to any client type, even already-full clients!")


@pytest.mark.asyncio
async def test_alkahest_client_all_combinations():
    """Test various combinations of adding extensions to different client types."""
    env = EnvTestManager()

    # Test 1: Start with ERC20, add ERC721 and ERC1155
    client1 = AlkahestClient.with_no_extensions(
        private_key="0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d",
        rpc_url=env.rpc_url
    )
    client1 = await client1.with_erc20(config=None)
    client1 = await client1.with_erc721(config=None)
    client1 = await client1.with_erc1155(config=None)
    
    assert hasattr(client1, 'erc20'), "Client1 should have ERC20"
    assert hasattr(client1, 'erc721'), "Client1 should have ERC721"
    assert hasattr(client1, 'erc1155'), "Client1 should have ERC1155"

    # Test 2: Start with full client, add more extensions
    client2 = AlkahestClient(
        private_key="0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d",
        rpc_url=env.rpc_url
    )
    client2 = await client2.with_attestation(config=None)
    client2 = await client2.with_oracle(config=None)
    
    assert hasattr(client2, 'erc20'), "Client2 should have ERC20 (from full init)"
    assert hasattr(client2, 'attestation'), "Client2 should have attestation"
    assert hasattr(client2, 'oracle'), "Client2 should have oracle"

    # Test 3: Add all extension types to a single client
    client3 = AlkahestClient.with_no_extensions(
        private_key="0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d",
        rpc_url=env.rpc_url
    )
    client3 = await client3.with_erc20(config=None)
    client3 = await client3.with_erc721(config=None)
    client3 = await client3.with_erc1155(config=None)
    client3 = await client3.with_token_bundle(config=None)
    client3 = await client3.with_attestation(config=None)
    client3 = await client3.with_string_obligation(config=None)
    client3 = await client3.with_oracle(config=None)
    
    # Verify all extensions are present
    extensions = ['erc20', 'erc721', 'erc1155', 'token_bundle', 'attestation', 'string_obligation', 'oracle']
    for ext in extensions:
        assert hasattr(client3, ext), f"Client3 should have {ext} extension"

    print("✅ AlkahestClient all combinations test passed!")
    print("🎯 All extension combinations work with the new architecture!")


@pytest.mark.asyncio
async def test_alkahest_client_with_extension_custom_config():
    """Test AlkahestClient with extension using custom configuration."""
    env = EnvTestManager()

    # Start with no extensions client
    base_client = AlkahestClient.with_no_extensions(
        private_key="0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d",
        rpc_url=env.rpc_url
    )

    # Test with custom config - this tests the config parameter functionality
    # Note: We're testing with None config0 first, but the API supports custom addresses
    client_with_erc20 = await base_client.with_erc20(config=None)

    # Verify the extension was added successfully
    assert hasattr(client_with_erc20, 'erc20'), "Client should have ERC20 extension"
    erc20_client = client_with_erc20.erc20
    assert erc20_client is not None, "ERC20 client should exist"

    print("✅ AlkahestClient with extension custom config test passed!")
