# SparkCompanion Solana Programs

Fully decentralized, permissionless DeFi programs for token launches and trading.

## Overview

SparkCompanion provides two Solana programs for a complete token launch ecosystem:

1. **spark-companion-bonding** - Bonding Curve for Token Launches
2. **spark-companion-amm** - Concentrated Liquidity AMM

## Key Features

### No Admin Wallets / No Multi-sig Required

These programs are **fully permissionless**:
- Anyone can create tokens
- Anyone can trade
- Anyone can provide liquidity
- No centralized control
- No admin keys that could be compromised

### Bonding Curve (spark-companion-bonding)

A constant product bonding curve for fair token launches:

| Feature | Description |
|---------|-------------|
| **Fair Launch** | Equal opportunity for all buyers |
| **Automatic Pricing** | Price discovery through bonding curve |
| **Migration** | Graduates to AMM at 85 SOL threshold |
| **Creator Fees** | 1% to token creator |
| **Platform Fees** | 1% for platform sustainability |

#### Instructions

- `initialize_global` - One-time platform setup
- `initialize_bonding_curve` - Create new token
- `buy_tokens` - Purchase tokens with SOL
- `sell_tokens` - Sell tokens for SOL
- `migrate_to_amm` - Graduate to AMM (permissionless)
- `collect_creator_fees` - Creator withdraws fees

### AMM (spark-companion-amm)

A concentrated liquidity AMM similar to Uniswap V3:

| Feature | Description |
|---------|-------------|
| **Concentrated Liquidity** | Capital efficient positions |
| **Multiple Fee Tiers** | 0.05%, 0.30%, 1.00% |
| **Position NFTs** | LP positions as NFTs |
| **Rewards** | Up to 3 reward tokens per pool |

#### Instructions

- `initialize_amm_global` - One-time platform setup
- `create_pool` - Create new liquidity pool
- `open_position` - Create LP position
- `increase_liquidity` - Add to position
- `decrease_liquidity` - Remove from position
- `swap` - Execute trade
- `collect_fees` - Collect LP fees
- `initialize_reward` - Setup liquidity mining

## Constants

### Bonding Curve

```rust
VIRTUAL_SOL_RESERVES: 30 SOL
VIRTUAL_TOKEN_RESERVES: 1,000,000,000 tokens
MIGRATION_THRESHOLD: 85 SOL
TOTAL_SUPPLY: 1,000,000,000 tokens
LP_RESERVE: 20%
PLATFORM_FEE: 1%
CREATOR_FEE: 1%
```

### AMM

```rust
TICK_SPACING_10: 0.05% fee tier
TICK_SPACING_60: 0.30% fee tier
TICK_SPACING_200: 1.00% fee tier
MIN_LIQUIDITY: 100,000
REWARD_SLOTS: 3
```

## Building

```bash
# Build bonding curve
cd programs/spark-companion-bonding
cargo build-sbf

# Build AMM
cd programs/spark-companion-amm
cargo build-sbf
```

## Testing

```bash
anchor test
```

## Deployment

```bash
# Deploy to devnet
anchor deploy --provider.cluster devnet

# Deploy to mainnet
anchor deploy --provider.cluster mainnet
```

## Security

- All math operations use checked arithmetic
- Slippage protection on all trades
- No admin keys or backdoors
- Fully auditable on-chain

## License

MIT License - SparkCompanion Team
