"""
Test IEAS (Ethereum Attestation Service) Python bindings
"""
import pytest
from alkahest_py import (
    PyAttestation,
    PyAttestationRequest, 
    PyAttestationRequestData,
    PyAttested,
    PyRevocationRequest,
    PyRevocationRequestData,
    PyRevoked,
    PyTimestamped,
)

@pytest.mark.asyncio
async def test_ieas_types():
    """Test IEAS type creation and basic functionality"""
    print("🧪 Testing IEAS Python bindings...")
    
    # Test PyAttestationRequestData
    request_data = PyAttestationRequestData(
        recipient="0x1234567890123456789012345678901234567890",
        expiration_time=1735689600,  # Future timestamp
        revocable=True,
        ref_uid="0x0000000000000000000000000000000000000000000000000000000000000000",
        data=b"Hello, Attestation!",
        value=0
    )
    print(f"✅ PyAttestationRequestData: {request_data}")
    
    # Test PyAttestationRequest
    request = PyAttestationRequest(
        schema="0x1234567890123456789012345678901234567890123456789012345678901234",
        data=request_data
    )
    print(f"✅ PyAttestationRequest: {request}")
    
    # Test PyAttestation
    attestation = PyAttestation(
        uid="0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890",
        schema="0x1234567890123456789012345678901234567890123456789012345678901234",
        time=1735603200,  # Current timestamp
        expiration_time=1735689600,  # Future timestamp
        revocation_time=0,  # Not revoked
        ref_uid="0x0000000000000000000000000000000000000000000000000000000000000000",
        recipient="0x1234567890123456789012345678901234567890",
        attester="0xabcdef1234567890abcdef1234567890abcdef12",
        revocable=True,
        data=b"Hello, Attestation!"
    )
    print(f"✅ PyAttestation: {attestation}")
    
    # Test attestation validation methods
    print(f"✅ Attestation is valid: {attestation.is_valid()}")
    print(f"✅ Attestation is expired: {attestation.is_expired()}")
    print(f"✅ Attestation is revoked: {attestation.is_revoked()}")
    
    # Test PyRevocationRequestData
    revocation_data = PyRevocationRequestData(
        uid="0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890",
        value=0
    )
    print(f"✅ PyRevocationRequestData: {revocation_data}")
    
    # Test PyRevocationRequest
    revocation_request = PyRevocationRequest(
        schema="0x1234567890123456789012345678901234567890123456789012345678901234",
        data=revocation_data
    )
    print(f"✅ PyRevocationRequest: {revocation_request}")
    
    # Test event types
    attested_event = PyAttested(
        recipient="0x1234567890123456789012345678901234567890",
        attester="0xabcdef1234567890abcdef1234567890abcdef12",
        uid="0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890",
        schema_uid="0x1234567890123456789012345678901234567890123456789012345678901234"
    )
    print(f"✅ PyAttested: {attested_event}")
    
    revoked_event = PyRevoked(
        recipient="0x1234567890123456789012345678901234567890",
        attester="0xabcdef1234567890abcdef1234567890abcdef12",
        uid="0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890",
        schema_uid="0x1234567890123456789012345678901234567890123456789012345678901234"
    )
    print(f"✅ PyRevoked: {revoked_event}")
    
    timestamped_event = PyTimestamped(
        data=b"Timestamped data",
        timestamp=1735603200
    )
    print(f"✅ PyTimestamped: {timestamped_event}")
    
    print("🎉 All IEAS types created successfully!")
