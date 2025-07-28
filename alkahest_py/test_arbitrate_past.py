#!/usr/bin/env python3
"""
Test the Oracle arbitrate_past_sync functionality with complete escrow, arbitration, and payment flow
"""

import pytest
import time
from alkahest_py import (
    EnvTestManager,
    StringObligationData,
    AttestationFilter,
    FulfillmentParams,
    ArbitrateOptions,
    MockERC20,
    TrustedOracleArbiterDemandData,
)

@pytest.mark.asyncio
async def test_arbitrate_past_sync():
    """Test complete arbitrate_past_sync flow: escrow → fulfillment → arbitration → payment collection"""
    # Setup test environment
    env = EnvTestManager()
    
    # Setup escrow with proper oracle demand data
    mock_erc20 = MockERC20(env.mock_addresses.erc20_a, env.god_wallet_provider)
    mock_erc20.transfer(env.alice, 100)
    
    price = {"address": env.mock_addresses.erc20_a, "value": 100}
    trusted_oracle_arbiter = env.addresses.arbiters_addresses.trusted_oracle_arbiter
    
    # Create proper demand data with Bob as the oracle
    oracle_client = env.bob_client.oracle
    demand_data = TrustedOracleArbiterDemandData(env.bob, [])
    demand_bytes = demand_data.encode_self()
    
    arbiter = {
        "arbiter": trusted_oracle_arbiter,
        "demand": demand_bytes
    }
    
    expiration = int(time.time()) + 3600
    escrow_receipt = await env.alice_client.erc20.permit_and_buy_with_erc20(
        price, arbiter, expiration
    )
    escrow_uid = escrow_receipt['log']['uid']
    
    # Make fulfillment obligation
    string_client = env.bob_client.string_obligation
    fulfillment_uid = await string_client.do_obligation("good", escrow_uid)

    # Create filter and fulfillment params
    filter_obj = AttestationFilter(
        attester=env.addresses.string_obligation_addresses.obligation,
        recipient=env.bob,
        schema_uid=None,
        uid=None,
        ref_uid=escrow_uid,
        from_block=0,
        to_block=None,
    )
    
    obligation_abi = StringObligationData(item="")
    fulfillment_params = FulfillmentParams(
        obligation_abi=obligation_abi,
        filter=filter_obj
    )
    
    options = ArbitrateOptions(
        require_oracle=True,
        skip_arbitrated=False,
        require_request=False,
        only_new=False
    )
    
    # Decision function that approves "good" obligations
    def decision_function(obligation_str):
        print(f"Decision function called with obligation: {obligation_str}")
        return obligation_str == "good"
    
    # Call arbitrate_past_sync
    result = await oracle_client.arbitrate_past_sync(
        fulfillment_params,
        decision_function,
        options
    )
    
    # Verify arbitration found decisions
    assert not (result.total_count != 1 or result.successful_count != 1), "Expected 1 successful decision, got {result.successful_count}/{result.total_count}"
    
    # Verify decision details
    decision = result.decisions[0]
    assert not (not decision.decision or decision.obligation_data != "good"), "Decision incorrect: {decision.decision}, obligation: {decision.obligation_data}"
    
    # Collect payment
    collection_receipt = await env.bob_client.erc20.collect_escrow(
        escrow_uid, fulfillment_uid
    )
    
    # Verify collection receipt
    assert not (not collection_receipt or not collection_receipt.startswith('0x')), "Invalid collection receipt: {collection_receipt}"
    
    # For compatibility with run_all_tests.py
@pytest.mark.asyncio
async def test_fixed_arbitrate_past_sync():
    """Alias for the main test function to maintain compatibility"""
    return await test_arbitrate_past_sync()
