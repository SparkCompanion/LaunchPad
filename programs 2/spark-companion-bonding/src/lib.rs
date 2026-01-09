use anchor_lang::prelude::*;

declare_id!("SparkBond111111111111111111111111111111111");

pub mod constants;
pub mod state;
pub mod instructions;
pub mod errors;
pub mod events;

use instructions::*;

/// ============================================================================
/// SPARKCOMPANION BONDING CURVE PROGRAM
/// ============================================================================
/// A fully decentralized bonding curve for token launches.
/// NO admin wallets, NO multi-sig requirements - pure permissionless DeFi.
/// 
/// Features:
/// - Constant product bonding curve (x * y = k)
/// - Automatic price discovery
/// - Migration to AMM when threshold reached
/// - Fair launch mechanism
/// ============================================================================

#[program]
pub mod spark_companion_bonding {
    use super::*;

    /// ========================================================================
    /// INITIALIZE GLOBAL CONFIGURATION
    /// ========================================================================
    /// Sets up the global state for the bonding curve platform.
    /// Called once during deployment. No admin controls.
    /// 
    /// Labels: [SETUP] [ONE-TIME] [PERMISSIONLESS]
    pub fn initialize_global(ctx: Context<InitializeGlobal>) -> Result<()> {
        instructions::initialize_global(ctx)
    }

    /// ========================================================================
    /// INITIALIZE BONDING CURVE FOR NEW TOKEN
    /// ========================================================================
    /// Creates a new bonding curve for a token launch.
    /// Anyone can create a token - fully permissionless.
    /// 
    /// Labels: [TOKEN-LAUNCH] [PERMISSIONLESS] [CREATOR]
    /// 
    /// Parameters:
    /// - name: Token name (max 32 chars)
    /// - symbol: Token symbol (max 10 chars)  
    /// - uri: Metadata URI for token image/info
    pub fn initialize_bonding_curve(
        ctx: Context<InitializeBondingCurve>,
        name: String,
        symbol: String,
        uri: String,
    ) -> Result<()> {
        instructions::initialize_bonding_curve(ctx, name, symbol, uri)
    }

    /// ========================================================================
    /// BUY TOKENS FROM BONDING CURVE
    /// ========================================================================
    /// Purchase tokens using SOL. Price increases with each buy.
    /// 
    /// Labels: [TRADE] [BUY] [USER]
    /// 
    /// Parameters:
    /// - token_amount: Amount of tokens to buy
    /// - max_sol_cost: Maximum SOL willing to spend (slippage protection)
    pub fn buy_tokens(ctx: Context<BuyTokens>, token_amount: u64, max_sol_cost: u64) -> Result<()> {
        instructions::buy_tokens(ctx, token_amount, max_sol_cost)
    }

    /// ========================================================================
    /// SELL TOKENS TO BONDING CURVE
    /// ========================================================================
    /// Sell tokens back for SOL. Price decreases with each sell.
    /// 
    /// Labels: [TRADE] [SELL] [USER]
    /// 
    /// Parameters:
    /// - token_amount: Amount of tokens to sell
    /// - min_sol_received: Minimum SOL to receive (slippage protection)
    pub fn sell_tokens(ctx: Context<SellTokens>, token_amount: u64, min_sol_received: u64) -> Result<()> {
        instructions::sell_tokens(ctx, token_amount, min_sol_received)
    }

    /// ========================================================================
    /// INITIALIZE USER VOLUME TRACKER
    /// ========================================================================
    /// Creates a tracking account for user's trading volume.
    /// Used for analytics and potential rewards.
    /// 
    /// Labels: [USER] [ANALYTICS] [TRACKING]
    pub fn init_user_volume_accumulator(ctx: Context<InitUserVolumeAccumulator>) -> Result<()> {
        instructions::init_user_volume_accumulator(ctx)
    }

    /// ========================================================================
    /// MIGRATE TO AMM
    /// ========================================================================
    /// Migrates the bonding curve to a full AMM pool.
    /// Triggered automatically when SOL threshold is reached.
    /// Fully permissionless - anyone can trigger migration.
    /// 
    /// Labels: [MIGRATION] [AMM] [PERMISSIONLESS] [GRADUATION]
    pub fn migrate_to_amm(ctx: Context<MigrateToAmm>) -> Result<()> {
        instructions::migrate_to_amm(ctx)
    }

    /// ========================================================================
    /// COLLECT CREATOR FEES
    /// ========================================================================
    /// Creator can collect their accumulated trading fees.
    /// Only the original token creator can call this.
    /// 
    /// Labels: [FEES] [CREATOR] [WITHDRAW]
    /// 
    /// Parameters:
    /// - amount: Amount of fees to collect in lamports
    pub fn collect_creator_fees(ctx: Context<CollectCreatorFees>, amount: u64) -> Result<()> {
        instructions::collect_creator_fees(ctx, amount)
    }
}
