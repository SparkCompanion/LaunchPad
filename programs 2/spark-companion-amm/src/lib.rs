use anchor_lang::prelude::*;

declare_id!("SparkAMM1111111111111111111111111111111111");

pub mod constants;
pub mod state;
pub mod instructions;
pub mod errors;
pub mod events;
pub mod math;

use instructions::*;

/// ============================================================================
/// SPARKCOMPANION AMM PROGRAM
/// ============================================================================
/// A fully decentralized Concentrated Liquidity Market Maker (CLMM).
/// NO admin wallets, NO multi-sig requirements - pure permissionless DeFi.
/// 
/// Features:
/// - Concentrated liquidity positions (like Uniswap V3)
/// - Efficient capital utilization
/// - Position NFTs for liquidity providers
/// - Multi-tier fee structure
/// - Reward emissions for liquidity mining
/// ============================================================================

#[program]
pub mod spark_companion_amm {
    use super::*;

    /// ========================================================================
    /// INITIALIZE AMM GLOBAL CONFIGURATION
    /// ========================================================================
    /// Sets up the global state for the AMM platform.
    /// Called once during deployment. No admin controls.
    /// 
    /// Labels: [SETUP] [ONE-TIME] [PERMISSIONLESS]
    pub fn initialize_amm_global(ctx: Context<InitializeAmmGlobal>) -> Result<()> {
        instructions::initialize_amm_global(ctx)
    }

    /// ========================================================================
    /// CREATE LIQUIDITY POOL
    /// ========================================================================
    /// Creates a new concentrated liquidity pool for a token pair.
    /// Anyone can create pools - fully permissionless.
    /// 
    /// Labels: [POOL] [CREATE] [PERMISSIONLESS]
    /// 
    /// Parameters:
    /// - sqrt_price_x64: Initial price (Q64.64 fixed point)
    /// - tick_spacing: Tick spacing for the pool (10, 60, or 200)
    pub fn create_pool(
        ctx: Context<CreatePool>,
        sqrt_price_x64: u128,
        tick_spacing: u16,
    ) -> Result<()> {
        instructions::create_pool(ctx, sqrt_price_x64, tick_spacing)
    }

    /// ========================================================================
    /// OPEN LIQUIDITY POSITION
    /// ========================================================================
    /// Creates a new liquidity position NFT for a price range.
    /// 
    /// Labels: [POSITION] [CREATE] [LP]
    /// 
    /// Parameters:
    /// - tick_lower: Lower price boundary tick
    /// - tick_upper: Upper price boundary tick
    pub fn open_position(
        ctx: Context<OpenPosition>,
        tick_lower: i32,
        tick_upper: i32,
    ) -> Result<()> {
        instructions::open_position(ctx, tick_lower, tick_upper)
    }

    /// ========================================================================
    /// INCREASE LIQUIDITY
    /// ========================================================================
    /// Adds liquidity to an existing position.
    /// 
    /// Labels: [LIQUIDITY] [ADD] [LP]
    /// 
    /// Parameters:
    /// - liquidity_delta: Amount of liquidity to add
    /// - amount0_max: Max token A to deposit
    /// - amount1_max: Max token B to deposit
    pub fn increase_liquidity(
        ctx: Context<IncreaseLiquidity>,
        liquidity_delta: u128,
        amount0_max: u64,
        amount1_max: u64,
    ) -> Result<()> {
        instructions::increase_liquidity(ctx, liquidity_delta, amount0_max, amount1_max)
    }

    /// ========================================================================
    /// DECREASE LIQUIDITY
    /// ========================================================================
    /// Removes liquidity from an existing position.
    /// 
    /// Labels: [LIQUIDITY] [REMOVE] [LP]
    /// 
    /// Parameters:
    /// - liquidity_delta: Amount of liquidity to remove
    /// - amount0_min: Min token A to receive
    /// - amount1_min: Min token B to receive
    pub fn decrease_liquidity(
        ctx: Context<DecreaseLiquidity>,
        liquidity_delta: u128,
        amount0_min: u64,
        amount1_min: u64,
    ) -> Result<()> {
        instructions::decrease_liquidity(ctx, liquidity_delta, amount0_min, amount1_min)
    }

    /// ========================================================================
    /// SWAP TOKENS
    /// ========================================================================
    /// Executes a swap through the pool.
    /// 
    /// Labels: [SWAP] [TRADE] [USER]
    /// 
    /// Parameters:
    /// - amount: Input amount to swap
    /// - other_amount_threshold: Slippage protection
    /// - sqrt_price_limit_x64: Price limit for the swap
    /// - is_base_input: True if swapping token A for B
    pub fn swap(
        ctx: Context<Swap>,
        amount: u64,
        other_amount_threshold: u64,
        sqrt_price_limit_x64: u128,
        is_base_input: bool,
    ) -> Result<()> {
        instructions::swap(ctx, amount, other_amount_threshold, sqrt_price_limit_x64, is_base_input)
    }

    /// ========================================================================
    /// COLLECT POSITION FEES
    /// ========================================================================
    /// Collects accumulated trading fees from a position.
    /// Only the position owner can collect.
    /// 
    /// Labels: [FEES] [COLLECT] [LP]
    /// 
    /// Parameters:
    /// - amount0_requested: Max token A fees to collect
    /// - amount1_requested: Max token B fees to collect
    pub fn collect_fees(
        ctx: Context<CollectFees>,
        amount0_requested: u64,
        amount1_requested: u64,
    ) -> Result<()> {
        instructions::collect_fees(ctx, amount0_requested, amount1_requested)
    }

    /// ========================================================================
    /// INITIALIZE TICK ARRAY
    /// ========================================================================
    /// Initializes a tick array for a price range.
    /// 
    /// Labels: [TICK] [INITIALIZE] [POOL]
    /// 
    /// Parameters:
    /// - start_tick_index: Starting tick for the array
    pub fn initialize_tick_array(
        ctx: Context<InitializeTickArray>,
        start_tick_index: i32,
    ) -> Result<()> {
        instructions::initialize_tick_array(ctx, start_tick_index)
    }

    /// ========================================================================
    /// INITIALIZE REWARD
    /// ========================================================================
    /// Sets up reward emissions for liquidity mining.
    /// Pool creator can initialize rewards.
    /// 
    /// Labels: [REWARDS] [INITIALIZE] [MINING]
    /// 
    /// Parameters:
    /// - reward_index: Reward slot (0-2)
    pub fn initialize_reward(
        ctx: Context<InitializeReward>,
        reward_index: u8,
    ) -> Result<()> {
        instructions::initialize_reward(ctx, reward_index)
    }

    /// ========================================================================
    /// SET POOL REWARD EMISSIONS
    /// ========================================================================
    /// Updates reward emissions rate.
    /// Only reward authority can update.
    /// 
    /// Labels: [REWARDS] [UPDATE] [MINING]
    /// 
    /// Parameters:
    /// - reward_index: Reward slot to update
    /// - emissions_per_second_x64: New emission rate
    pub fn set_pool_reward(
        ctx: Context<SetPoolReward>,
        reward_index: u8,
        emissions_per_second_x64: u128,
    ) -> Result<()> {
        instructions::set_pool_reward(ctx, reward_index, emissions_per_second_x64)
    }
}
