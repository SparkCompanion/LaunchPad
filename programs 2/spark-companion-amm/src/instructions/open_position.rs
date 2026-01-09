use anchor_lang::prelude::*;
use crate::{
    state::{Pool, Position},
    constants::*,
    events::PositionCreatedEvent,
    errors::SparkAmmError,
};

/// ============================================================================
/// OPEN POSITION CONTEXT
/// ============================================================================
/// Account validation for opening a new liquidity position.
/// 
/// Labels: [CONTEXT] [ACCOUNTS] [POSITION]
#[derive(Accounts)]
#[instruction(tick_lower: i32, tick_upper: i32)]
pub struct OpenPosition<'info> {
    /// Pool state
    /// Label: [ACCOUNT] [POOL] [READ]
    #[account(
        constraint = pool.status == POOL_STATUS_INITIALIZED @ SparkAmmError::PoolNotActive
    )]
    pub pool: Account<'info, Pool>,

    /// Position state (PDA)
    /// Label: [ACCOUNT] [POSITION] [PDA]
    #[account(
        init,
        payer = owner,
        space = Position::LEN,
        seeds = [
            POSITION_SEED,
            pool.key().as_ref(),
            owner.key().as_ref(),
            &tick_lower.to_le_bytes(),
            &tick_upper.to_le_bytes()
        ],
        bump
    )]
    pub position: Account<'info, Position>,

    /// Position owner
    /// Label: [ACCOUNT] [OWNER] [SIGNER]
    #[account(mut)]
    pub owner: Signer<'info>,

    /// Position mint (NFT)
    /// Label: [ACCOUNT] [NFT] [MINT]
    /// CHECK: Will be initialized as NFT mint
    #[account(mut)]
    pub position_mint: AccountInfo<'info>,

    /// System program
    /// Label: [PROGRAM] [SYSTEM]
    pub system_program: Program<'info, System>,

    /// Token program
    /// Label: [PROGRAM] [TOKEN]
    /// CHECK: SPL Token program
    pub token_program: AccountInfo<'info>,
}

/// ============================================================================
/// OPEN POSITION HANDLER
/// ============================================================================
/// Creates a new liquidity position in a price range.
/// 
/// Labels: [HANDLER] [POSITION] [LP]
pub fn open_position(
    ctx: Context<OpenPosition>,
    tick_lower: i32,
    tick_upper: i32,
) -> Result<()> {
    let pool = &ctx.accounts.pool;
    let position = &mut ctx.accounts.position;
    let clock = Clock::get()?;

    // Validate tick range
    require!(tick_lower < tick_upper, SparkAmmError::InvalidTickRange);
    require!(tick_lower >= MIN_TICK, SparkAmmError::TickOutOfRange);
    require!(tick_upper <= MAX_TICK, SparkAmmError::TickOutOfRange);
    
    // Validate ticks align with spacing
    require!(
        tick_lower % (pool.tick_spacing as i32) == 0,
        SparkAmmError::InvalidTickRange
    );
    require!(
        tick_upper % (pool.tick_spacing as i32) == 0,
        SparkAmmError::InvalidTickRange
    );

    // Initialize position
    position.mint = ctx.accounts.position_mint.key();
    position.owner = ctx.accounts.owner.key();
    position.pool_id = pool.id;
    position.tick_lower = tick_lower;
    position.tick_upper = tick_upper;
    position.liquidity = 0;
    position.fee_growth_inside_last_a_x64 = 0;
    position.fee_growth_inside_last_b_x64 = 0;
    position.fees_owed_a = 0;
    position.fees_owed_b = 0;
    position.reward_growth_inside_last = [0u128; 3];
    position.rewards_owed = [0u64; 3];
    position.bump = ctx.bumps.position;
    position.reserved = [0u64; 4];

    // Emit position created event
    emit!(PositionCreatedEvent {
        position_mint: position.mint,
        owner: position.owner,
        pool_id: position.pool_id,
        tick_lower: position.tick_lower,
        tick_upper: position.tick_upper,
        timestamp: clock.unix_timestamp,
    });

    msg!("Position opened: ticks [{}, {}]", tick_lower, tick_upper);

    Ok(())
}
