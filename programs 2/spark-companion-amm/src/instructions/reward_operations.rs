use anchor_lang::prelude::*;
use crate::{
    state::{Pool, RewardInfo},
    constants::*,
    events::RewardInitializedEvent,
    errors::SparkAmmError,
};

/// ============================================================================
/// INITIALIZE REWARD CONTEXT
/// ============================================================================
/// Account validation for setting up pool rewards.
/// 
/// Labels: [CONTEXT] [ACCOUNTS] [REWARDS]
#[derive(Accounts)]
#[instruction(reward_index: u8)]
pub struct InitializeReward<'info> {
    /// Pool state
    /// Label: [ACCOUNT] [POOL] [WRITE]
    #[account(mut)]
    pub pool: Account<'info, Pool>,

    /// Reward token mint
    /// Label: [ACCOUNT] [REWARD] [MINT]
    /// CHECK: Validated as token mint
    pub reward_mint: AccountInfo<'info>,

    /// Reward vault
    /// Label: [ACCOUNT] [REWARD] [VAULT]
    /// CHECK: Will be initialized as token account
    #[account(
        seeds = [POOL_REWARD_VAULT_SEED, pool.key().as_ref(), &[reward_index]],
        bump
    )]
    pub reward_vault: AccountInfo<'info>,

    /// Reward authority (can set emissions)
    /// Label: [ACCOUNT] [AUTHORITY] [SIGNER]
    #[account(mut)]
    pub authority: Signer<'info>,

    /// System program
    /// Label: [PROGRAM] [SYSTEM]
    pub system_program: Program<'info, System>,

    /// Token program
    /// Label: [PROGRAM] [TOKEN]
    /// CHECK: SPL Token program
    pub token_program: AccountInfo<'info>,
}

/// ============================================================================
/// SET POOL REWARD CONTEXT
/// ============================================================================
/// Account validation for updating reward emissions.
/// 
/// Labels: [CONTEXT] [ACCOUNTS] [REWARDS]
#[derive(Accounts)]
#[instruction(reward_index: u8)]
pub struct SetPoolReward<'info> {
    /// Pool state
    /// Label: [ACCOUNT] [POOL] [WRITE]
    #[account(mut)]
    pub pool: Account<'info, Pool>,

    /// Reward authority
    /// Label: [ACCOUNT] [AUTHORITY] [SIGNER]
    #[account(
        constraint = authority.key() == pool.reward_infos[reward_index as usize].authority
            @ SparkAmmError::NotRewardAuthority
    )]
    pub authority: Signer<'info>,
}

/// ============================================================================
/// INITIALIZE REWARD HANDLER
/// ============================================================================
/// Sets up a reward token for liquidity mining.
/// 
/// Labels: [HANDLER] [REWARDS] [INITIALIZATION]
pub fn initialize_reward(
    ctx: Context<InitializeReward>,
    reward_index: u8,
) -> Result<()> {
    let pool = &mut ctx.accounts.pool;
    let clock = Clock::get()?;

    // Validate reward index
    require!(
        (reward_index as usize) < REWARD_NUM,
        SparkAmmError::InvalidRewardIndex
    );

    // Check reward slot is not already initialized
    require!(
        pool.reward_infos[reward_index as usize].mint == Pubkey::default(),
        SparkAmmError::RewardAlreadyInitialized
    );

    // Initialize reward info
    pool.reward_infos[reward_index as usize] = RewardInfo {
        mint: ctx.accounts.reward_mint.key(),
        vault: ctx.accounts.reward_vault.key(),
        authority: ctx.accounts.authority.key(),
        emissions_per_second_x64: 0,
        growth_global_x64: 0,
        last_update_time: clock.unix_timestamp as u64,
        total_amount_owed: 0,
    };

    // Emit event
    emit!(RewardInitializedEvent {
        pool_id: pool.id,
        reward_index,
        reward_mint: ctx.accounts.reward_mint.key(),
        authority: ctx.accounts.authority.key(),
        timestamp: clock.unix_timestamp,
    });

    msg!("Reward {} initialized for pool", reward_index);

    Ok(())
}

/// ============================================================================
/// SET POOL REWARD HANDLER
/// ============================================================================
/// Updates reward emissions rate.
/// Only the reward authority can update.
/// 
/// Labels: [HANDLER] [REWARDS] [UPDATE]
pub fn set_pool_reward(
    ctx: Context<SetPoolReward>,
    reward_index: u8,
    emissions_per_second_x64: u128,
) -> Result<()> {
    let pool = &mut ctx.accounts.pool;
    let clock = Clock::get()?;

    // Validate reward index
    require!(
        (reward_index as usize) < REWARD_NUM,
        SparkAmmError::InvalidRewardIndex
    );

    // Update emissions
    let reward_info = &mut pool.reward_infos[reward_index as usize];
    reward_info.emissions_per_second_x64 = emissions_per_second_x64;
    reward_info.last_update_time = clock.unix_timestamp as u64;

    msg!("Reward {} emissions updated: {}", reward_index, emissions_per_second_x64);

    Ok(())
}
