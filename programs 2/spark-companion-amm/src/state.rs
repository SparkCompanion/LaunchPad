use anchor_lang::prelude::*;
use crate::errors::SparkAmmError;

/// ============================================================================
/// AMM GLOBAL STATE
/// ============================================================================
/// Platform-wide configuration. Fully decentralized - no admin controls.
/// 
/// Labels: [STATE] [GLOBAL] [PERMISSIONLESS]
#[account]
pub struct AmmGlobal {
    /// Protocol fee rate
    /// Label: [FEES] [PROTOCOL]
    pub protocol_fee_rate: u32,

    /// Fund fee rate
    /// Label: [FEES] [FUND]
    pub fund_fee_rate: u32,

    /// Default trade fee rate for new pools
    /// Label: [FEES] [DEFAULT]
    pub default_trade_fee_rate: u32,

    /// Fee to create a new pool (in lamports)
    /// Label: [FEES] [CREATION]
    pub create_pool_fee: u64,

    /// Total pools created
    /// Label: [ANALYTICS] [POOLS]
    pub total_pools: u32,

    /// Total volume across all pools
    /// Label: [ANALYTICS] [VOLUME]
    pub total_volume: u64,

    /// Total fees collected
    /// Label: [ANALYTICS] [FEES]
    pub total_fees_collected: u64,

    /// Program version
    /// Label: [VERSION] [UPGRADE]
    pub version: u8,

    /// PDA bump
    /// Label: [PDA] [BUMP]
    pub bump: u8,

    /// Reserved space for future upgrades
    /// Label: [RESERVED] [FUTURE]
    pub reserved: [u64; 8],
}

impl AmmGlobal {
    pub const LEN: usize = 8 + // discriminator
        4 + // protocol_fee_rate
        4 + // fund_fee_rate
        4 + // default_trade_fee_rate
        8 + // create_pool_fee
        4 + // total_pools
        8 + // total_volume
        8 + // total_fees_collected
        1 + // version
        1 + // bump
        64; // reserved
}

/// ============================================================================
/// LIQUIDITY POOL STATE
/// ============================================================================
/// Individual pool configuration and state.
/// 
/// Labels: [STATE] [POOL] [CLMM]
#[account]
pub struct Pool {
    /// Pool ID (unique identifier)
    /// Label: [POOL] [ID]
    pub id: Pubkey,

    /// Mint of token A (typically SOL wrapped)
    /// Label: [TOKEN] [MINT-A]
    pub mint_a: Pubkey,

    /// Mint of token B (custom token)
    /// Label: [TOKEN] [MINT-B]
    pub mint_b: Pubkey,

    /// Vault holding token A
    /// Label: [VAULT] [TOKEN-A]
    pub vault_a: Pubkey,

    /// Vault holding token B
    /// Label: [VAULT] [TOKEN-B]
    pub vault_b: Pubkey,

    /// Pool PDA bump seed
    /// Label: [PDA] [BUMP]
    pub bump: u8,

    /// Current sqrt price (Q64.64 fixed point)
    /// Label: [PRICE] [CURRENT]
    pub sqrt_price_x64: u128,

    /// Current tick index
    /// Label: [TICK] [CURRENT]
    pub tick_current: i32,

    /// Tick spacing for this pool
    /// Label: [TICK] [SPACING]
    pub tick_spacing: u16,

    /// Pool status (active, disabled, etc)
    /// Label: [STATUS] [POOL]
    pub status: u8,

    /// Trade fee rate for swaps
    /// Label: [FEES] [TRADE]
    pub trade_fee_rate: u32,

    /// Protocol fee rate
    /// Label: [FEES] [PROTOCOL]
    pub protocol_fee_rate: u32,

    /// Fund fee rate
    /// Label: [FEES] [FUND]
    pub fund_fee_rate: u32,

    /// Total liquidity in the pool
    /// Label: [LIQUIDITY] [TOTAL]
    pub liquidity: u128,

    /// Protocol fees owed in token A
    /// Label: [FEES] [OWED-A]
    pub protocol_fees_token_a: u64,

    /// Protocol fees owed in token B
    /// Label: [FEES] [OWED-B]
    pub protocol_fees_token_b: u64,

    /// Fund fees owed in token A
    /// Label: [FEES] [FUND-A]
    pub fund_fees_token_a: u64,

    /// Fund fees owed in token B
    /// Label: [FEES] [FUND-B]
    pub fund_fees_token_b: u64,

    /// Global fee growth for token A (Q64.64)
    /// Label: [FEES] [GROWTH-A]
    pub fee_growth_global_a_x64: u128,

    /// Global fee growth for token B (Q64.64)
    /// Label: [FEES] [GROWTH-B]
    pub fee_growth_global_b_x64: u128,

    /// Reward configurations (up to 3)
    /// Label: [REWARDS] [CONFIG]
    pub reward_infos: [RewardInfo; 3],

    /// Total volume in token A
    /// Label: [ANALYTICS] [VOLUME-A]
    pub total_volume_a: u64,

    /// Total volume in token B
    /// Label: [ANALYTICS] [VOLUME-B]
    pub total_volume_b: u64,

    /// Pool creation timestamp
    /// Label: [TIMESTAMP] [CREATED]
    pub created_at: i64,

    /// Last interaction timestamp
    /// Label: [TIMESTAMP] [UPDATED]
    pub updated_at: i64,

    /// Reserved space
    /// Label: [RESERVED] [FUTURE]
    pub reserved: [u64; 4],
}

impl Pool {
    pub const LEN: usize = 8 + // discriminator
        32 + // id
        32 + // mint_a
        32 + // mint_b
        32 + // vault_a
        32 + // vault_b
        1 + // bump
        16 + // sqrt_price_x64
        4 + // tick_current
        2 + // tick_spacing
        1 + // status
        4 + // trade_fee_rate
        4 + // protocol_fee_rate
        4 + // fund_fee_rate
        16 + // liquidity
        8 + // protocol_fees_token_a
        8 + // protocol_fees_token_b
        8 + // fund_fees_token_a
        8 + // fund_fees_token_b
        16 + // fee_growth_global_a_x64
        16 + // fee_growth_global_b_x64
        RewardInfo::LEN * 3 + // reward_infos
        8 + // total_volume_a
        8 + // total_volume_b
        8 + // created_at
        8 + // updated_at
        32; // reserved

    /// Check if tick spacing is valid
    pub fn is_valid_tick_spacing(&self) -> bool {
        self.tick_spacing == 10 || self.tick_spacing == 60 || self.tick_spacing == 200
    }
}

/// ============================================================================
/// REWARD INFO
/// ============================================================================
/// Configuration for a single reward token.
/// 
/// Labels: [STATE] [REWARD] [MINING]
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Default)]
pub struct RewardInfo {
    /// Reward token mint
    /// Label: [REWARD] [MINT]
    pub mint: Pubkey,

    /// Reward vault holding tokens
    /// Label: [REWARD] [VAULT]
    pub vault: Pubkey,

    /// Authority that can set emissions
    /// Label: [REWARD] [AUTHORITY]
    pub authority: Pubkey,

    /// Emissions per second (Q64.64)
    /// Label: [REWARD] [EMISSIONS]
    pub emissions_per_second_x64: u128,

    /// Global growth (Q64.64)
    /// Label: [REWARD] [GROWTH]
    pub growth_global_x64: u128,

    /// Last update timestamp
    /// Label: [REWARD] [UPDATED]
    pub last_update_time: u64,

    /// Total amount owed
    /// Label: [REWARD] [OWED]
    pub total_amount_owed: u64,
}

impl RewardInfo {
    pub const LEN: usize = 32 + // mint
        32 + // vault
        32 + // authority
        16 + // emissions_per_second_x64
        16 + // growth_global_x64
        8 + // last_update_time
        8; // total_amount_owed
}

/// ============================================================================
/// LIQUIDITY POSITION
/// ============================================================================
/// A user's liquidity position in a specific price range.
/// 
/// Labels: [STATE] [POSITION] [LP]
#[account]
pub struct Position {
    /// Position NFT mint
    /// Label: [POSITION] [NFT]
    pub mint: Pubkey,

    /// Position owner
    /// Label: [POSITION] [OWNER]
    pub owner: Pubkey,

    /// Pool this position belongs to
    /// Label: [POSITION] [POOL]
    pub pool_id: Pubkey,

    /// Lower tick boundary
    /// Label: [POSITION] [TICK-LOWER]
    pub tick_lower: i32,

    /// Upper tick boundary
    /// Label: [POSITION] [TICK-UPPER]
    pub tick_upper: i32,

    /// Amount of liquidity
    /// Label: [POSITION] [LIQUIDITY]
    pub liquidity: u128,

    /// Fee growth inside last for token A (Q64.64)
    /// Label: [FEES] [GROWTH-A]
    pub fee_growth_inside_last_a_x64: u128,

    /// Fee growth inside last for token B (Q64.64)
    /// Label: [FEES] [GROWTH-B]
    pub fee_growth_inside_last_b_x64: u128,

    /// Fees owed in token A
    /// Label: [FEES] [OWED-A]
    pub fees_owed_a: u64,

    /// Fees owed in token B
    /// Label: [FEES] [OWED-B]
    pub fees_owed_b: u64,

    /// Reward growth inside last (per reward)
    /// Label: [REWARDS] [GROWTH]
    pub reward_growth_inside_last: [u128; 3],

    /// Rewards owed (per reward)
    /// Label: [REWARDS] [OWED]
    pub rewards_owed: [u64; 3],

    /// Position PDA bump
    /// Label: [PDA] [BUMP]
    pub bump: u8,

    /// Reserved space
    /// Label: [RESERVED] [FUTURE]
    pub reserved: [u64; 4],
}

impl Position {
    pub const LEN: usize = 8 + // discriminator
        32 + // mint
        32 + // owner
        32 + // pool_id
        4 + // tick_lower
        4 + // tick_upper
        16 + // liquidity
        16 + // fee_growth_inside_last_a_x64
        16 + // fee_growth_inside_last_b_x64
        8 + // fees_owed_a
        8 + // fees_owed_b
        16 * 3 + // reward_growth_inside_last
        8 * 3 + // rewards_owed
        1 + // bump
        32; // reserved
}

/// ============================================================================
/// TICK ARRAY
/// ============================================================================
/// Array of ticks for efficient price range lookups.
/// 
/// Labels: [STATE] [TICK] [ARRAY]
#[account]
pub struct TickArray {
    /// Start tick index for this array
    /// Label: [TICK] [START]
    pub start_tick_index: i32,

    /// Array of 88 ticks
    /// Label: [TICK] [DATA]
    pub ticks: [Tick; 88],

    /// Number of initialized ticks
    /// Label: [TICK] [COUNT]
    pub initialized_tick_count: u32,

    /// Pool this tick array belongs to
    /// Label: [TICK] [POOL]
    pub pool_id: Pubkey,

    /// PDA bump seed
    /// Label: [PDA] [BUMP]
    pub bump: u8,
}

impl TickArray {
    pub const LEN: usize = 8 + // discriminator
        4 + // start_tick_index
        Tick::LEN * 88 + // ticks
        4 + // initialized_tick_count
        32 + // pool_id
        1; // bump

    /// Check if a tick is within this array
    pub fn contains_tick(&self, tick: i32) -> bool {
        tick >= self.start_tick_index && tick < self.start_tick_index + 88
    }
}

/// ============================================================================
/// TICK
/// ============================================================================
/// Individual tick state for concentrated liquidity.
/// 
/// Labels: [STATE] [TICK] [CLMM]
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Default)]
pub struct Tick {
    /// Net liquidity change when tick is crossed
    /// Label: [TICK] [LIQUIDITY-NET]
    pub liquidity_net: i128,

    /// Gross liquidity on this tick
    /// Label: [TICK] [LIQUIDITY-GROSS]
    pub liquidity_gross: u128,

    /// Fee growth outside for token A (Q64.64)
    /// Label: [FEES] [OUTSIDE-A]
    pub fee_growth_outside_a_x64: u128,

    /// Fee growth outside for token B (Q64.64)
    /// Label: [FEES] [OUTSIDE-B]
    pub fee_growth_outside_b_x64: u128,

    /// Reward growth outside (per reward)
    /// Label: [REWARDS] [OUTSIDE]
    pub reward_growth_outside: [u128; 3],

    /// Whether this tick is initialized
    /// Label: [TICK] [INITIALIZED]
    pub initialized: bool,
}

impl Tick {
    pub const LEN: usize = 16 + // liquidity_net
        16 + // liquidity_gross
        16 + // fee_growth_outside_a_x64
        16 + // fee_growth_outside_b_x64
        16 * 3 + // reward_growth_outside
        1; // initialized
}
