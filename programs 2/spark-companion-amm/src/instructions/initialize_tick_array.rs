use anchor_lang::prelude::*;
use crate::{
    state::{Pool, TickArray, Tick},
    constants::*,
    errors::SparkAmmError,
};

/// ============================================================================
/// INITIALIZE TICK ARRAY CONTEXT
/// ============================================================================
/// Account validation for initializing a tick array.
/// 
/// Labels: [CONTEXT] [ACCOUNTS] [TICK]
#[derive(Accounts)]
#[instruction(start_tick_index: i32)]
pub struct InitializeTickArray<'info> {
    /// Pool state
    /// Label: [ACCOUNT] [POOL] [READ]
    pub pool: Account<'info, Pool>,

    /// Tick array account (PDA)
    /// Label: [ACCOUNT] [TICK-ARRAY] [PDA]
    #[account(
        init,
        payer = payer,
        space = TickArray::LEN,
        seeds = [
            TICK_ARRAY_SEED,
            pool.key().as_ref(),
            &start_tick_index.to_le_bytes()
        ],
        bump
    )]
    pub tick_array: Account<'info, TickArray>,

    /// Payer for account creation
    /// Label: [ACCOUNT] [PAYER] [SIGNER]
    #[account(mut)]
    pub payer: Signer<'info>,

    /// System program
    /// Label: [PROGRAM] [SYSTEM]
    pub system_program: Program<'info, System>,
}

/// ============================================================================
/// INITIALIZE TICK ARRAY HANDLER
/// ============================================================================
/// Initializes a tick array for storing tick data.
/// 
/// Labels: [HANDLER] [TICK] [INITIALIZATION]
pub fn initialize_tick_array(
    ctx: Context<InitializeTickArray>,
    start_tick_index: i32,
) -> Result<()> {
    let pool = &ctx.accounts.pool;
    let tick_array = &mut ctx.accounts.tick_array;

    // Validate start tick is aligned to array boundaries
    let ticks_per_array = TICK_ARRAY_SIZE * (pool.tick_spacing as i32);
    require!(
        start_tick_index % ticks_per_array == 0,
        SparkAmmError::InvalidTickRange
    );

    // Validate tick is in valid range
    require!(
        start_tick_index >= MIN_TICK && start_tick_index <= MAX_TICK,
        SparkAmmError::TickOutOfRange
    );

    // Initialize tick array
    tick_array.start_tick_index = start_tick_index;
    tick_array.ticks = [Tick::default(); 88];
    tick_array.initialized_tick_count = 0;
    tick_array.pool_id = pool.id;
    tick_array.bump = ctx.bumps.tick_array;

    msg!("Tick array initialized at index: {}", start_tick_index);

    Ok(())
}
