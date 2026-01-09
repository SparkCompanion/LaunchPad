use anchor_lang::prelude::*;
use crate::{
    state::{Pool, Position},
    constants::*,
    events::LiquidityChangeEvent,
    errors::SparkAmmError,
};

/// ============================================================================
/// INCREASE LIQUIDITY CONTEXT
/// ============================================================================
/// Account validation for adding liquidity to a position.
/// 
/// Labels: [CONTEXT] [ACCOUNTS] [LIQUIDITY]
#[derive(Accounts)]
pub struct IncreaseLiquidity<'info> {
    /// Pool state
    /// Label: [ACCOUNT] [POOL] [WRITE]
    #[account(
        mut,
        constraint = pool.status == POOL_STATUS_INITIALIZED @ SparkAmmError::PoolNotActive
    )]
    pub pool: Account<'info, Pool>,

    /// Position state
    /// Label: [ACCOUNT] [POSITION] [WRITE]
    #[account(
        mut,
        constraint = position.pool_id == pool.id @ SparkAmmError::PositionNotFound,
        constraint = position.owner == owner.key() @ SparkAmmError::NotPositionOwner
    )]
    pub position: Account<'info, Position>,

    /// Token A vault
    /// Label: [ACCOUNT] [VAULT] [TOKEN-A]
    /// CHECK: Pool's token A vault
    #[account(mut)]
    pub vault_a: AccountInfo<'info>,

    /// Token B vault
    /// Label: [ACCOUNT] [VAULT] [TOKEN-B]
    /// CHECK: Pool's token B vault
    #[account(mut)]
    pub vault_b: AccountInfo<'info>,

    /// Owner's token A account
    /// Label: [ACCOUNT] [OWNER] [TOKEN-A]
    /// CHECK: Owner's token account
    #[account(mut)]
    pub owner_token_a: AccountInfo<'info>,

    /// Owner's token B account
    /// Label: [ACCOUNT] [OWNER] [TOKEN-B]
    /// CHECK: Owner's token account
    #[account(mut)]
    pub owner_token_b: AccountInfo<'info>,

    /// Position owner
    /// Label: [ACCOUNT] [OWNER] [SIGNER]
    pub owner: Signer<'info>,

    /// Token program
    /// Label: [PROGRAM] [TOKEN]
    /// CHECK: SPL Token program
    pub token_program: AccountInfo<'info>,
}

/// ============================================================================
/// INCREASE LIQUIDITY HANDLER
/// ============================================================================
/// Adds liquidity to an existing position.
/// 
/// Labels: [HANDLER] [LIQUIDITY] [ADD]
pub fn increase_liquidity(
    ctx: Context<IncreaseLiquidity>,
    liquidity_delta: u128,
    amount0_max: u64,
    amount1_max: u64,
) -> Result<()> {
    let pool = &mut ctx.accounts.pool;
    let position = &mut ctx.accounts.position;
    let clock = Clock::get()?;

    // Validate liquidity amount
    require!(liquidity_delta >= MIN_LIQUIDITY, SparkAmmError::LiquidityTooLow);

    // Calculate token amounts needed for this liquidity
    let sqrt_price_lower = crate::math::tick_to_sqrt_price_x64(position.tick_lower)?;
    let sqrt_price_upper = crate::math::tick_to_sqrt_price_x64(position.tick_upper)?;

    let (amount_a, amount_b) = if pool.sqrt_price_x64 <= sqrt_price_lower {
        // Below range - only token A needed
        let amount_a = crate::math::get_amount_a_from_liquidity(
            sqrt_price_lower,
            sqrt_price_upper,
            liquidity_delta,
            true,
        )?;
        (amount_a, 0u64)
    } else if pool.sqrt_price_x64 < sqrt_price_upper {
        // In range - both tokens needed
        let amount_a = crate::math::get_amount_a_from_liquidity(
            pool.sqrt_price_x64,
            sqrt_price_upper,
            liquidity_delta,
            true,
        )?;
        let amount_b = crate::math::get_amount_b_from_liquidity(
            sqrt_price_lower,
            pool.sqrt_price_x64,
            liquidity_delta,
            true,
        )?;
        (amount_a, amount_b)
    } else {
        // Above range - only token B needed
        let amount_b = crate::math::get_amount_b_from_liquidity(
            sqrt_price_lower,
            sqrt_price_upper,
            liquidity_delta,
            true,
        )?;
        (0u64, amount_b)
    };

    // Check slippage
    require!(amount_a <= amount0_max, SparkAmmError::SlippageExceeded);
    require!(amount_b <= amount1_max, SparkAmmError::SlippageExceeded);

    // Update position liquidity
    position.liquidity = position.liquidity
        .checked_add(liquidity_delta)
        .ok_or(SparkAmmError::Overflow)?;

    // Update pool liquidity if position is in range
    if pool.tick_current >= position.tick_lower && pool.tick_current < position.tick_upper {
        pool.liquidity = pool.liquidity
            .checked_add(liquidity_delta)
            .ok_or(SparkAmmError::Overflow)?;
    }

    pool.updated_at = clock.unix_timestamp;

    // Emit event
    emit!(LiquidityChangeEvent {
        pool_id: pool.id,
        owner: ctx.accounts.owner.key(),
        tick_lower: position.tick_lower,
        tick_upper: position.tick_upper,
        liquidity_delta,
        is_add: true,
        amount_a,
        amount_b,
        timestamp: clock.unix_timestamp,
    });

    msg!("Liquidity added: {} (tokens: {}, {})", liquidity_delta, amount_a, amount_b);

    Ok(())
}
