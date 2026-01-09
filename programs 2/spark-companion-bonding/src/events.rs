use anchor_lang::prelude::*;

/// ============================================================================
/// SPARKCOMPANION BONDING CURVE EVENTS
/// ============================================================================
/// All events emitted by the bonding curve program for indexing/tracking.
/// 
/// Labels: [EVENTS] [INDEXING] [ANALYTICS]

/// ========================================================================
/// TOKEN CREATION EVENT
/// ========================================================================
/// Emitted when a new token bonding curve is created.
/// 
/// Labels: [EVENT] [TOKEN] [CREATION]
#[event]
pub struct TokenCreatedEvent {
    /// Token mint address
    pub token_mint: Pubkey,
    /// Creator wallet address
    pub creator: Pubkey,
    /// Token name
    pub name: String,
    /// Token symbol
    pub symbol: String,
    /// Initial virtual SOL reserves
    pub virtual_sol_reserves: u64,
    /// Initial virtual token reserves
    pub virtual_token_reserves: u64,
    /// Migration threshold in lamports
    pub migration_threshold: u64,
    /// Creation timestamp
    pub timestamp: i64,
}

/// ========================================================================
/// TRADE EVENT
/// ========================================================================
/// Emitted on every buy/sell transaction.
/// 
/// Labels: [EVENT] [TRADE] [VOLUME]
#[event]
pub struct TradeEvent {
    /// Token mint address
    pub token_mint: Pubkey,
    /// Trader wallet address
    pub trader: Pubkey,
    /// Whether this is a buy (true) or sell (false)
    pub is_buy: bool,
    /// Amount of tokens traded
    pub token_amount: u64,
    /// Amount of SOL traded
    pub sol_amount: u64,
    /// Platform fee charged
    pub platform_fee: u64,
    /// Creator fee charged
    pub creator_fee: u64,
    /// New token price after trade
    pub new_price: u64,
    /// Current SOL reserves
    pub sol_reserves: u64,
    /// Current token reserves
    pub token_reserves: u64,
    /// Trade timestamp
    pub timestamp: i64,
}

/// ========================================================================
/// MIGRATION EVENT
/// ========================================================================
/// Emitted when token graduates to AMM.
/// 
/// Labels: [EVENT] [MIGRATION] [GRADUATION]
#[event]
pub struct MigrationEvent {
    /// Token mint address
    pub token_mint: Pubkey,
    /// Token creator
    pub creator: Pubkey,
    /// AMM program ID
    pub amm_program_id: Pubkey,
    /// AMM pool address
    pub amm_pool_address: Pubkey,
    /// Final SOL in bonding curve
    pub final_sol_reserves: u64,
    /// Final tokens in bonding curve
    pub final_token_reserves: u64,
    /// LP tokens allocated
    pub lp_tokens_allocated: u64,
    /// Migration timestamp
    pub timestamp: i64,
}

/// ========================================================================
/// FEE COLLECTION EVENT
/// ========================================================================
/// Emitted when creator collects their fees.
/// 
/// Labels: [EVENT] [FEES] [WITHDRAWAL]
#[event]
pub struct CreatorFeesCollectedEvent {
    /// Token mint address
    pub token_mint: Pubkey,
    /// Creator wallet address
    pub creator: Pubkey,
    /// Amount of fees collected
    pub amount: u64,
    /// Destination wallet
    pub destination: Pubkey,
    /// Collection timestamp
    pub timestamp: i64,
}

/// ========================================================================
/// GLOBAL INITIALIZED EVENT
/// ========================================================================
/// Emitted when global state is initialized.
/// 
/// Labels: [EVENT] [GLOBAL] [INITIALIZATION]
#[event]
pub struct GlobalInitializedEvent {
    /// Platform fee in basis points
    pub platform_fee: u16,
    /// Creator fee in basis points
    pub creator_fee: u16,
    /// Migration fee in basis points
    pub migration_fee: u16,
    /// Whether migration is enabled
    pub migration_enabled: bool,
    /// Initialization timestamp
    pub timestamp: i64,
}

/// ========================================================================
/// PRICE UPDATE EVENT
/// ========================================================================
/// Emitted to track price changes for charting.
/// 
/// Labels: [EVENT] [PRICE] [CHARTING]
#[event]
pub struct PriceUpdateEvent {
    /// Token mint address
    pub token_mint: Pubkey,
    /// Current price in lamports per token
    pub price: u64,
    /// Current market cap in lamports
    pub market_cap: u64,
    /// 24h volume (if tracked)
    pub volume_24h: u64,
    /// Bonding curve progress percentage (0-100)
    pub bonding_progress: u8,
    /// Update timestamp
    pub timestamp: i64,
}
