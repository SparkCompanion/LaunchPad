use anchor_lang::prelude::*;
use crate::errors::SparkBondingError;

/// ============================================================================
/// GLOBAL STATE
/// ============================================================================
/// Platform-wide configuration. Fully decentralized - no admin controls.
/// 
/// Labels: [STATE] [GLOBAL] [PERMISSIONLESS]
#[account]
pub struct Global {
    /// Platform fee in basis points
    /// Label: [FEES] [PLATFORM]
    pub platform_fee_basis_points: u16,

    /// Creator fee in basis points
    /// Label: [FEES] [CREATOR]
    pub creator_fee_basis_points: u16,

    /// Migration fee in basis points
    /// Label: [FEES] [MIGRATION]
    pub migration_fee_basis_points: u16,

    /// Maximum allowed slippage in basis points
    /// Label: [SLIPPAGE] [PROTECTION]
    pub max_slippage_basis_points: u16,

    /// Whether migration to AMM is enabled
    /// Label: [MIGRATION] [FLAG]
    pub migration_enabled: bool,

    /// Total volume across all tokens (in lamports)
    /// Label: [ANALYTICS] [VOLUME]
    pub total_volume_sol: u64,

    /// Total fees collected (in lamports)
    /// Label: [ANALYTICS] [FEES]
    pub total_fees_collected: u64,

    /// Number of tokens created on platform
    /// Label: [ANALYTICS] [TOKENS]
    pub tokens_created: u32,

    /// Number of successful migrations to AMM
    /// Label: [ANALYTICS] [MIGRATIONS]
    pub successful_migrations: u32,

    /// Program version for upgrades
    /// Label: [VERSION] [UPGRADE]
    pub version: u8,

    /// Bump seed for PDA
    /// Label: [PDA] [BUMP]
    pub bump: u8,

    /// Reserved space for future upgrades
    /// Label: [RESERVED] [FUTURE]
    pub reserved: [u64; 8],
}

impl Global {
    pub const LEN: usize = 8 + // discriminator
        2 + // platform_fee_basis_points
        2 + // creator_fee_basis_points
        2 + // migration_fee_basis_points
        2 + // max_slippage_basis_points
        1 + // migration_enabled
        8 + // total_volume_sol
        8 + // total_fees_collected
        4 + // tokens_created
        4 + // successful_migrations
        1 + // version
        1 + // bump
        64; // reserved
}

/// ============================================================================
/// BONDING CURVE STATE
/// ============================================================================
/// Individual token's bonding curve configuration and reserves.
/// 
/// Labels: [STATE] [BONDING-CURVE] [TOKEN]
#[account]
pub struct BondingCurve {
    /// Associated token mint address
    /// Label: [TOKEN] [MINT]
    pub token_mint: Pubkey,

    /// Creator of the token (receives creator fees)
    /// Label: [CREATOR] [OWNER]
    pub creator: Pubkey,

    /// Token name (max 32 chars)
    /// Label: [METADATA] [NAME]
    pub name: String,

    /// Token symbol (max 10 chars)
    /// Label: [METADATA] [SYMBOL]
    pub symbol: String,

    /// Virtual SOL reserves for pricing calculation
    /// Label: [PRICING] [VIRTUAL]
    pub virtual_sol_reserves: u64,

    /// Virtual token reserves for pricing calculation
    /// Label: [PRICING] [VIRTUAL]
    pub virtual_token_reserves: u64,

    /// Actual SOL in the vault
    /// Label: [RESERVES] [SOL]
    pub real_sol_reserves: u64,

    /// Actual tokens in the vault
    /// Label: [RESERVES] [TOKENS]
    pub real_token_reserves: u64,

    /// Tokens reserved for LP after migration
    /// Label: [LP] [RESERVE]
    pub lp_reserve_supply: u64,

    /// SOL threshold to trigger migration
    /// Label: [MIGRATION] [THRESHOLD]
    pub migration_threshold: u64,

    /// Whether migration threshold has been reached
    /// Label: [MIGRATION] [READY]
    pub migration_ready: bool,

    /// Whether token has migrated to AMM
    /// Label: [MIGRATION] [COMPLETED]
    pub is_migrated: bool,

    /// AMM program ID after migration
    /// Label: [AMM] [PROGRAM]
    pub amm_program_id: Option<Pubkey>,

    /// AMM pool address after migration
    /// Label: [AMM] [POOL]
    pub amm_pool_address: Option<Pubkey>,

    /// Total SOL volume traded
    /// Label: [ANALYTICS] [VOLUME]
    pub total_volume_sol: u64,

    /// Total token volume traded
    /// Label: [ANALYTICS] [VOLUME]
    pub total_volume_tokens: u64,

    /// Platform fees collected for this token
    /// Label: [FEES] [PLATFORM]
    pub platform_fees_collected: u64,

    /// Creator fees available to claim
    /// Label: [FEES] [CREATOR]
    pub creator_fees_collected: u64,

    /// Number of buy transactions
    /// Label: [ANALYTICS] [BUYS]
    pub buy_count: u32,

    /// Number of sell transactions
    /// Label: [ANALYTICS] [SELLS]
    pub sell_count: u32,

    /// Unix timestamp of creation
    /// Label: [TIMESTAMP] [CREATED]
    pub created_at: i64,

    /// Unix timestamp of last trade
    /// Label: [TIMESTAMP] [LAST-TRADE]
    pub last_trade_at: i64,

    /// PDA bump seeds
    /// Label: [PDA] [BUMPS]
    pub bump: u8,
    pub sol_vault_bump: u8,
    pub token_vault_bump: u8,
    pub lp_reserve_bump: u8,

    /// Reserved space for future upgrades
    /// Label: [RESERVED] [FUTURE]
    pub reserved: [u64; 4],
}

impl BondingCurve {
    pub const LEN: usize = 8 + // discriminator
        32 + // token_mint
        32 + // creator
        4 + 32 + // name (String)
        4 + 10 + // symbol (String)
        8 + // virtual_sol_reserves
        8 + // virtual_token_reserves
        8 + // real_sol_reserves
        8 + // real_token_reserves
        8 + // lp_reserve_supply
        8 + // migration_threshold
        1 + // migration_ready
        1 + // is_migrated
        33 + // amm_program_id (Option<Pubkey>)
        33 + // amm_pool_address (Option<Pubkey>)
        8 + // total_volume_sol
        8 + // total_volume_tokens
        8 + // platform_fees_collected
        8 + // creator_fees_collected
        4 + // buy_count
        4 + // sell_count
        8 + // created_at
        8 + // last_trade_at
        1 + // bump
        1 + // sol_vault_bump
        1 + // token_vault_bump
        1 + // lp_reserve_bump
        32; // reserved

    /// Check if migration threshold has been reached
    /// Label: [MIGRATION] [CHECK]
    pub fn is_migration_threshold_met(&self) -> bool {
        self.real_sol_reserves >= self.migration_threshold
    }

    /// Calculate current token price in lamports per token
    /// Label: [PRICING] [CALCULATION]
    pub fn current_price(&self) -> Result<u64> {
        let total_sol = self.virtual_sol_reserves
            .checked_add(self.real_sol_reserves)
            .ok_or(SparkBondingError::Overflow)?;
        
        let total_tokens = self.virtual_token_reserves
            .checked_sub(self.real_token_reserves)
            .ok_or(SparkBondingError::Underflow)?;

        if total_tokens == 0 {
            return Err(SparkBondingError::DivisionByZero.into());
        }

        const PRECISION_SCALE: u64 = 1_000_000_000;
        
        if total_sol > u64::MAX / PRECISION_SCALE {
            return Err(SparkBondingError::Overflow.into());
        }
        
        let scaled_sol = total_sol
            .checked_mul(PRECISION_SCALE)
            .ok_or(SparkBondingError::Overflow)?;
            
        scaled_sol
            .checked_div(total_tokens)
            .ok_or(SparkBondingError::DivisionByZero.into())
    }

    /// Validate trade amounts before execution
    /// Label: [VALIDATION] [TRADE]
    pub fn validate_trade_amounts(&self, token_amount: u64, is_buy: bool) -> Result<()> {
        require!(token_amount > 0, SparkBondingError::InvalidTokenAmount);
        require!(!self.is_migrated, SparkBondingError::AlreadyMigrated);
        
        if is_buy {
            require!(
                token_amount <= self.real_token_reserves,
                SparkBondingError::InsufficientTokenReserves
            );
        } else {
            let circulating_supply = self.virtual_token_reserves
                .checked_sub(self.real_token_reserves)
                .ok_or(SparkBondingError::Underflow)?;
            require!(
                token_amount <= circulating_supply,
                SparkBondingError::InvalidTokenAmount
            );
        }
        
        Ok(())
    }
}

/// ============================================================================
/// USER VOLUME ACCUMULATOR
/// ============================================================================
/// Tracks individual user's trading activity for analytics/rewards.
/// 
/// Labels: [STATE] [USER] [ANALYTICS]
#[account]
pub struct UserVolumeAccumulator {
    /// User's public key
    /// Label: [USER] [ADDRESS]
    pub user: Pubkey,

    /// Total SOL volume traded
    /// Label: [VOLUME] [SOL]
    pub volume_sol: u64,

    /// Total token volume traded
    /// Label: [VOLUME] [TOKENS]
    pub volume_tokens: u64,

    /// Total number of trades
    /// Label: [ANALYTICS] [COUNT]
    pub trades_count: u32,

    /// Last trade timestamp
    /// Label: [TIMESTAMP] [LAST-TRADE]
    pub last_trade_timestamp: i64,

    /// PDA bump
    /// Label: [PDA] [BUMP]
    pub bump: u8,

    /// Reserved space
    /// Label: [RESERVED] [FUTURE]
    pub reserved: [u64; 2],
}

impl UserVolumeAccumulator {
    pub const LEN: usize = 8 + // discriminator
        32 + // user
        8 + // volume_sol
        8 + // volume_tokens
        4 + // trades_count
        8 + // last_trade_timestamp
        1 + // bump
        16; // reserved
}
