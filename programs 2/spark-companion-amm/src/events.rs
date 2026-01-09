use anchor_lang::prelude::*;

/// ============================================================================
/// SPARKCOMPANION AMM EVENTS
/// ============================================================================
/// All events emitted by the AMM program for indexing/tracking.
/// 
/// Labels: [EVENTS] [INDEXING] [ANALYTICS]

/// ========================================================================
/// POOL CREATED EVENT
/// ========================================================================
/// Emitted when a new liquidity pool is created.
/// 
/// Labels: [EVENT] [POOL] [CREATION]
#[event]
pub struct PoolCreatedEvent {
    /// Pool ID
    pub pool_id: Pubkey,
    /// Token A mint
    pub mint_a: Pubkey,
    /// Token B mint
    pub mint_b: Pubkey,
    /// Initial sqrt price (Q64.64)
    pub sqrt_price_x64: u128,
    /// Tick spacing
    pub tick_spacing: u16,
    /// Trade fee rate
    pub trade_fee_rate: u32,
    /// Creator wallet
    pub creator: Pubkey,
    /// Creation timestamp
    pub timestamp: i64,
}

/// ========================================================================
/// SWAP EVENT
/// ========================================================================
/// Emitted on every swap transaction.
/// 
/// Labels: [EVENT] [SWAP] [TRADE]
#[event]
pub struct SwapEvent {
    /// Pool ID
    pub pool_id: Pubkey,
    /// Trader wallet
    pub trader: Pubkey,
    /// Amount of token A swapped
    pub amount_a: u64,
    /// Amount of token B swapped
    pub amount_b: u64,
    /// Whether token A was input
    pub a_to_b: bool,
    /// New sqrt price (Q64.64)
    pub sqrt_price_x64: u128,
    /// New tick
    pub tick: i32,
    /// Fee amount
    pub fee_amount: u64,
    /// Swap timestamp
    pub timestamp: i64,
}

/// ========================================================================
/// LIQUIDITY CHANGE EVENT
/// ========================================================================
/// Emitted when liquidity is added or removed.
/// 
/// Labels: [EVENT] [LIQUIDITY] [LP]
#[event]
pub struct LiquidityChangeEvent {
    /// Pool ID
    pub pool_id: Pubkey,
    /// Position owner
    pub owner: Pubkey,
    /// Lower tick
    pub tick_lower: i32,
    /// Upper tick
    pub tick_upper: i32,
    /// Liquidity delta (positive for add, negative encoded as separate bool)
    pub liquidity_delta: u128,
    /// Whether this is an add (true) or remove (false)
    pub is_add: bool,
    /// Amount of token A
    pub amount_a: u64,
    /// Amount of token B
    pub amount_b: u64,
    /// Timestamp
    pub timestamp: i64,
}

/// ========================================================================
/// POSITION CREATED EVENT
/// ========================================================================
/// Emitted when a new position is opened.
/// 
/// Labels: [EVENT] [POSITION] [CREATION]
#[event]
pub struct PositionCreatedEvent {
    /// Position mint (NFT)
    pub position_mint: Pubkey,
    /// Owner wallet
    pub owner: Pubkey,
    /// Pool ID
    pub pool_id: Pubkey,
    /// Lower tick
    pub tick_lower: i32,
    /// Upper tick
    pub tick_upper: i32,
    /// Creation timestamp
    pub timestamp: i64,
}

/// ========================================================================
/// FEES COLLECTED EVENT
/// ========================================================================
/// Emitted when position fees are collected.
/// 
/// Labels: [EVENT] [FEES] [COLLECTION]
#[event]
pub struct FeesCollectedEvent {
    /// Position mint
    pub position_mint: Pubkey,
    /// Owner wallet
    pub owner: Pubkey,
    /// Pool ID
    pub pool_id: Pubkey,
    /// Token A fees collected
    pub amount_a: u64,
    /// Token B fees collected
    pub amount_b: u64,
    /// Collection timestamp
    pub timestamp: i64,
}

/// ========================================================================
/// REWARD INITIALIZED EVENT
/// ========================================================================
/// Emitted when a reward is set up for a pool.
/// 
/// Labels: [EVENT] [REWARDS] [INITIALIZATION]
#[event]
pub struct RewardInitializedEvent {
    /// Pool ID
    pub pool_id: Pubkey,
    /// Reward index (0-2)
    pub reward_index: u8,
    /// Reward token mint
    pub reward_mint: Pubkey,
    /// Reward authority
    pub authority: Pubkey,
    /// Initialization timestamp
    pub timestamp: i64,
}

/// ========================================================================
/// AMM GLOBAL INITIALIZED EVENT
/// ========================================================================
/// Emitted when global state is initialized.
/// 
/// Labels: [EVENT] [GLOBAL] [INITIALIZATION]
#[event]
pub struct AmmGlobalInitializedEvent {
    /// Protocol fee rate
    pub protocol_fee_rate: u32,
    /// Default trade fee rate
    pub default_trade_fee_rate: u32,
    /// Initialization timestamp
    pub timestamp: i64,
}
