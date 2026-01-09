use anchor_lang::prelude::*;

/// ============================================================================
/// SPARKCOMPANION AMM ERRORS
/// ============================================================================
/// All error codes for the AMM program.
/// 
/// Labels: [ERRORS] [VALIDATION] [SECURITY]
#[error_code]
pub enum SparkAmmError {
    /// ========================================================================
    /// MATH ERRORS
    /// ========================================================================

    /// Arithmetic overflow occurred
    /// Label: [MATH] [OVERFLOW]
    #[msg("Arithmetic overflow")]
    Overflow,

    /// Arithmetic underflow occurred
    /// Label: [MATH] [UNDERFLOW]
    #[msg("Arithmetic underflow")]
    Underflow,

    /// Division by zero attempted
    /// Label: [MATH] [DIVISION]
    #[msg("Division by zero")]
    DivisionByZero,

    /// ========================================================================
    /// POOL ERRORS
    /// ========================================================================

    /// Pool already exists for this token pair
    /// Label: [POOL] [EXISTS]
    #[msg("Pool already exists")]
    PoolAlreadyExists,

    /// Pool not found
    /// Label: [POOL] [NOT-FOUND]
    #[msg("Pool not found")]
    PoolNotFound,

    /// Pool is not active
    /// Label: [POOL] [INACTIVE]
    #[msg("Pool is not active")]
    PoolNotActive,

    /// Invalid tick spacing
    /// Label: [POOL] [TICK-SPACING]
    #[msg("Invalid tick spacing")]
    InvalidTickSpacing,

    /// ========================================================================
    /// PRICE ERRORS
    /// ========================================================================

    /// Price is out of valid range
    /// Label: [PRICE] [OUT-OF-RANGE]
    #[msg("Price out of range")]
    PriceOutOfRange,

    /// Sqrt price limit exceeded
    /// Label: [PRICE] [LIMIT]
    #[msg("Sqrt price limit exceeded")]
    SqrtPriceLimitExceeded,

    /// ========================================================================
    /// TICK ERRORS
    /// ========================================================================

    /// Tick is out of valid range
    /// Label: [TICK] [OUT-OF-RANGE]
    #[msg("Tick out of range")]
    TickOutOfRange,

    /// Tick array not found
    /// Label: [TICK] [ARRAY-NOT-FOUND]
    #[msg("Tick array not found")]
    TickArrayNotFound,

    /// Invalid tick range (lower >= upper)
    /// Label: [TICK] [INVALID-RANGE]
    #[msg("Invalid tick range")]
    InvalidTickRange,

    /// ========================================================================
    /// LIQUIDITY ERRORS
    /// ========================================================================

    /// Liquidity amount too low
    /// Label: [LIQUIDITY] [TOO-LOW]
    #[msg("Liquidity too low")]
    LiquidityTooLow,

    /// Insufficient liquidity for operation
    /// Label: [LIQUIDITY] [INSUFFICIENT]
    #[msg("Insufficient liquidity")]
    InsufficientLiquidity,

    /// ========================================================================
    /// POSITION ERRORS
    /// ========================================================================

    /// Position not found
    /// Label: [POSITION] [NOT-FOUND]
    #[msg("Position not found")]
    PositionNotFound,

    /// Not the position owner
    /// Label: [POSITION] [UNAUTHORIZED]
    #[msg("Not position owner")]
    NotPositionOwner,

    /// Position has no liquidity
    /// Label: [POSITION] [EMPTY]
    #[msg("Position is empty")]
    PositionEmpty,

    /// ========================================================================
    /// SWAP ERRORS
    /// ========================================================================

    /// Slippage tolerance exceeded
    /// Label: [SWAP] [SLIPPAGE]
    #[msg("Slippage exceeded")]
    SlippageExceeded,

    /// Zero amount provided for swap
    /// Label: [SWAP] [ZERO-AMOUNT]
    #[msg("Zero amount")]
    ZeroAmount,

    /// ========================================================================
    /// FEE ERRORS
    /// ========================================================================

    /// Fee rate too high
    /// Label: [FEES] [TOO-HIGH]
    #[msg("Fee rate too high")]
    FeeTooHigh,

    /// Insufficient fees to collect
    /// Label: [FEES] [INSUFFICIENT]
    #[msg("Insufficient fees")]
    InsufficientFees,

    /// ========================================================================
    /// REWARD ERRORS
    /// ========================================================================

    /// Invalid reward index
    /// Label: [REWARDS] [INVALID-INDEX]
    #[msg("Invalid reward index")]
    InvalidRewardIndex,

    /// Reward already initialized
    /// Label: [REWARDS] [ALREADY-INIT]
    #[msg("Reward already initialized")]
    RewardAlreadyInitialized,

    /// Not reward authority
    /// Label: [REWARDS] [UNAUTHORIZED]
    #[msg("Not reward authority")]
    NotRewardAuthority,

    /// ========================================================================
    /// TOKEN ERRORS
    /// ========================================================================

    /// Token mints must be different
    /// Label: [TOKEN] [SAME-MINT]
    #[msg("Same token mints")]
    SameTokenMints,

    /// Invalid token account
    /// Label: [TOKEN] [INVALID-ACCOUNT]
    #[msg("Invalid token account")]
    InvalidTokenAccount,

    /// ========================================================================
    /// STATE ERRORS
    /// ========================================================================

    /// Global state already initialized
    /// Label: [STATE] [ALREADY-INIT]
    #[msg("Already initialized")]
    AlreadyInitialized,
}
