use anchor_lang::prelude::*;
use crate::{state::AmmGlobal, constants::*, events::AmmGlobalInitializedEvent};

/// ============================================================================
/// INITIALIZE AMM GLOBAL CONTEXT
/// ============================================================================
/// Account validation for global state initialization.
/// 
/// Labels: [CONTEXT] [ACCOUNTS] [INITIALIZATION]
#[derive(Accounts)]
pub struct InitializeAmmGlobal<'info> {
    /// Global state account (PDA)
    /// Label: [ACCOUNT] [GLOBAL] [PDA]
    #[account(
        init,
        payer = payer,
        space = AmmGlobal::LEN,
        seeds = [GLOBAL_SEED],
        bump
    )]
    pub global: Account<'info, AmmGlobal>,

    /// Payer for account creation
    /// Label: [ACCOUNT] [PAYER] [SIGNER]
    #[account(mut)]
    pub payer: Signer<'info>,

    /// System program
    /// Label: [PROGRAM] [SYSTEM]
    pub system_program: Program<'info, System>,
}

/// ============================================================================
/// INITIALIZE AMM GLOBAL HANDLER
/// ============================================================================
/// Sets up the global configuration for the AMM platform.
/// This is a one-time initialization - fully permissionless.
/// 
/// Labels: [HANDLER] [INITIALIZATION] [ONE-TIME]
pub fn initialize_amm_global(ctx: Context<InitializeAmmGlobal>) -> Result<()> {
    let global = &mut ctx.accounts.global;
    let clock = Clock::get()?;

    // Initialize with default fee configuration
    // NO admin wallets - pure permissionless DeFi
    global.protocol_fee_rate = DEFAULT_PROTOCOL_FEE_RATE;
    global.fund_fee_rate = DEFAULT_FUND_FEE_RATE;
    global.default_trade_fee_rate = DEFAULT_TRADE_FEE_RATE;
    global.create_pool_fee = 0; // Free to create pools
    global.total_pools = 0;
    global.total_volume = 0;
    global.total_fees_collected = 0;
    global.version = PROGRAM_VERSION;
    global.bump = ctx.bumps.global;
    global.reserved = [0u64; 8];

    // Emit initialization event
    emit!(AmmGlobalInitializedEvent {
        protocol_fee_rate: global.protocol_fee_rate,
        default_trade_fee_rate: global.default_trade_fee_rate,
        timestamp: clock.unix_timestamp,
    });

    msg!("SparkCompanion AMM Global initialized - Permissionless CLMM");

    Ok(())
}
