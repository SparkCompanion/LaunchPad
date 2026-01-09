use anchor_lang::prelude::*;

/// ============================================================================
/// SPARKCOMPANION BONDING CURVE ERRORS
/// ============================================================================
/// All error codes for the bonding curve program.
/// 
/// Labels: [ERRORS] [VALIDATION] [SECURITY]
#[error_code]
pub enum SparkBondingError {
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
    /// VALIDATION ERRORS
    /// ========================================================================

    /// Invalid token amount provided
    /// Label: [VALIDATION] [AMOUNT]
    #[msg("Invalid token amount")]
    InvalidTokenAmount,

    /// Slippage tolerance exceeded
    /// Label: [VALIDATION] [SLIPPAGE]
    #[msg("Slippage tolerance exceeded")]
    SlippageExceeded,

    /// Token name too long (max 32 chars)
    /// Label: [VALIDATION] [NAME]
    #[msg("Token name too long")]
    NameTooLong,

    /// Token symbol too long (max 10 chars)
    /// Label: [VALIDATION] [SYMBOL]
    #[msg("Symbol too long")]
    SymbolTooLong,

    /// ========================================================================
    /// RESERVE ERRORS
    /// ========================================================================

    /// Not enough tokens in reserve
    /// Label: [RESERVES] [TOKENS]
    #[msg("Insufficient token reserves")]
    InsufficientTokenReserves,

    /// Not enough SOL in reserve
    /// Label: [RESERVES] [SOL]
    #[msg("Insufficient SOL reserves")]
    InsufficientSolReserves,

    /// Not enough fees available to collect
    /// Label: [RESERVES] [FEES]
    #[msg("Insufficient fees available")]
    InsufficientFees,

    /// ========================================================================
    /// MIGRATION ERRORS
    /// ========================================================================

    /// Token has already migrated to AMM
    /// Label: [MIGRATION] [COMPLETED]
    #[msg("Token already migrated to AMM")]
    AlreadyMigrated,

    /// Migration threshold not yet reached
    /// Label: [MIGRATION] [THRESHOLD]
    #[msg("Migration threshold not reached")]
    MigrationThresholdNotReached,

    /// Migration is currently disabled
    /// Label: [MIGRATION] [DISABLED]
    #[msg("Migration is disabled")]
    MigrationDisabled,

    /// ========================================================================
    /// AUTHORIZATION ERRORS
    /// ========================================================================

    /// Only token creator can perform this action
    /// Label: [AUTH] [CREATOR]
    #[msg("Only creator can perform this action")]
    UnauthorizedCreator,

    /// Invalid account owner
    /// Label: [AUTH] [OWNER]
    #[msg("Invalid account owner")]
    InvalidAccountOwner,

    /// ========================================================================
    /// STATE ERRORS
    /// ========================================================================

    /// Global state already initialized
    /// Label: [STATE] [INITIALIZED]
    #[msg("Global already initialized")]
    AlreadyInitialized,

    /// Bonding curve not initialized
    /// Label: [STATE] [UNINITIALIZED]
    #[msg("Bonding curve not initialized")]
    NotInitialized,
}
