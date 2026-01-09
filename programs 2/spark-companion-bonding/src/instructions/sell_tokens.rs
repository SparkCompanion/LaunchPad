use anchor_lang::prelude::*;
use crate::{
    state::{Global, BondingCurve, UserVolumeAccumulator},
    constants::*,
    events::{TradeEvent, PriceUpdateEvent},
    errors::SparkBondingError,
};

/// ============================================================================
/// SELL TOKENS CONTEXT
/// ============================================================================
/// Account validation for selling tokens back to the bonding curve.
/// 
/// Labels: [CONTEXT] [ACCOUNTS] [SELL]
#[derive(Accounts)]
pub struct SellTokens<'info> {
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

    /// User's volume tracker (optional)
    /// Label: [ACCOUNT] [USER] [VOLUME]
    #[account(
        mut,
        seeds = [USER_VOLUME_SEED, seller.key().as_ref()],
        bump = user_volume.bump
    )]
    pub user_volume: Option<Account<'info, UserVolumeAccumulator>>,

    /// SOL vault to pay seller from
    /// Label: [ACCOUNT] [VAULT] [SOL]
    /// CHECK: PDA validated by seeds
    #[account(
        mut,
        seeds = [SOL_VAULT_SEED, bonding_curve.token_mint.as_ref()],
        bump = bonding_curve.sol_vault_bump
    )]
    pub sol_vault: AccountInfo<'info>,

    /// Token vault to receive tokens
    /// Label: [ACCOUNT] [VAULT] [TOKEN]
    /// CHECK: Associated token account
    #[account(mut)]
    pub token_vault: AccountInfo<'info>,

    /// Seller's token account
    /// Label: [ACCOUNT] [SELLER] [TOKEN]
    /// CHECK: Associated token account
    #[account(mut)]
    pub seller_token_account: AccountInfo<'info>,

    /// Seller wallet
    /// Label: [ACCOUNT] [SELLER] [SIGNER]
    #[account(mut)]
    pub seller: Signer<'info>,

    /// Token program
    /// Label: [PROGRAM] [TOKEN]
    /// CHECK: SPL Token program
    pub token_program: AccountInfo<'info>,

    /// System program
    /// Label: [PROGRAM] [SYSTEM]
    pub system_program: Program<'info, System>,
}

/// ============================================================================
/// SELL TOKENS HANDLER
/// ============================================================================
/// Sells tokens back to the bonding curve for SOL.
/// Uses constant product formula: x * y = k
/// 
/// Labels: [HANDLER] [TRADE] [SELL]
pub fn sell_tokens(
    ctx: Context<SellTokens>,
    token_amount: u64,
    min_sol_received: u64,
) -> Result<()> {
    let bonding_curve = &mut ctx.accounts.bonding_curve;
    let global = &mut ctx.accounts.global;
    let clock = Clock::get()?;

    // Validate trade
    bonding_curve.validate_trade_amounts(token_amount, false)?;

    // Calculate SOL received using constant product formula
    let total_sol = bonding_curve.virtual_sol_reserves
        .checked_add(bonding_curve.real_sol_reserves)
        .ok_or(SparkBondingError::Overflow)?;

    let total_tokens = bonding_curve.virtual_token_reserves
        .checked_sub(bonding_curve.real_token_reserves)
        .ok_or(SparkBondingError::Underflow)?;

    // Calculate new token balance after adding back sold tokens
    let new_total_tokens = total_tokens
        .checked_sub(token_amount)
        .ok_or(SparkBondingError::Underflow)?;

    require!(new_total_tokens > 0, SparkBondingError::InvalidTokenAmount);

    // k = total_sol * total_tokens
    let k = (total_sol as u128)
        .checked_mul(total_tokens as u128)
        .ok_or(SparkBondingError::Overflow)?;

    let new_sol_reserves = k
        .checked_div(new_total_tokens as u128)
        .ok_or(SparkBondingError::DivisionByZero)? as u64;

    let sol_received_gross = new_sol_reserves
        .checked_sub(total_sol)
        .ok_or(SparkBondingError::Underflow)?;

    // Calculate fees
    let platform_fee = sol_received_gross
        .checked_mul(global.platform_fee_basis_points as u64)
        .ok_or(SparkBondingError::Overflow)?
        .checked_div(BASIS_POINTS_DENOMINATOR)
        .ok_or(SparkBondingError::DivisionByZero)?;

    let creator_fee = sol_received_gross
        .checked_mul(global.creator_fee_basis_points as u64)
        .ok_or(SparkBondingError::Overflow)?
        .checked_div(BASIS_POINTS_DENOMINATOR)
        .ok_or(SparkBondingError::DivisionByZero)?;

    let sol_received_net = sol_received_gross
        .checked_sub(platform_fee)
        .ok_or(SparkBondingError::Underflow)?
        .checked_sub(creator_fee)
        .ok_or(SparkBondingError::Underflow)?;

    // Check slippage
    require!(sol_received_net >= min_sol_received, SparkBondingError::SlippageExceeded);

    // Verify sufficient SOL in reserves
    require!(
        bonding_curve.real_sol_reserves >= sol_received_gross,
        SparkBondingError::InsufficientSolReserves
    );

    // Update reserves
    bonding_curve.real_sol_reserves = bonding_curve.real_sol_reserves
        .checked_sub(sol_received_gross)
        .ok_or(SparkBondingError::Underflow)?;

    bonding_curve.real_token_reserves = bonding_curve.real_token_reserves
        .checked_add(token_amount)
        .ok_or(SparkBondingError::Overflow)?;

    // Update fees
    bonding_curve.platform_fees_collected = bonding_curve.platform_fees_collected
        .checked_add(platform_fee)
        .ok_or(SparkBondingError::Overflow)?;

    bonding_curve.creator_fees_collected = bonding_curve.creator_fees_collected
        .checked_add(creator_fee)
        .ok_or(SparkBondingError::Overflow)?;

    // Update analytics
    bonding_curve.total_volume_sol = bonding_curve.total_volume_sol
        .checked_add(sol_received_gross)
        .ok_or(SparkBondingError::Overflow)?;

    bonding_curve.total_volume_tokens = bonding_curve.total_volume_tokens
        .checked_add(token_amount)
        .ok_or(SparkBondingError::Overflow)?;

    bonding_curve.sell_count = bonding_curve.sell_count.checked_add(1).unwrap();
    bonding_curve.last_trade_at = clock.unix_timestamp;

    // Update global stats
    global.total_volume_sol = global.total_volume_sol
        .checked_add(sol_received_gross)
        .ok_or(SparkBondingError::Overflow)?;

    global.total_fees_collected = global.total_fees_collected
        .checked_add(platform_fee)
        .ok_or(SparkBondingError::Overflow)?;

    // Update user volume if account exists
    if let Some(user_volume) = &mut ctx.accounts.user_volume {
        user_volume.volume_sol = user_volume.volume_sol
            .checked_add(sol_received_gross)
            .ok_or(SparkBondingError::Overflow)?;
        user_volume.volume_tokens = user_volume.volume_tokens
            .checked_add(token_amount)
            .ok_or(SparkBondingError::Overflow)?;
        user_volume.trades_count = user_volume.trades_count.checked_add(1).unwrap();
        user_volume.last_trade_timestamp = clock.unix_timestamp;
    }

    // Calculate new price
    let new_price = bonding_curve.current_price()?;

    // Calculate bonding progress
    let bonding_progress = ((bonding_curve.real_sol_reserves as u128)
        .checked_mul(100)
        .unwrap()
        .checked_div(bonding_curve.migration_threshold as u128)
        .unwrap_or(0)) as u8;

    // Emit trade event
    emit!(TradeEvent {
        token_mint: bonding_curve.token_mint,
        trader: ctx.accounts.seller.key(),
        is_buy: false,
        token_amount,
        sol_amount: sol_received_net,
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

    msg!("Sell: {} tokens for {} SOL (fee: {} + {})", 
        token_amount, sol_received_net, platform_fee, creator_fee);

    Ok(())
}
