use anchor_lang::prelude::*;
use crate::{
    state::{AmmGlobal, Pool},
    constants::*,
    events::SwapEvent,
    errors::SparkAmmError,
};

/// ============================================================================
/// SWAP CONTEXT
/// ============================================================================
/// Account validation for executing a swap.
/// 
/// Labels: [CONTEXT] [ACCOUNTS] [SWAP]
#[derive(Accounts)]
pub struct Swap<'info> {
    /// Global state
    /// Label: [ACCOUNT] [GLOBAL] [WRITE]
    #[account(
        mut,
        seeds = [GLOBAL_SEED],
        bump = global.bump
    )]
    pub global: Account<'info, AmmGlobal>,

    /// Pool state
    /// Label: [ACCOUNT] [POOL] [WRITE]
    #[account(
        mut,
        constraint = pool.status == POOL_STATUS_INITIALIZED @ SparkAmmError::PoolNotActive
    )]
    pub pool: Account<'info, Pool>,

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

    /// Trader's input token account
    /// Label: [ACCOUNT] [TRADER] [INPUT]
    /// CHECK: Trader's token account
    #[account(mut)]
    pub trader_input: AccountInfo<'info>,

    /// Trader's output token account
    /// Label: [ACCOUNT] [TRADER] [OUTPUT]
    /// CHECK: Trader's token account
    #[account(mut)]
    pub trader_output: AccountInfo<'info>,

    /// Trader wallet
    /// Label: [ACCOUNT] [TRADER] [SIGNER]
    pub trader: Signer<'info>,

    /// Token program
    /// Label: [PROGRAM] [TOKEN]
    /// CHECK: SPL Token program
    pub token_program: AccountInfo<'info>,
}

/// ============================================================================
/// SWAP HANDLER
/// ============================================================================
/// Executes a swap through the concentrated liquidity pool.
/// 
/// Labels: [HANDLER] [SWAP] [TRADE]
pub fn swap(
    ctx: Context<Swap>,
    amount: u64,
    other_amount_threshold: u64,
    sqrt_price_limit_x64: u128,
    is_base_input: bool,
) -> Result<()> {
    let global = &mut ctx.accounts.global;
    let pool = &mut ctx.accounts.pool;
    let clock = Clock::get()?;

    // Validate amount
    require!(amount > 0, SparkAmmError::ZeroAmount);

    // Validate price limit
    let a_to_b = is_base_input;
    if a_to_b {
        require!(
            sqrt_price_limit_x64 < pool.sqrt_price_x64 && sqrt_price_limit_x64 >= MIN_SQRT_PRICE_X64,
            SparkAmmError::SqrtPriceLimitExceeded
        );
    } else {
        require!(
            sqrt_price_limit_x64 > pool.sqrt_price_x64 && sqrt_price_limit_x64 <= MAX_SQRT_PRICE_X64,
            SparkAmmError::SqrtPriceLimitExceeded
        );
    }

    // Execute swap (simplified - full implementation would iterate through ticks)
    let (sqrt_price_next, amount_in, amount_out, fee_amount) = crate::math::compute_swap_step(
        pool.sqrt_price_x64,
        sqrt_price_limit_x64,
        pool.liquidity,
        amount,
        pool.trade_fee_rate,
        a_to_b,
    )?;

    // Validate slippage
    if is_base_input {
        require!(amount_out >= other_amount_threshold, SparkAmmError::SlippageExceeded);
    } else {
        require!(amount_in <= other_amount_threshold, SparkAmmError::SlippageExceeded);
    }

    // Update pool state
    pool.sqrt_price_x64 = sqrt_price_next;
    pool.tick_current = crate::math::sqrt_price_x64_to_tick(sqrt_price_next)?;
    pool.updated_at = clock.unix_timestamp;

    // Update fee growth
    if pool.liquidity > 0 {
        let fee_growth_delta = (fee_amount as u128)
            .checked_mul(Q64)
            .ok_or(SparkAmmError::Overflow)?
            .checked_div(pool.liquidity)
            .ok_or(SparkAmmError::DivisionByZero)?;

        if a_to_b {
            pool.fee_growth_global_a_x64 = pool.fee_growth_global_a_x64
                .checked_add(fee_growth_delta)
                .ok_or(SparkAmmError::Overflow)?;
            pool.total_volume_a = pool.total_volume_a
                .checked_add(amount_in)
                .ok_or(SparkAmmError::Overflow)?;
        } else {
            pool.fee_growth_global_b_x64 = pool.fee_growth_global_b_x64
                .checked_add(fee_growth_delta)
                .ok_or(SparkAmmError::Overflow)?;
            pool.total_volume_b = pool.total_volume_b
                .checked_add(amount_in)
                .ok_or(SparkAmmError::Overflow)?;
        }
    }

    // Update protocol fees
    let protocol_fee = fee_amount
        .checked_mul(pool.protocol_fee_rate as u64)
        .ok_or(SparkAmmError::Overflow)?
        .checked_div(FEE_RATE_DENOMINATOR_VALUE)
        .ok_or(SparkAmmError::DivisionByZero)?;

    if a_to_b {
        pool.protocol_fees_token_a = pool.protocol_fees_token_a
            .checked_add(protocol_fee)
            .ok_or(SparkAmmError::Overflow)?;
    } else {
        pool.protocol_fees_token_b = pool.protocol_fees_token_b
            .checked_add(protocol_fee)
            .ok_or(SparkAmmError::Overflow)?;
    }

    // Update global stats
    global.total_volume = global.total_volume
        .checked_add(amount_in as u64)
        .ok_or(SparkAmmError::Overflow)?;
    global.total_fees_collected = global.total_fees_collected
        .checked_add(fee_amount)
        .ok_or(SparkAmmError::Overflow)?;

    // Emit swap event
    emit!(SwapEvent {
        pool_id: pool.id,
        trader: ctx.accounts.trader.key(),
        amount_a: if a_to_b { amount_in } else { amount_out },
        amount_b: if a_to_b { amount_out } else { amount_in },
        a_to_b,
        sqrt_price_x64: pool.sqrt_price_x64,
        tick: pool.tick_current,
        fee_amount,
        timestamp: clock.unix_timestamp,
    });

    msg!("Swap executed: {} -> {} (fee: {})", amount_in, amount_out, fee_amount);

    Ok(())
}
