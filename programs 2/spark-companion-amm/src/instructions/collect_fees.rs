use anchor_lang::prelude::*;
use crate::{
    state::{Pool, Position},
    events::FeesCollectedEvent,
    errors::SparkAmmError,
};

/// ============================================================================
/// COLLECT FEES CONTEXT
/// ============================================================================
/// Account validation for collecting position fees.
/// 
/// Labels: [CONTEXT] [ACCOUNTS] [FEES]
#[derive(Accounts)]
pub struct CollectFees<'info> {
    /// Pool state
    /// Label: [ACCOUNT] [POOL] [READ]
    pub pool: Account<'info, Pool>,

    /// Position state
    /// Label: [ACCOUNT] [POSITION] [WRITE]
    #[account(
        mut,
        constraint = position.pool_id == pool.id @ SparkAmmError::PositionNotFound,
        constraint = position.owner == owner.key() @ SparkAmmError::NotPositionOwner
    )]
    pub position: Account<'info, Position>,

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

    /// Owner's token A account
    /// Label: [ACCOUNT] [OWNER] [TOKEN-A]
    /// CHECK: Owner's token account to receive fees
    #[account(mut)]
    pub owner_token_a: AccountInfo<'info>,

    /// Owner's token B account
    /// Label: [ACCOUNT] [OWNER] [TOKEN-B]
    /// CHECK: Owner's token account to receive fees
    #[account(mut)]
    pub owner_token_b: AccountInfo<'info>,

    /// Position owner
    /// Label: [ACCOUNT] [OWNER] [SIGNER]
    pub owner: Signer<'info>,

    /// Token program
    /// Label: [PROGRAM] [TOKEN]
    /// CHECK: SPL Token program
    pub token_program: AccountInfo<'info>,
}

/// ============================================================================
/// COLLECT FEES HANDLER
/// ============================================================================
/// Collects accumulated trading fees from a position.
/// Only the position owner can collect.
/// 
/// Labels: [HANDLER] [FEES] [LP]
pub fn collect_fees(
    ctx: Context<CollectFees>,
    amount0_requested: u64,
    amount1_requested: u64,
) -> Result<()> {
    let position = &mut ctx.accounts.position;
    let clock = Clock::get()?;

    // Calculate collectable amounts
    let amount_a = position.fees_owed_a.min(amount0_requested);
    let amount_b = position.fees_owed_b.min(amount1_requested);

    // Update position fees owed
    position.fees_owed_a = position.fees_owed_a
        .checked_sub(amount_a)
        .ok_or(SparkAmmError::Underflow)?;
    position.fees_owed_b = position.fees_owed_b
        .checked_sub(amount_b)
        .ok_or(SparkAmmError::Underflow)?;

    // Emit event
    emit!(FeesCollectedEvent {
        position_mint: position.mint,
        owner: ctx.accounts.owner.key(),
        pool_id: position.pool_id,
        amount_a,
        amount_b,
        timestamp: clock.unix_timestamp,
    });

    msg!("Fees collected: {} token A, {} token B", amount_a, amount_b);

    Ok(())
}
