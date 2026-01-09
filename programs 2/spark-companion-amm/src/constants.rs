/// ============================================================================
/// SPARKCOMPANION AMM CONSTANTS
/// ============================================================================
/// All configuration constants for the concentrated liquidity AMM.
/// NO admin wallets - fully decentralized and permissionless.
/// ============================================================================

/// ============================================================================
/// PRICE RANGE CONSTANTS (Q64.64 Fixed Point)
/// ============================================================================

/// Minimum sqrt price (Q64.64)
/// Label: [PRICE] [MINIMUM]
pub const MIN_SQRT_PRICE_X64: u128 = 4295048016;

/// Maximum sqrt price (Q64.64)
/// Label: [PRICE] [MAXIMUM]
pub const MAX_SQRT_PRICE_X64: u128 = 79226673515401279992447579055;

/// ============================================================================
/// TICK CONSTANTS
/// ============================================================================

/// Minimum tick index
/// Label: [TICK] [MINIMUM]
pub const MIN_TICK: i32 = -443636;

/// Maximum tick index
/// Label: [TICK] [MAXIMUM]
pub const MAX_TICK: i32 = 443636;

/// Number of ticks per tick array
/// Label: [TICK] [ARRAY-SIZE]
pub const TICK_ARRAY_SIZE: i32 = 88;

/// Standard tick spacings for different fee tiers
/// Label: [TICK] [SPACING]
pub const TICK_SPACING_10: u16 = 10;   // 0.05% fee tier
pub const TICK_SPACING_60: u16 = 60;   // 0.30% fee tier
pub const TICK_SPACING_200: u16 = 200; // 1.00% fee tier

/// ============================================================================
/// FEE CONSTANTS
/// ============================================================================

/// Fee rate denominator (1,000,000 = 100%)
/// Label: [FEES] [DENOMINATOR]
pub const FEE_RATE_DENOMINATOR_VALUE: u64 = 1000000;

/// Protocol fee multiplier
/// Label: [FEES] [PROTOCOL]
pub const PROTOCOL_FEE_RATE_MUL_VALUE: u64 = 12000;

/// Fund fee multiplier
/// Label: [FEES] [FUND]
pub const FUND_FEE_RATE_MUL_VALUE: u64 = 25000;

/// Default protocol fee rate (1.2%)
/// Label: [FEES] [DEFAULT]
pub const DEFAULT_PROTOCOL_FEE_RATE: u32 = 120;

/// Default trade fee rate (0.25%)
/// Label: [FEES] [DEFAULT]
pub const DEFAULT_TRADE_FEE_RATE: u32 = 2500;

/// Default fund fee rate (4%)
/// Label: [FEES] [DEFAULT]
pub const DEFAULT_FUND_FEE_RATE: u32 = 40000;

/// Basis points denominator
/// Label: [MATH] [DENOMINATOR]
pub const BASIS_POINTS_DENOMINATOR: u64 = 10000;

/// ============================================================================
/// LIQUIDITY CONSTANTS
/// ============================================================================

/// Minimum liquidity for a position
/// Label: [LIQUIDITY] [MINIMUM]
pub const MIN_LIQUIDITY: u128 = 100000;

/// Q64 fixed point multiplier (2^64)
/// Label: [MATH] [FIXED-POINT]
pub const Q64: u128 = 1 << 64;

/// Q128 fixed point multiplier (2^127 max for u128)
/// Label: [MATH] [FIXED-POINT]
pub const Q128: u128 = 1u128 << 127;

/// ============================================================================
/// PDA SEEDS
/// ============================================================================

/// Global state PDA seed
/// Label: [PDA] [GLOBAL]
pub const GLOBAL_SEED: &[u8] = b"spark_amm_global";

/// Pool PDA seed
/// Label: [PDA] [POOL]
pub const POOL_SEED: &[u8] = b"spark_pool";

/// Position PDA seed
/// Label: [PDA] [POSITION]
pub const POSITION_SEED: &[u8] = b"spark_position";

/// Tick array PDA seed
/// Label: [PDA] [TICK]
pub const TICK_ARRAY_SEED: &[u8] = b"spark_tick_array";

/// Pool vault PDA seed
/// Label: [PDA] [VAULT]
pub const POOL_VAULT_SEED: &[u8] = b"spark_pool_vault";

/// Pool reward vault PDA seed
/// Label: [PDA] [REWARD]
pub const POOL_REWARD_VAULT_SEED: &[u8] = b"spark_pool_reward_vault";

/// Personal position PDA seed
/// Label: [PDA] [PERSONAL]
pub const PERSONAL_POSITION_SEED: &[u8] = b"spark_personal_position";

/// Observation state PDA seed
/// Label: [PDA] [ORACLE]
pub const OBSERVATION_STATE_SEED: &[u8] = b"spark_observation_state";

/// ============================================================================
/// POOL STATUS CONSTANTS
/// ============================================================================

/// Pool is initialized and active
/// Label: [STATUS] [ACTIVE]
pub const POOL_STATUS_INITIALIZED: u8 = 1;

/// Pool is disabled (no trades)
/// Label: [STATUS] [DISABLED]
pub const POOL_STATUS_DISABLED: u8 = 2;

/// Pool allows withdrawals only
/// Label: [STATUS] [WITHDRAW-ONLY]
pub const POOL_STATUS_WITHDRAW_ONLY: u8 = 3;

/// Pool allows swaps only
/// Label: [STATUS] [SWAP-ONLY]
pub const POOL_STATUS_SWAP_ONLY: u8 = 4;

/// ============================================================================
/// REWARD CONSTANTS
/// ============================================================================

/// Maximum number of reward tokens per pool
/// Label: [REWARDS] [MAXIMUM]
pub const REWARD_NUM: usize = 3;

/// Reward PDA seed
/// Label: [PDA] [REWARD]
pub const REWARD_SEED: &[u8] = b"spark_reward";

/// ============================================================================
/// ORACLE CONSTANTS
/// ============================================================================

/// Default observation update duration (15 seconds)
/// Label: [ORACLE] [DURATION]
pub const OBSERVATION_UPDATE_DURATION_DEFAULT: u32 = 15;

/// ============================================================================
/// VERSION
/// ============================================================================

/// Current program version
/// Label: [VERSION] [UPGRADE]
pub const PROGRAM_VERSION: u8 = 1;
