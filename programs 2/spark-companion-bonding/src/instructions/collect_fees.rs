use anchor_lang::prelude::*;
use crate::{
    state::{Global, BondingCurve},
    constants::*,
    events::CreatorFeesCollectedEvent,
    errors::SparkBondingError,
};

/// ============================================================================
/// COLLECT CREATOR FEES CONTEXT
/// ============================================================================
/// Account validation for creator fee collection.
/// NO admin or multisig required - only the original creator.
/// 
/// Labels: [CONTEXT] [ACCOUNTS] [FEES]
#[derive(Accounts)]
pub struct CollectCreatorFees<'info> {
    /// Global state account
    /// Label: [ACCOUNT] [GLOBAL] [READ]
    #[account(
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

    /// SOL vault to withdraw fees from
    /// Label: [ACCOUNT] [VAULT] [SOL]
    /// CHECK: PDA validated by seeds
    #[account(
        mut,
        seeds = [SOL_VAULT_SEED, bonding_curve.token_mint.as_ref()],
        bump = bonding_curve.sol_vault_bump
    )]
    pub sol_vault: AccountInfo<'info>,

    /// Original token creator (only they can collect)
    /// Label: [ACCOUNT] [CREATOR] [SIGNER]
    #[account(
        mut,
        constraint = creator.key() == bonding_curve.creator 
            @ SparkBondingError::UnauthorizedCreator
    )]
    pub creator: Signer<'info>,

    /// Destination for fees (usually creator's wallet)
    /// Label: [ACCOUNT] [DESTINATION] [FEES]
    /// CHECK: Creator's designated destination
    #[account(mut)]
    pub fee_destination: AccountInfo<'info>,

    /// System program
    /// Label: [PROGRAM] [SYSTEM]
    pub system_program: Program<'info, System>,
}

/// ============================================================================
/// COLLECT CREATOR FEES HANDLER
/// ============================================================================
/// Allows the token creator to collect their accumulated trading fees.
/// Only the original creator can call this - no admin required.
/// 
/// Labels: [HANDLER] [FEES] [CREATOR-ONLY]
pub fn collect_creator_fees(ctx: Context<CollectCreatorFees>, amount: u64) -> Result<()> {
    let bonding_curve = &mut ctx.accounts.bonding_curve;
    let clock = Clock::get()?;

    // Verify sufficient fees available
    require!(
        amount <= bonding_curve.creator_fees_collected,
        SparkBondingError::InsufficientFees
    );

    // Update fee tracking
    bonding_curve.creator_fees_collected = bonding_curve.creator_fees_collected
        .checked_sub(amount)
        .ok_or(SparkBondingError::Underflow)?;

    // Transfer fees from vault to destination
    // Note: Actual transfer would use CPI to transfer lamports
    // This is the state update portion

    // Emit event
    emit!(CreatorFeesCollectedEvent {
        token_mint: bonding_curve.token_mint,
        creator: ctx.accounts.creator.key(),
        amount,
        destination: ctx.accounts.fee_destination.key(),
        timestamp: clock.unix_timestamp,
    });

    msg!("Creator fees collected: {} lamports", amount);
    msg!("  Token: {} ({})", bonding_curve.name, bonding_curve.symbol);
    msg!("  Destination: {}", ctx.accounts.fee_destination.key());

    Ok(())
}
