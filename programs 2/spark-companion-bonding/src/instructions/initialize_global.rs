use anchor_lang::prelude::*;
use crate::{state::Global, constants::*, events::GlobalInitializedEvent};

/// ============================================================================
/// INITIALIZE GLOBAL CONTEXT
/// ============================================================================
/// Account validation for global state initialization.
/// 
/// Labels: [CONTEXT] [ACCOUNTS] [INITIALIZATION]
#[derive(Accounts)]
pub struct InitializeGlobal<'info> {
    /// Global state account (PDA)
    /// Label: [ACCOUNT] [GLOBAL] [PDA]
    #[account(
        init,
        payer = payer,
        space = Global::LEN,
        seeds = [GLOBAL_SEED],
        bump
    )]
    pub global: Account<'info, Global>,

    /// Payer for account creation
    /// Label: [ACCOUNT] [PAYER] [SIGNER]
    #[account(mut)]
    pub payer: Signer<'info>,

    /// System program
    /// Label: [PROGRAM] [SYSTEM]
    pub system_program: Program<'info, System>,
}

/// ============================================================================
/// INITIALIZE GLOBAL HANDLER
/// ============================================================================
/// Sets up the global configuration for the bonding curve platform.
/// This is a one-time initialization - fully permissionless.
/// 
/// Labels: [HANDLER] [INITIALIZATION] [ONE-TIME]
pub fn initialize_global(ctx: Context<InitializeGlobal>) -> Result<()> {
    let global = &mut ctx.accounts.global;
    let clock = Clock::get()?;

    // Initialize with default fee configuration
    // NO admin wallets - fees go to protocol and creators only
    global.platform_fee_basis_points = PLATFORM_FEE_BASIS_POINTS;
    global.creator_fee_basis_points = CREATOR_FEE_BASIS_POINTS;
    global.migration_fee_basis_points = MIGRATION_FEE_BASIS_POINTS;
    global.max_slippage_basis_points = MAX_SLIPPAGE_BASIS_POINTS;
    global.migration_enabled = true;
    global.total_volume_sol = 0;
    global.total_fees_collected = 0;
    global.tokens_created = 0;
    global.successful_migrations = 0;
    global.version = PROGRAM_VERSION;
    global.bump = ctx.bumps.global;
    global.reserved = [0u64; 8];

    // Emit initialization event
    emit!(GlobalInitializedEvent {
        platform_fee: global.platform_fee_basis_points,
        creator_fee: global.creator_fee_basis_points,
        migration_fee: global.migration_fee_basis_points,
        migration_enabled: global.migration_enabled,
        timestamp: clock.unix_timestamp,
    });

    msg!("SparkCompanion Global initialized - Permissionless DeFi");

    Ok(())
}
