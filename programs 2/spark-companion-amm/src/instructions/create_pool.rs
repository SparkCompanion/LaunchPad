use anchor_lang::prelude::*;
use crate::{
    state::{AmmGlobal, Pool, RewardInfo},
    constants::*,
    events::PoolCreatedEvent,
    errors::SparkAmmError,
};

/// ============================================================================
/// CREATE POOL CONTEXT
/// ============================================================================
/// Account validation for creating a new liquidity pool.
/// 
/// Labels: [CONTEXT] [ACCOUNTS] [POOL-CREATION]
#[derive(Accounts)]
pub struct CreatePool<'info> {
    /// Global state account
    /// Label: [ACCOUNT] [GLOBAL] [WRITE]
    #[account(
        mut,
        seeds = [GLOBAL_SEED],
        bump = global.bump
    )]
    pub global: Account<'info, AmmGlobal>,

    /// Pool state account (PDA)
    /// Label: [ACCOUNT] [POOL] [PDA]
    #[account(
        init,
        payer = creator,
        space = Pool::LEN,
        seeds = [POOL_SEED, mint_a.key().as_ref(), mint_b.key().as_ref()],
        bump
    )]
    pub pool: Account<'info, Pool>,

    /// Token A mint
    /// Label: [ACCOUNT] [TOKEN] [MINT-A]
    /// CHECK: Validated as token mint
    pub mint_a: AccountInfo<'info>,

    /// Token B mint
    /// Label: [ACCOUNT] [TOKEN] [MINT-B]
    /// CHECK: Validated as token mint
    pub mint_b: AccountInfo<'info>,

    /// Vault for token A
    /// Label: [ACCOUNT] [VAULT] [TOKEN-A]
    /// CHECK: Will be initialized as token account
    #[account(
        seeds = [POOL_VAULT_SEED, pool.key().as_ref(), mint_a.key().as_ref()],
        bump
    )]
    pub vault_a: AccountInfo<'info>,

    /// Vault for token B
    /// Label: [ACCOUNT] [VAULT] [TOKEN-B]
    /// CHECK: Will be initialized as token account
    #[account(
        seeds = [POOL_VAULT_SEED, pool.key().as_ref(), mint_b.key().as_ref()],
        bump
    )]
    pub vault_b: AccountInfo<'info>,

    /// Pool creator
    /// Label: [ACCOUNT] [CREATOR] [SIGNER]
    #[account(mut)]
    pub creator: Signer<'info>,

    /// System program
    /// Label: [PROGRAM] [SYSTEM]
    pub system_program: Program<'info, System>,

    /// Token program
    /// Label: [PROGRAM] [TOKEN]
    /// CHECK: SPL Token program
    pub token_program: AccountInfo<'info>,
}

/// ============================================================================
/// CREATE POOL HANDLER
/// ============================================================================
/// Creates a new concentrated liquidity pool.
/// Anyone can create pools - fully permissionless.
/// 
/// Labels: [HANDLER] [POOL] [PERMISSIONLESS]
pub fn create_pool(
    ctx: Context<CreatePool>,
    sqrt_price_x64: u128,
    tick_spacing: u16,
) -> Result<()> {
    let global = &mut ctx.accounts.global;
    let pool = &mut ctx.accounts.pool;
    let clock = Clock::get()?;

    // Validate mints are different
    require!(
        ctx.accounts.mint_a.key() != ctx.accounts.mint_b.key(),
        SparkAmmError::SameTokenMints
    );

    // Validate tick spacing
    require!(
        tick_spacing == TICK_SPACING_10 || 
        tick_spacing == TICK_SPACING_60 || 
        tick_spacing == TICK_SPACING_200,
        SparkAmmError::InvalidTickSpacing
    );

    // Validate sqrt price
    require!(
        sqrt_price_x64 >= MIN_SQRT_PRICE_X64 && sqrt_price_x64 <= MAX_SQRT_PRICE_X64,
        SparkAmmError::PriceOutOfRange
    );

    // Calculate initial tick from sqrt price
    let tick_current = crate::math::sqrt_price_x64_to_tick(sqrt_price_x64)?;

    // Initialize pool state
    pool.id = pool.key();
    pool.mint_a = ctx.accounts.mint_a.key();
    pool.mint_b = ctx.accounts.mint_b.key();
    pool.vault_a = ctx.accounts.vault_a.key();
    pool.vault_b = ctx.accounts.vault_b.key();
    pool.bump = ctx.bumps.pool;
    pool.sqrt_price_x64 = sqrt_price_x64;
    pool.tick_current = tick_current;
    pool.tick_spacing = tick_spacing;
    pool.status = POOL_STATUS_INITIALIZED;
    pool.trade_fee_rate = global.default_trade_fee_rate;
    pool.protocol_fee_rate = global.protocol_fee_rate;
    pool.fund_fee_rate = global.fund_fee_rate;
    pool.liquidity = 0;
    pool.protocol_fees_token_a = 0;
    pool.protocol_fees_token_b = 0;
    pool.fund_fees_token_a = 0;
    pool.fund_fees_token_b = 0;
    pool.fee_growth_global_a_x64 = 0;
    pool.fee_growth_global_b_x64 = 0;
    pool.reward_infos = [RewardInfo::default(); 3];
    pool.total_volume_a = 0;
    pool.total_volume_b = 0;
    pool.created_at = clock.unix_timestamp;
    pool.updated_at = clock.unix_timestamp;
    pool.reserved = [0u64; 4];

    // Update global stats
    global.total_pools = global.total_pools.checked_add(1).unwrap();

    // Emit pool created event
    emit!(PoolCreatedEvent {
        pool_id: pool.id,
        mint_a: pool.mint_a,
        mint_b: pool.mint_b,
        sqrt_price_x64: pool.sqrt_price_x64,
        tick_spacing: pool.tick_spacing,
        trade_fee_rate: pool.trade_fee_rate,
        creator: ctx.accounts.creator.key(),
        timestamp: clock.unix_timestamp,
    });

    msg!("SparkCompanion Pool Created");
    msg!("  Token A: {}", pool.mint_a);
    msg!("  Token B: {}", pool.mint_b);
    msg!("  Tick Spacing: {}", pool.tick_spacing);

    Ok(())
}
