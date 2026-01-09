/// ============================================================================
/// SPARKCOMPANION BONDING CURVE CONSTANTS
/// ============================================================================
/// All configuration constants for the bonding curve.
/// NO admin wallets - fully decentralized and permissionless.
/// ============================================================================

/// ============================================================================
/// BONDING CURVE PARAMETERS
/// ============================================================================

/// Virtual SOL reserves for initial pricing (30 SOL)
/// Label: [PRICING] [VIRTUAL-LIQUIDITY]
pub const VIRTUAL_SOL_RESERVES: u64 = 30_000_000_000;

/// Virtual token reserves for initial pricing (1 Billion tokens)
/// Label: [PRICING] [VIRTUAL-LIQUIDITY]
pub const VIRTUAL_TOKEN_RESERVES: u64 = 1_000_000_000_000_000;

/// SOL threshold to trigger AMM migration (85 SOL)
/// Label: [MIGRATION] [THRESHOLD]
pub const MIGRATION_THRESHOLD: u64 = 85_000_000_000;

/// Total token supply (1 Billion tokens with decimals)
/// Label: [SUPPLY] [TOKENOMICS]
pub const TOTAL_SUPPLY: u64 = 1_000_000_000_000_000;

/// Percentage of tokens reserved for LP (20%)
/// Label: [LP] [TOKENOMICS]
pub const LP_RESERVE_PERCENTAGE: u64 = 20;

/// ============================================================================
/// FEE CONSTANTS (Basis Points)
/// ============================================================================
/// All fees are in basis points (1 bp = 0.01%)

/// Platform fee on trades (1% = 100 basis points)
/// Label: [FEES] [PLATFORM]
pub const PLATFORM_FEE_BASIS_POINTS: u16 = 100;

/// Creator fee on trades (1% = 100 basis points)
/// Label: [FEES] [CREATOR]
pub const CREATOR_FEE_BASIS_POINTS: u16 = 100;

/// Migration fee (5% = 500 basis points)
/// Label: [FEES] [MIGRATION]
pub const MIGRATION_FEE_BASIS_POINTS: u16 = 500;

/// Maximum allowed slippage (10% = 1000 basis points)
/// Label: [SLIPPAGE] [PROTECTION]
pub const MAX_SLIPPAGE_BASIS_POINTS: u16 = 1000;

/// Basis points denominator (100% = 10000)
/// Label: [MATH] [DENOMINATOR]
pub const BASIS_POINTS_DENOMINATOR: u64 = 10000;

/// ============================================================================
/// PDA SEEDS
/// ============================================================================
/// Seeds for Program Derived Addresses

/// Global state PDA seed
/// Label: [PDA] [GLOBAL]
pub const GLOBAL_SEED: &[u8] = b"spark_global";

/// Bonding curve PDA seed
/// Label: [PDA] [BONDING-CURVE]
pub const BONDING_CURVE_SEED: &[u8] = b"spark_bonding";

/// User volume tracker PDA seed
/// Label: [PDA] [USER-VOLUME]
pub const USER_VOLUME_SEED: &[u8] = b"spark_user_volume";

/// LP reserve PDA seed
/// Label: [PDA] [LP-RESERVE]
pub const LP_RESERVE_SEED: &[u8] = b"spark_lp_reserve";

/// SOL vault PDA seed
/// Label: [PDA] [SOL-VAULT]
pub const SOL_VAULT_SEED: &[u8] = b"spark_sol_vault";

/// Token vault PDA seed
/// Label: [PDA] [TOKEN-VAULT]
pub const TOKEN_VAULT_SEED: &[u8] = b"spark_token_vault";

/// ============================================================================
/// TOKEN DECIMALS
/// ============================================================================

/// Standard token decimals (9 decimals like SOL)
/// Label: [TOKEN] [DECIMALS]
pub const TOKEN_DECIMALS: u8 = 9;

/// ============================================================================
/// VERSION
/// ============================================================================

/// Current program version
/// Label: [VERSION] [UPGRADE]
pub const PROGRAM_VERSION: u8 = 1;
