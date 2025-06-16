# ERC20 Buy ERC721 for ERC20 Test Flow

## Overview

The `test_buy_erc721_for_erc20.py` implements a NFT purchase order flow where:

1. Alice offers ERC20 tokens in exchange for a specific ERC721 NFT
2. Creates an escrow-backed buy order that can be fulfilled by the NFT owner

## Test Flow

### Setup Phase

- Alice receives 100 units of ERC20 tokens
- Balance verification confirms successful transfer

### Purchase Order Creation Phase

1. Alice approves 50 ERC20 tokens for escrow
2. Alice creates buy order: offers 50 ERC20 tokens for NFT token ID 1
3. Alice's ERC20 tokens are moved to escrow
4. Buy attestation UID is generated for the purchase order

### Verification Phase

- Alice's remaining ERC20 balance is verified (initial + 100 - 50 escrowed)
- Escrow contract holds exactly 50 ERC20 tokens
- Valid attestation UID confirms the buy order is live and discoverable

## Key Features Tested

- ✅ **Cross-token-type escrow** - ERC20 tokens escrowed for ERC721 purchase
- ✅ **NFT purchase orders** - Create discoverable buy orders for specific NFTs
- ✅ **Attestation-based order system** - Orders are recorded on-chain via attestations
- ✅ **Escrow security** - Buyer's funds are safely held until fulfillment
- ✅ **Token ID specificity** - Orders target specific NFT token IDs

## Functions Used

- `approve()` - ERC20 token approval for escrow
- `buy_erc721_for_erc20()` - Creates NFT purchase order with ERC20 escrow
- Balance checks for escrow verification

## Real-world Use Cases

1. **NFT Marketplaces** - Users can create standing buy orders for specific NFTs
2. **Collection Trading** - Collectors can make offers on specific pieces
3. **Automated Trading** - Bots can monitor and fulfill profitable orders
4. **Cross-platform Trading** - Orders are discoverable across different frontends

## Order Fulfillment (Future Test)

This test creates the buy order. A complementary test would show:

- NFT owner approving their NFT for transfer
- Calling a fulfillment function with the buy attestation UID
- Atomic exchange: NFT to buyer, ERC20 tokens to seller

## Advantages

1. **Discoverability** - Buy orders are recorded on-chain via attestations
2. **Security** - Buyer's funds are escrowed, preventing rug pulls
3. **Specificity** - Orders target exact NFT token IDs, not just collections
4. **Composability** - Other contracts can read and interact with orders

This test validates the NFT purchase order functionality, demonstrating how users can create secure, discoverable buy orders for specific NFTs using ERC20 tokens as payment.
