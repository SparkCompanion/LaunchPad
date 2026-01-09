use anchor_lang::prelude::*;
use crate::{
    state::{Global, BondingCurve},
    constants::*,
    events::TokenCreatedEvent,
    errors::SparkBondingError,
};

/// ============================================================================
/// INITIALIZE BONDING CURVE CONTEXT
/// ============================================================================
/// Account validation for creating a new token bonding curve.
/// 
/// Labels: [CONTEXT] [ACCOUNTS] [TOKEN-CREATION]
#[derive(Accounts)]
#[instruction(name: String, symbol: String, uri: String)]
pub struct InitializeBondingCurve<'info> {
    /// Global state account
    /// Label: [ACCOUNT] [GLOBAL] [READ]
    #[account(
        mut,
        seeds = [GLOBAL_SEED],
        bump = global.bump
    )]
    pub global: Account<'info, Global>,

    /// Bonding curve state account (PDA)
    /// Label: [ACCOUNT] [BONDING-CURVE] [PDA]
    #[account(
        init,
        payer = creator,
        space = BondingCurve::LEN,
        seeds = [BONDING_CURVE_SEED, token_mint.key().as_ref()],
        bump
    )]
    pub bonding_curve: Account<'info, BondingCurve>,

    /// Token mint account
    /// Label: [ACCOUNT] [TOKEN] [MINT]
    /// CHECK: Validated in instruction
    pub token_mint: AccountInfo<'info>,

    /// SOL vault for the bonding curve (PDA)
    /// Label: [ACCOUNT] [VAULT] [SOL]
    /// CHECK: PDA validated by seeds
    #[account(
        seeds = [SOL_VAULT_SEED, token_mint.key().as_ref()],
        bump
    )]
    pub sol_vault: AccountInfo<'info>,

    /// Token vault for the bonding curve
    /// Label: [ACCOUNT] [VAULT] [TOKEN]
    /// CHECK: Will be initialized separately
    pub token_vault: AccountInfo<'info>,

    /// LP reserve account
    /// Label: [ACCOUNT] [LP] [RESERVE]
    /// CHECK: Will be initialized separately
    pub lp_reserve: AccountInfo<'info>,

    /// Creator (token launcher) - pays for creation
    /// Label: [ACCOUNT] [CREATOR] [SIGNER]
    #[account(mut)]
    pub creator: Signer<'info>,

    /// System program
    /// Label: [PROGRAM] [SYSTEM]
    pub system_program: Program<'info, System>,
}

/// ============================================================================
/// INITIALIZE BONDING CURVE HANDLER
/// ============================================================================
/// Creates a new bonding curve for a token launch.
/// Fully permissionless - anyone can create a token.
/// 
/// Labels: [HANDLER] [TOKEN-LAUNCH] [PERMISSIONLESS]
pub fn initialize_bonding_curve(
    ctx: Context<InitializeBondingCurve>,
    name: String,
    symbol: String,
    _uri: String,
) -> Result<()> {
    // Validate name and symbol length
    require!(name.len() <= 32, SparkBondingError::NameTooLong);
    require!(symbol.len() <= 10, SparkBondingError::SymbolTooLong);

    let global = &mut ctx.accounts.global;
    let bonding_curve = &mut ctx.accounts.bonding_curve;
    let clock = Clock::get()?;

    // Initialize bonding curve with default parameters
    bonding_curve.token_mint = ctx.accounts.token_mint.key();
    bonding_curve.creator = ctx.accounts.creator.key();
    bonding_curve.name = name.clone();
    bonding_curve.symbol = symbol.clone();
    
    // Set virtual reserves for pricing
    bonding_curve.virtual_sol_reserves = VIRTUAL_SOL_RESERVES;
    bonding_curve.virtual_token_reserves = VIRTUAL_TOKEN_RESERVES;
    
    // Initialize real reserves
    bonding_curve.real_sol_reserves = 0;
    bonding_curve.real_token_reserves = TOTAL_SUPPLY - (TOTAL_SUPPLY * LP_RESERVE_PERCENTAGE / 100);
    bonding_curve.lp_reserve_supply = TOTAL_SUPPLY * LP_RESERVE_PERCENTAGE / 100;
    
    // Set migration parameters
    bonding_curve.migration_threshold = MIGRATION_THRESHOLD;
    bonding_curve.migration_ready = false;
    bonding_curve.is_migrated = false;
    bonding_curve.amm_program_id = None;
    bonding_curve.amm_pool_address = None;
    
    // Initialize analytics
    bonding_curve.total_volume_sol = 0;
    bonding_curve.total_volume_tokens = 0;
    bonding_curve.platform_fees_collected = 0;
    bonding_curve.creator_fees_collected = 0;
    bonding_curve.buy_count = 0;
    bonding_curve.sell_count = 0;
    
    // Set timestamps
    bonding_curve.created_at = clock.unix_timestamp;
    bonding_curve.last_trade_at = 0;
    
    // Store bump seeds
    bonding_curve.bump = ctx.bumps.bonding_curve;
    bonding_curve.sol_vault_bump = ctx.bumps.sol_vault;
    bonding_curve.token_vault_bump = 0; // Set during token vault init
    bonding_curve.lp_reserve_bump = 0; // Set during LP reserve init
    bonding_curve.reserved = [0u64; 4];

    // Update global stats
    global.tokens_created = global.tokens_created.checked_add(1).unwrap();

    // Emit token created event
    emit!(TokenCreatedEvent {
        token_mint: bonding_curve.token_mint,
        creator: bonding_curve.creator,
        name,
        symbol,
        virtual_sol_reserves: bonding_curve.virtual_sol_reserves,
        virtual_token_reserves: bonding_curve.virtual_token_reserves,
        migration_threshold: bonding_curve.migration_threshold,
        timestamp: clock.unix_timestamp,
    });

    msg!("SparkCompanion Token Created: {} ({})", bonding_curve.name, bonding_curve.symbol);

    Ok(())
}
