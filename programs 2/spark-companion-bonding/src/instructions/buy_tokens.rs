use anchor_lang::prelude::*;
use crate::{
    state::{Global, BondingCurve, UserVolumeAccumulator},
    constants::*,
    events::{TradeEvent, PriceUpdateEvent},
    errors::SparkBondingError,
};

/// ============================================================================
/// BUY TOKENS CONTEXT
/// ============================================================================
/// Account validation for buying tokens from the bonding curve.
/// 
/// Labels: [CONTEXT] [ACCOUNTS] [BUY]
#[derive(Accounts)]
pub struct BuyTokens<'info> {
    /// Global state account
    /// Label: [ACCOUNT] [GLOBAL] [READ]
    #[account(
        mut,
        seeds = [GLOBAL_SEED],
        bump = global.bump
    )]
    pub global: Account<'info, Global>,

    /// Bonding curve state
    /// Label: [ACCOUNT] [BONDING-CURVE] [WRITE]
    #[account(
        mut,
        seeds = [BONDING_CURVE_SEED, bonding_curve.token_mint.as_ref()],
        bump = bonding_curve.bump
    )]
    pub bonding_curve: Account<'info, BondingCurve>,

    /// User's volume tracker (optional - created separately)
    /// Label: [ACCOUNT] [USER] [VOLUME]
    #[account(
        mut,
        seeds = [USER_VOLUME_SEED, buyer.key().as_ref()],
        bump = user_volume.bump
    )]
    pub user_volume: Option<Account<'info, UserVolumeAccumulator>>,

    /// SOL vault to receive payment
    /// Label: [ACCOUNT] [VAULT] [SOL]
    /// CHECK: PDA validated by seeds
    #[account(
        mut,
        seeds = [SOL_VAULT_SEED, bonding_curve.token_mint.as_ref()],
        bump = bonding_curve.sol_vault_bump
    )]
    pub sol_vault: AccountInfo<'info>,

    /// Token vault to send tokens from
    /// Label: [ACCOUNT] [VAULT] [TOKEN]
    /// CHECK: Associated token account
    #[account(mut)]
    pub token_vault: AccountInfo<'info>,

    /// Buyer's token account to receive tokens
    /// Label: [ACCOUNT] [BUYER] [TOKEN]
    /// CHECK: Associated token account
    #[account(mut)]
    pub buyer_token_account: AccountInfo<'info>,

    /// Buyer wallet
    /// Label: [ACCOUNT] [BUYER] [SIGNER]
    #[account(mut)]
    pub buyer: Signer<'info>,

    /// Token program
    /// Label: [PROGRAM] [TOKEN]
    /// CHECK: SPL Token program
    pub token_program: AccountInfo<'info>,

    /// System program
    /// Label: [PROGRAM] [SYSTEM]
    pub system_program: Program<'info, System>,
}

/// ============================================================================
/// BUY TOKENS HANDLER
/// ============================================================================
/// Purchases tokens from the bonding curve using SOL.
/// Uses constant product formula: x * y = k
/// 
/// Labels: [HANDLER] [TRADE] [BUY]
pub fn buy_tokens(
    ctx: Context<BuyTokens>,
    token_amount: u64,
    max_sol_cost: u64,
) -> Result<()> {
    let bonding_curve = &mut ctx.accounts.bonding_curve;
    let global = &mut ctx.accounts.global;
    let clock = Clock::get()?;

    // Validate trade
    bonding_curve.validate_trade_amounts(token_amount, true)?;

    // Calculate SOL cost using constant product formula
    // price = (virtual_sol + real_sol) / (virtual_token - sold_tokens)
    let total_sol = bonding_curve.virtual_sol_reserves
        .checked_add(bonding_curve.real_sol_reserves)
        .ok_or(SparkBondingError::Overflow)?;

    let total_tokens = bonding_curve.virtual_token_reserves
        .checked_sub(bonding_curve.real_token_reserves)
        .ok_or(SparkBondingError::Underflow)?;

    // Calculate SOL needed for this many tokens (with slippage)
    let new_total_tokens = total_tokens
        .checked_add(token_amount)
        .ok_or(SparkBondingError::Overflow)?;

    // k = total_sol * total_tokens
    // new_sol = k / new_total_tokens
    let k = (total_sol as u128)
        .checked_mul(total_tokens as u128)
        .ok_or(SparkBondingError::Overflow)?;

    let new_sol_reserves = k
        .checked_div(new_total_tokens as u128)
        .ok_or(SparkBondingError::DivisionByZero)? as u64;

    let sol_cost = total_sol
        .checked_sub(new_sol_reserves)
        .ok_or(SparkBondingError::Underflow)?;

    // Calculate fees
    let platform_fee = sol_cost
        .checked_mul(global.platform_fee_basis_points as u64)
        .ok_or(SparkBondingError::Overflow)?
        .checked_div(BASIS_POINTS_DENOMINATOR)
        .ok_or(SparkBondingError::DivisionByZero)?;

    let creator_fee = sol_cost
        .checked_mul(global.creator_fee_basis_points as u64)
        .ok_or(SparkBondingError::Overflow)?
        .checked_div(BASIS_POINTS_DENOMINATOR)
        .ok_or(SparkBondingError::DivisionByZero)?;

    let total_cost = sol_cost
        .checked_add(platform_fee)
        .ok_or(SparkBondingError::Overflow)?
        .checked_add(creator_fee)
        .ok_or(SparkBondingError::Overflow)?;

    // Check slippage
    require!(total_cost <= max_sol_cost, SparkBondingError::SlippageExceeded);

    // Update reserves
    bonding_curve.real_sol_reserves = bonding_curve.real_sol_reserves
        .checked_add(sol_cost)
        .ok_or(SparkBondingError::Overflow)?;

    bonding_curve.real_token_reserves = bonding_curve.real_token_reserves
        .checked_sub(token_amount)
        .ok_or(SparkBondingError::Underflow)?;

    // Update fees
    bonding_curve.platform_fees_collected = bonding_curve.platform_fees_collected
        .checked_add(platform_fee)
        .ok_or(SparkBondingError::Overflow)?;

    bonding_curve.creator_fees_collected = bonding_curve.creator_fees_collected
        .checked_add(creator_fee)
        .ok_or(SparkBondingError::Overflow)?;

    // Update analytics
    bonding_curve.total_volume_sol = bonding_curve.total_volume_sol
        .checked_add(sol_cost)
        .ok_or(SparkBondingError::Overflow)?;

    bonding_curve.total_volume_tokens = bonding_curve.total_volume_tokens
        .checked_add(token_amount)
        .ok_or(SparkBondingError::Overflow)?;

    bonding_curve.buy_count = bonding_curve.buy_count.checked_add(1).unwrap();
    bonding_curve.last_trade_at = clock.unix_timestamp;

    // Update global stats
    global.total_volume_sol = global.total_volume_sol
        .checked_add(sol_cost)
        .ok_or(SparkBondingError::Overflow)?;

    global.total_fees_collected = global.total_fees_collected
        .checked_add(platform_fee)
        .ok_or(SparkBondingError::Overflow)?;

    // Update user volume if account exists
    if let Some(user_volume) = &mut ctx.accounts.user_volume {
        user_volume.volume_sol = user_volume.volume_sol
            .checked_add(sol_cost)
            .ok_or(SparkBondingError::Overflow)?;
        user_volume.volume_tokens = user_volume.volume_tokens
            .checked_add(token_amount)
            .ok_or(SparkBondingError::Overflow)?;
        user_volume.trades_count = user_volume.trades_count.checked_add(1).unwrap();
        user_volume.last_trade_timestamp = clock.unix_timestamp;
    }

    // Check migration threshold
    if bonding_curve.is_migration_threshold_met() && !bonding_curve.migration_ready {
        bonding_curve.migration_ready = true;
        msg!("Migration threshold reached! Token ready for graduation.");
    }

    // Calculate new price
    let new_price = bonding_curve.current_price()?;

    // Calculate bonding progress (0-100)
    let bonding_progress = ((bonding_curve.real_sol_reserves as u128)
        .checked_mul(100)
        .unwrap()
        .checked_div(bonding_curve.migration_threshold as u128)
        .unwrap_or(100)) as u8;

    // Emit trade event
    emit!(TradeEvent {
        token_mint: bonding_curve.token_mint,
        trader: ctx.accounts.buyer.key(),
        is_buy: true,
        token_amount,
        sol_amount: sol_cost,
        platform_fee,
        creator_fee,
        new_price,
        sol_reserves: bonding_curve.real_sol_reserves,
        token_reserves: bonding_curve.real_token_reserves,
        timestamp: clock.unix_timestamp,
    });

    // Emit price update
    emit!(PriceUpdateEvent {
        token_mint: bonding_curve.token_mint,
        price: new_price,
        market_cap: bonding_curve.real_sol_reserves,
        volume_24h: bonding_curve.total_volume_sol,
        bonding_progress,
        timestamp: clock.unix_timestamp,
    });

    msg!("Buy: {} tokens for {} SOL (fee: {} + {})", 
        token_amount, sol_cost, platform_fee, creator_fee);

    Ok(())
}
