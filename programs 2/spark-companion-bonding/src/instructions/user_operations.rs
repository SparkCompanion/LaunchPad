use anchor_lang::prelude::*;
use crate::{state::UserVolumeAccumulator, constants::*};

/// ============================================================================
/// INIT USER VOLUME ACCUMULATOR CONTEXT
/// ============================================================================
/// Account validation for creating user's volume tracker.
/// 
/// Labels: [CONTEXT] [ACCOUNTS] [USER]
#[derive(Accounts)]
pub struct InitUserVolumeAccumulator<'info> {
    /// User volume tracker account (PDA)
    /// Label: [ACCOUNT] [USER] [VOLUME]
    #[account(
        init,
        payer = user,
        space = UserVolumeAccumulator::LEN,
        seeds = [USER_VOLUME_SEED, user.key().as_ref()],
        bump
    )]
    pub user_volume: Account<'info, UserVolumeAccumulator>,

    /// User wallet
    /// Label: [ACCOUNT] [USER] [SIGNER]
    #[account(mut)]
    pub user: Signer<'info>,

    /// System program
    /// Label: [PROGRAM] [SYSTEM]
    pub system_program: Program<'info, System>,
}

/// ============================================================================
/// INIT USER VOLUME ACCUMULATOR HANDLER
/// ============================================================================
/// Creates a tracking account for user's trading activity.
/// Optional - used for analytics and potential rewards.
/// 
/// Labels: [HANDLER] [USER] [ANALYTICS]
pub fn init_user_volume_accumulator(ctx: Context<InitUserVolumeAccumulator>) -> Result<()> {
    let user_volume = &mut ctx.accounts.user_volume;
    let clock = Clock::get()?;

    user_volume.user = ctx.accounts.user.key();
    user_volume.volume_sol = 0;
    user_volume.volume_tokens = 0;
    user_volume.trades_count = 0;
    user_volume.last_trade_timestamp = clock.unix_timestamp;
    user_volume.bump = ctx.bumps.user_volume;
    user_volume.reserved = [0u64; 2];

    msg!("User volume tracker initialized for: {}", ctx.accounts.user.key());

    Ok(())
}
