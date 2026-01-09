/// ============================================================================
/// SPARKCOMPANION AMM MATH UTILITIES
/// ============================================================================
/// Mathematical functions for concentrated liquidity calculations.
/// 
/// Labels: [MATH] [CLMM] [UTILITIES]

use crate::constants::{Q64, MIN_TICK, MAX_TICK};
use crate::errors::SparkAmmError;
use anchor_lang::prelude::*;

/// ============================================================================
/// SQRT PRICE CALCULATIONS
/// ============================================================================

/// Convert tick to sqrt price (Q64.64 fixed point)
/// Formula: sqrt(1.0001^tick) * 2^64
/// 
/// Labels: [MATH] [TICK-TO-PRICE]
pub fn tick_to_sqrt_price_x64(tick: i32) -> Result<u128> {
    require!(tick >= MIN_TICK && tick <= MAX_TICK, SparkAmmError::TickOutOfRange);
    
    // Simplified calculation using pre-computed values
    // In production, use lookup tables for efficiency
    let abs_tick = tick.unsigned_abs();
    
    let mut ratio: u128 = if tick >= 0 {
        Q64
    } else {
        // For negative ticks, start with 1/sqrt(1.0001)
        Q64
    };
    
    // Apply bit-by-bit multiplication
    // Each bit corresponds to a power of sqrt(1.0001)
    if abs_tick & 1 != 0 {
        ratio = ratio.checked_mul(18446744073709551615).ok_or(SparkAmmError::Overflow)? >> 64;
    }
    
    // Continue for other bits...
    // (Full implementation would include all 24 bits)
    
    if tick < 0 {
        ratio = Q64.checked_mul(Q64).ok_or(SparkAmmError::Overflow)?
            .checked_div(ratio).ok_or(SparkAmmError::DivisionByZero)?;
    }
    
    Ok(ratio)
}

/// Convert sqrt price to tick
/// 
/// Labels: [MATH] [PRICE-TO-TICK]
pub fn sqrt_price_x64_to_tick(sqrt_price_x64: u128) -> Result<i32> {
    // Binary search or log calculation
    // Simplified version
    let log_sqrt_price = (sqrt_price_x64 as f64).ln() / (Q64 as f64).ln();
    let tick = (log_sqrt_price * 2.0 / (1.0001_f64).ln()) as i32;
    
    require!(tick >= MIN_TICK && tick <= MAX_TICK, SparkAmmError::TickOutOfRange);
    
    Ok(tick)
}

/// ============================================================================
/// LIQUIDITY CALCULATIONS
/// ============================================================================

/// Calculate liquidity from token amounts
/// 
/// Labels: [MATH] [LIQUIDITY]
pub fn get_liquidity_from_amounts(
    sqrt_price_x64: u128,
    sqrt_price_lower_x64: u128,
    sqrt_price_upper_x64: u128,
    amount_a: u64,
    amount_b: u64,
) -> Result<u128> {
    if sqrt_price_x64 <= sqrt_price_lower_x64 {
        // Current price below range - only token A
        get_liquidity_for_amount_a(sqrt_price_lower_x64, sqrt_price_upper_x64, amount_a)
    } else if sqrt_price_x64 < sqrt_price_upper_x64 {
        // Current price in range - both tokens
        let liquidity_a = get_liquidity_for_amount_a(sqrt_price_x64, sqrt_price_upper_x64, amount_a)?;
        let liquidity_b = get_liquidity_for_amount_b(sqrt_price_lower_x64, sqrt_price_x64, amount_b)?;
        Ok(liquidity_a.min(liquidity_b))
    } else {
        // Current price above range - only token B
        get_liquidity_for_amount_b(sqrt_price_lower_x64, sqrt_price_upper_x64, amount_b)
    }
}

/// Calculate liquidity from token A amount
/// 
/// Labels: [MATH] [LIQUIDITY-A]
pub fn get_liquidity_for_amount_a(
    sqrt_price_lower_x64: u128,
    sqrt_price_upper_x64: u128,
    amount_a: u64,
) -> Result<u128> {
    let numerator = (amount_a as u128)
        .checked_mul(sqrt_price_lower_x64)
        .ok_or(SparkAmmError::Overflow)?
        .checked_mul(sqrt_price_upper_x64)
        .ok_or(SparkAmmError::Overflow)?;
    
    let denominator = sqrt_price_upper_x64
        .checked_sub(sqrt_price_lower_x64)
        .ok_or(SparkAmmError::Underflow)?
        .checked_mul(Q64)
        .ok_or(SparkAmmError::Overflow)?;
    
    numerator.checked_div(denominator).ok_or(SparkAmmError::DivisionByZero.into())
}

/// Calculate liquidity from token B amount
/// 
/// Labels: [MATH] [LIQUIDITY-B]
pub fn get_liquidity_for_amount_b(
    sqrt_price_lower_x64: u128,
    sqrt_price_upper_x64: u128,
    amount_b: u64,
) -> Result<u128> {
    let numerator = (amount_b as u128)
        .checked_mul(Q64)
        .ok_or(SparkAmmError::Overflow)?;
    
    let denominator = sqrt_price_upper_x64
        .checked_sub(sqrt_price_lower_x64)
        .ok_or(SparkAmmError::Underflow)?;
    
    numerator.checked_div(denominator).ok_or(SparkAmmError::DivisionByZero.into())
}

/// ============================================================================
/// AMOUNT CALCULATIONS
/// ============================================================================

/// Calculate token A amount from liquidity
/// 
/// Labels: [MATH] [AMOUNT-A]
pub fn get_amount_a_from_liquidity(
    sqrt_price_lower_x64: u128,
    sqrt_price_upper_x64: u128,
    liquidity: u128,
    round_up: bool,
) -> Result<u64> {
    let numerator = liquidity
        .checked_mul(sqrt_price_upper_x64.checked_sub(sqrt_price_lower_x64).ok_or(SparkAmmError::Underflow)?)
        .ok_or(SparkAmmError::Overflow)?
        .checked_mul(Q64)
        .ok_or(SparkAmmError::Overflow)?;
    
    let denominator = sqrt_price_lower_x64
        .checked_mul(sqrt_price_upper_x64)
        .ok_or(SparkAmmError::Overflow)?;
    
    let mut amount = numerator.checked_div(denominator).ok_or(SparkAmmError::DivisionByZero)?;
    
    if round_up && numerator % denominator != 0 {
        amount = amount.checked_add(1).ok_or(SparkAmmError::Overflow)?;
    }
    
    Ok(amount as u64)
}

/// Calculate token B amount from liquidity
/// 
/// Labels: [MATH] [AMOUNT-B]
pub fn get_amount_b_from_liquidity(
    sqrt_price_lower_x64: u128,
    sqrt_price_upper_x64: u128,
    liquidity: u128,
    round_up: bool,
) -> Result<u64> {
    let numerator = liquidity
        .checked_mul(sqrt_price_upper_x64.checked_sub(sqrt_price_lower_x64).ok_or(SparkAmmError::Underflow)?)
        .ok_or(SparkAmmError::Overflow)?;
    
    let mut amount = numerator.checked_div(Q64).ok_or(SparkAmmError::DivisionByZero)?;
    
    if round_up && numerator % Q64 != 0 {
        amount = amount.checked_add(1).ok_or(SparkAmmError::Overflow)?;
    }
    
    Ok(amount as u64)
}

/// ============================================================================
/// SWAP CALCULATIONS
/// ============================================================================

/// Calculate swap output amount
/// 
/// Labels: [MATH] [SWAP]
pub fn compute_swap_step(
    sqrt_price_current_x64: u128,
    sqrt_price_target_x64: u128,
    liquidity: u128,
    amount_remaining: u64,
    fee_rate: u32,
    a_to_b: bool,
) -> Result<(u128, u64, u64, u64)> {
    let fee_rate_complement = 1_000_000u64.checked_sub(fee_rate as u64).ok_or(SparkAmmError::Underflow)?;
    let amount_remaining_less_fee = (amount_remaining as u128)
        .checked_mul(fee_rate_complement as u128)
        .ok_or(SparkAmmError::Overflow)?
        .checked_div(1_000_000)
        .ok_or(SparkAmmError::DivisionByZero)? as u64;
    
    // Calculate amounts based on direction
    let (amount_in, amount_out, sqrt_price_next_x64) = if a_to_b {
        // Token A to Token B
        let amount_a = get_amount_a_from_liquidity(
            sqrt_price_target_x64,
            sqrt_price_current_x64,
            liquidity,
            true,
        )?;
        
        if amount_remaining_less_fee >= amount_a {
            (amount_a, 0u64, sqrt_price_target_x64)
        } else {
            // Partial fill
            (amount_remaining_less_fee, 0u64, sqrt_price_current_x64)
        }
    } else {
        // Token B to Token A
        let amount_b = get_amount_b_from_liquidity(
            sqrt_price_current_x64,
            sqrt_price_target_x64,
            liquidity,
            true,
        )?;
        
        if amount_remaining_less_fee >= amount_b {
            (amount_b, 0u64, sqrt_price_target_x64)
        } else {
            (amount_remaining_less_fee, 0u64, sqrt_price_current_x64)
        }
    };
    
    // Calculate fee
    let fee_amount = amount_remaining
        .checked_sub(amount_in)
        .ok_or(SparkAmmError::Underflow)?
        .min((amount_in as u128).checked_mul(fee_rate as u128).ok_or(SparkAmmError::Overflow)?
            .checked_div(fee_rate_complement as u128).ok_or(SparkAmmError::DivisionByZero)? as u64);
    
    Ok((sqrt_price_next_x64, amount_in, amount_out, fee_amount))
}
