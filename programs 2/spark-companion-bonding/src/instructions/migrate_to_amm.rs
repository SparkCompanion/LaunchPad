use anchor_lang::prelude::*;
use crate::{
    state::{Global, BondingCurve},
    constants::*,
    events::MigrationEvent,
    errors::SparkBondingError,
};

/// ============================================================================
/// MIGRATE TO AMM CONTEXT
/// ============================================================================
/// Account validation for migrating bonding curve to AMM.
/// 
/// Labels: [CONTEXT] [ACCOUNTS] [MIGRATION]
#[derive(Accounts)]
pub struct MigrateToAmm<'info> {
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

    /// Migrator (anyone can trigger once threshold met)
    /// Label: [ACCOUNT] [MIGRATOR] [SIGNER]
    #[account(mut)]
    pub migrator: Signer<'info>,

    /// AMM program for migration
    /// Label: [PROGRAM] [AMM]
    /// CHECK: Will be validated during CPI
    pub amm_program: AccountInfo<'info>,

    /// AMM pool account to create
    /// Label: [ACCOUNT] [AMM] [POOL]
    /// CHECK: Will be initialized by AMM program
    #[account(mut)]
    pub amm_pool: AccountInfo<'info>,

    /// System program
    /// Label: [PROGRAM] [SYSTEM]
    pub system_program: Program<'info, System>,
}

/// ============================================================================
/// MIGRATE TO AMM HANDLER
/// ============================================================================
/// Migrates a bonding curve to a full AMM pool.
/// Fully permissionless - anyone can trigger once threshold is reached.
/// This is the "graduation" moment for the token.
/// 
/// Labels: [HANDLER] [MIGRATION] [PERMISSIONLESS] [GRADUATION]
pub fn migrate_to_amm(ctx: Context<MigrateToAmm>) -> Result<()> {
    let global = &mut ctx.accounts.global;
    let bonding_curve = &mut ctx.accounts.bonding_curve;
    let clock = Clock::get()?;

    // Verify migration is enabled
    require!(global.migration_enabled, SparkBondingError::MigrationDisabled);

    // Verify not already migrated
    require!(!bonding_curve.is_migrated, SparkBondingError::AlreadyMigrated);

    // Verify threshold is met
    require!(
        bonding_curve.is_migration_threshold_met(),
        SparkBondingError::MigrationThresholdNotReached
    );

    // Calculate migration fee
    let migration_fee = bonding_curve.real_sol_reserves
        .checked_mul(global.migration_fee_basis_points as u64)
        .ok_or(SparkBondingError::Overflow)?
        .checked_div(BASIS_POINTS_DENOMINATOR)
        .ok_or(SparkBondingError::DivisionByZero)?;

    let sol_for_amm = bonding_curve.real_sol_reserves
        .checked_sub(migration_fee)
        .ok_or(SparkBondingError::Underflow)?;

    // Mark as migrated
    bonding_curve.is_migrated = true;
    bonding_curve.amm_program_id = Some(ctx.accounts.amm_program.key());
    bonding_curve.amm_pool_address = Some(ctx.accounts.amm_pool.key());

    // Update global stats
    global.successful_migrations = global.successful_migrations.checked_add(1).unwrap();

    // Emit migration event
    emit!(MigrationEvent {
        token_mint: bonding_curve.token_mint,
        creator: bonding_curve.creator,
        amm_program_id: ctx.accounts.amm_program.key(),
        amm_pool_address: ctx.accounts.amm_pool.key(),
        final_sol_reserves: sol_for_amm,
        final_token_reserves: bonding_curve.lp_reserve_supply,
        lp_tokens_allocated: bonding_curve.lp_reserve_supply,
        timestamp: clock.unix_timestamp,
    });

    msg!("Token GRADUATED to AMM!");
    msg!("  Token: {} ({})", bonding_curve.name, bonding_curve.symbol);
    msg!("  SOL for AMM: {}", sol_for_amm);
    msg!("  LP Tokens: {}", bonding_curve.lp_reserve_supply);
    msg!("  Migration Fee: {}", migration_fee);

    Ok(())
}
