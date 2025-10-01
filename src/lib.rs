use rust_decimal::Decimal;
use std::cmp::{Ordering, PartialEq, PartialOrd};
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign,
    Mul, MulAssign, Neg, Not, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign,
};
use std::str::FromStr;

// num_traits for mathematical operations
use num_traits::{FromPrimitive, Num, One, Signed, ToPrimitive, Zero};

pub mod prelude {
    pub use super::Number;
    pub use super::js_dec;
    pub use core::str::FromStr;
    pub use num_traits::{FromPrimitive, One, Signed, ToPrimitive, Zero};
    pub use rust_decimal::{Decimal, RoundingStrategy};
}

/// A JavaScript-semantic number that uses Decimal for finite arithmetic
/// but properly handles IEEE special values (NaN, ±Infinity, -0)
#[derive(Debug, Clone, Copy)]
pub enum Number {
    /// A finite decimal number
    Finite(Decimal),
    /// JavaScript NaN (Not a Number)
    NaN,
    /// JavaScript positive infinity
    PositiveInfinity,
    /// JavaScript negative infinity
    NegativeInfinity,
    /// JavaScript negative zero (distinct from positive zero)
    NegativeZero,
}

impl Number {
    // Constants
    pub const NAN: Number = Number::NaN;
    pub const POSITIVE_INFINITY: Number = Number::PositiveInfinity;
    pub const NEGATIVE_INFINITY: Number = Number::NegativeInfinity;
    pub const ZERO: Number = Number::Finite(Decimal::ZERO);
    pub const ONE: Number = Number::Finite(Decimal::ONE);
    pub const NEGATIVE_ZERO: Number = Number::NegativeZero;

    pub fn new(num: i64, scale: u32) -> Self {
        Self::Finite(Decimal::new(num, scale))
    }
    pub const fn new_uint(num: u32) -> Self {
        Self::Finite(Decimal::from_parts(num, 0, 0, false, 0))
    }

    pub fn try_from_i128_with_scale(num: i128, scale: u32) -> Result<Self, rust_decimal::Error> {
        Ok(Self::Finite(Decimal::try_from_i128_with_scale(num, scale)?))
    }

    // Type checking predicates
    pub fn is_nan(&self) -> bool {
        matches!(self, Number::NaN)
    }

    pub fn is_finite(&self) -> bool {
        matches!(self, Number::Finite(_) | Number::NegativeZero)
    }

    pub fn is_infinite(&self) -> bool {
        matches!(self, Number::PositiveInfinity | Number::NegativeInfinity)
    }

    pub fn is_positive_infinity(&self) -> bool {
        matches!(self, Number::PositiveInfinity)
    }

    pub fn is_negative_infinity(&self) -> bool {
        matches!(self, Number::NegativeInfinity)
    }

    // Mathematical functions following JS semantics
    pub fn abs(self) -> Number {
        match self {
            Number::Finite(d) => Number::Finite(d.abs()),
            Number::NegativeZero => Number::ZERO, // abs(-0) = +0
            Number::NaN => Number::NaN,
            Number::PositiveInfinity => Number::PositiveInfinity,
            Number::NegativeInfinity => Number::PositiveInfinity,
        }
    }

    pub fn floor(self) -> Number {
        match self {
            Number::Finite(d) => {
                // JavaScript floor: largest integer less than or equal to the number
                // Using f64 conversion is safe here since the result is always a whole number
                // and whole numbers within reasonable range are exactly representable in f64
                let f = d.to_f64().unwrap_or(0.0);
                Number::from(f.floor())
            }
            Number::NegativeZero => Number::NegativeZero, // floor(-0) = -0
            Number::NaN => Number::NaN,
            Number::PositiveInfinity => Number::PositiveInfinity,
            Number::NegativeInfinity => Number::NegativeInfinity,
        }
    }

    pub fn ceil(self) -> Number {
        match self {
            Number::Finite(d) => {
                // JavaScript ceil: smallest integer greater than or equal to the number
                // Using f64 conversion is safe here since the result is always a whole number
                let f = d.to_f64().unwrap_or(0.0);
                Number::from(f.ceil())
            }
            Number::NegativeZero => Number::NegativeZero, // ceil(-0) = -0
            Number::NaN => Number::NaN,
            Number::PositiveInfinity => Number::PositiveInfinity,
            Number::NegativeInfinity => Number::NegativeInfinity,
        }
    }

    pub fn round(self) -> Number {
        match self {
            Number::Finite(d) => {
                // JavaScript round: rounds to nearest integer, ties away from zero
                // For -3.5, should round to -3 (away from zero)
                let f = d.to_f64().unwrap_or(0.0);
                let rounded = if f >= 0.0 {
                    (f + 0.5).floor()
                } else {
                    // For negative numbers, round ties away from zero
                    // -3.5 should become -3, not -4
                    // Use: (f + 0.5).ceil() for negative numbers
                    (f + 0.5).ceil()
                };
                Number::from(rounded)
            }
            Number::NegativeZero => Number::NegativeZero, // round(-0) = -0
            Number::NaN => Number::NaN,
            Number::PositiveInfinity => Number::PositiveInfinity,
            Number::NegativeInfinity => Number::NegativeInfinity,
        }
    }

    pub fn round_dp(self, dp: u32) -> Number {
        match self {
            Number::Finite(d) => Number::Finite(d.round_dp(dp)),
            Number::NegativeZero => Number::NegativeZero, // round(-0) = -0
            Number::NaN => Number::NaN,
            Number::PositiveInfinity => Number::PositiveInfinity,
            Number::NegativeInfinity => Number::NegativeInfinity,
        }
    }

    pub fn trunc(self) -> Number {
        match self {
            Number::Finite(d) => Number::Finite(d.trunc()),
            Number::NegativeZero => Number::NegativeZero, // trunc(-0) = -0
            Number::NaN => Number::NaN,
            Number::PositiveInfinity => Number::PositiveInfinity,
            Number::NegativeInfinity => Number::NegativeInfinity,
        }
    }

    pub fn sqrt(self) -> Number {
        match self {
            Number::Finite(d) => {
                if d < Decimal::ZERO {
                    Number::NaN // sqrt of negative number is NaN in JS
                } else if d.is_zero() {
                    Number::Finite(Decimal::ZERO) // sqrt(0) = 0
                } else {
                    // Babylonian method (Newton-Raphson) for square root
                    // Formula: x_{n+1} = (x_n + S/x_n) / 2
                    let mut x = d / Decimal::from(2); // Initial guess: S/2
                    let two = Decimal::from(2);

                    // Iterate until convergence (or max iterations)
                    for _ in 0..50 {
                        // Max 50 iterations should be plenty
                        let next = (x + d / x) / two;
                        // Check for convergence (difference smaller than epsilon)
                        if (next - x).abs()
                            < Decimal::from_str("0.0000000000000000000000000001")
                                .unwrap_or(Decimal::ZERO)
                        {
                            break;
                        }
                        x = next;
                    }

                    // Check if this is a perfect square by squaring the result
                    let x_squared = x * x;
                    if (x_squared - d).abs()
                        < Decimal::from_str("0.0000000000000000000000000001")
                            .unwrap_or(Decimal::ZERO)
                    {
                        // Round to nearest integer if very close
                        let rounded = x.round();
                        if (rounded * rounded - d).abs() < (x * x - d).abs() {
                            x = rounded;
                        }
                    }

                    Number::Finite(x)
                }
            }
            Number::NegativeZero => Number::ZERO, // sqrt(-0) = +0 in JS
            Number::NaN => Number::NaN,
            Number::PositiveInfinity => Number::PositiveInfinity,
            Number::NegativeInfinity => Number::NaN, // sqrt(-Infinity) = NaN
        }
    }

    pub fn pow(self, exponent: Number) -> Number {
        match (self, exponent) {
            // Handle NaN cases first
            (Number::NaN, Number::Finite(exp)) if exp.is_zero() => Number::ONE, // NaN**0 = 1 in JS
            (Number::NaN, _) => Number::NaN,
            (_, Number::NaN) => Number::NaN,

            // Handle 0**0 = 1 (special JS behavior)
            (Number::Finite(base), Number::Finite(exp)) if base.is_zero() && exp.is_zero() => {
                Number::ONE
            }
            (Number::NegativeZero, Number::Finite(exp)) if exp.is_zero() => Number::ONE,
            (Number::Finite(base), Number::NegativeZero) if base.is_zero() => Number::ONE,
            (Number::NegativeZero, Number::NegativeZero) => Number::ONE,

            // Handle zero base cases
            (Number::Finite(base), Number::Finite(exp)) if base.is_zero() => {
                if exp > Decimal::ZERO {
                    Number::ZERO
                } else if exp < Decimal::ZERO {
                    Number::POSITIVE_INFINITY
                } else {
                    Number::ONE // 0**0 = 1
                }
            }
            (Number::NegativeZero, Number::Finite(exp)) => {
                if exp > Decimal::ZERO {
                    // Check if exponent is odd or even
                    if exp.fract().is_zero() {
                        let exp_i64 = exp.to_i64().unwrap_or(0);
                        if exp_i64 % 2 == 1 {
                            Number::NegativeZero // (-0)^odd = -0
                        } else {
                            Number::ZERO // (-0)^even = +0
                        }
                    } else {
                        Number::ZERO // (-0)^fractional = +0
                    }
                } else if exp < Decimal::ZERO {
                    Number::NEGATIVE_INFINITY // (-0)^negative = -∞
                } else {
                    Number::ONE // already handled above
                }
            }

            // Handle infinity exponent cases
            (Number::Finite(base), Number::PositiveInfinity) => {
                let abs_base = base.abs();
                if abs_base > Decimal::ONE {
                    Number::POSITIVE_INFINITY
                } else if abs_base < Decimal::ONE {
                    Number::ZERO
                } else {
                    Number::ONE // 1**Infinity = 1
                }
            }
            (Number::Finite(base), Number::NegativeInfinity) => {
                let abs_base = base.abs();
                if abs_base > Decimal::ONE {
                    Number::ZERO
                } else if abs_base < Decimal::ONE {
                    Number::POSITIVE_INFINITY
                } else {
                    Number::ONE // 1**(-Infinity) = 1
                }
            }
            (Number::NegativeZero, Number::PositiveInfinity) => Number::ZERO,
            (Number::NegativeZero, Number::NegativeInfinity) => Number::POSITIVE_INFINITY,

            // Handle infinity base cases
            (Number::PositiveInfinity, Number::Finite(exp)) => {
                if exp > Decimal::ZERO {
                    Number::POSITIVE_INFINITY
                } else if exp < Decimal::ZERO {
                    Number::ZERO
                } else {
                    Number::ONE // Infinity**0 = 1
                }
            }
            (Number::NegativeInfinity, Number::Finite(exp)) => {
                if exp > Decimal::ZERO {
                    // Check if exponent is odd or even
                    if exp.fract().is_zero() {
                        let exp_i64 = exp.to_i64().unwrap_or(0);
                        if exp_i64 % 2 == 1 {
                            Number::NEGATIVE_INFINITY
                        } else {
                            Number::POSITIVE_INFINITY
                        }
                    } else {
                        Number::NaN // Fractional exponent of negative number
                    }
                } else if exp < Decimal::ZERO {
                    Number::ZERO
                } else {
                    Number::ONE // (-Infinity)**0 = 1
                }
            }

            // Handle infinity ** infinity cases
            (Number::PositiveInfinity, Number::PositiveInfinity) => Number::POSITIVE_INFINITY,
            (Number::PositiveInfinity, Number::NegativeInfinity) => Number::ZERO,
            (Number::NegativeInfinity, Number::PositiveInfinity) => Number::POSITIVE_INFINITY,
            (Number::NegativeInfinity, Number::NegativeInfinity) => Number::ZERO,

            // Handle finite base and exponent
            (Number::Finite(base), Number::Finite(exp)) => {
                // Check for negative base with fractional exponent
                if base < Decimal::ZERO && !exp.fract().is_zero() {
                    return Number::NaN;
                }

                // For integer exponents, use repeated multiplication for better precision
                if exp.fract().is_zero() {
                    let exp_i64 = exp.to_i64().unwrap_or(0);
                    if exp_i64 >= 0 && exp_i64 <= 1000 {
                        // Reasonable range for integer powers
                        let mut result = Decimal::ONE;
                        let mut current_base = base;
                        let mut current_exp = exp_i64 as u64;

                        // Fast exponentiation by squaring
                        while current_exp > 0 {
                            if current_exp % 2 == 1 {
                                result *= current_base;
                            }
                            current_base *= current_base;
                            current_exp /= 2;
                        }

                        return Number::Finite(result);
                    }
                }

                // For fractional exponents or large integer exponents, use exp(ln(base) * exp)
                // but this requires implementing ln and exp functions with Decimal precision
                // TODO: Implement proper decimal-precision ln and exp functions
                // For now, we can't handle fractional exponents without losing precision
                if exp.fract().is_zero() {
                    // Handle negative integer exponents
                    let exp_i64 = exp.to_i64().unwrap_or(0);
                    if exp_i64 < 0 {
                        // base^(-n) = 1 / (base^n)
                        let positive_exp = (-exp_i64) as u64;
                        let mut result = Decimal::ONE;
                        let mut current_base = base;
                        let mut current_exp = positive_exp;

                        // Fast exponentiation by squaring
                        while current_exp > 0 {
                            if current_exp % 2 == 1 {
                                result *= current_base;
                            }
                            current_base *= current_base;
                            current_exp /= 2;
                        }

                        Number::Finite(Decimal::ONE / result)
                    } else {
                        // Very large positive exponent - this will likely overflow
                        // For now, return positive infinity for very large results
                        Number::POSITIVE_INFINITY
                    }
                } else {
                    // Handle special case of 0.5 exponent (square root)
                    if exp == Decimal::from_str("0.5").unwrap_or(Decimal::ZERO) {
                        return Number::Finite(base).sqrt();
                    }

                    // Fractional exponent - use a^b = e^(b * ln(a))
                    // TODO: For high precision, implement ln and exp with Decimal directly
                    let ln_base = Number::Finite(base).log();
                    let exp_arg = Number::Finite(exp) * ln_base;
                    exp_arg.exp()
                }
            }

            // Handle cases where exponent is NegativeZero
            (Number::Finite(_), Number::NegativeZero) => Number::ONE, // x^(-0) = 1
            (Number::PositiveInfinity, Number::NegativeZero) => Number::ONE, // (+∞)^(-0) = 1
            (Number::NegativeInfinity, Number::NegativeZero) => Number::ONE, // (-∞)^(-0) = 1
        }
    }

    pub fn log(self) -> Number {
        // TODO: For high precision, implement using Newton's method or Taylor series:
        // ln(x) can be computed using the series: ln(1+u) = u - u²/2 + u³/3 - u⁴/4 + ...
        // Or use Newton's method: find y such that e^y = x
        // For now, using f64 conversion for compatibility
        match self {
            Number::Finite(d) => {
                if d <= Decimal::ZERO {
                    if d.is_zero() {
                        Number::NegativeInfinity
                    } else {
                        Number::NaN // log of negative number
                    }
                } else {
                    let f = d.to_f64().unwrap_or(0.0);
                    Number::from(f.ln())
                }
            }
            Number::NegativeZero => Number::NegativeInfinity, // log(-0) = -∞
            Number::NaN => Number::NaN,
            Number::PositiveInfinity => Number::PositiveInfinity,
            Number::NegativeInfinity => Number::NaN,
        }
    }

    pub fn log10(self) -> Number {
        // TODO: For high precision, implement as log(x) / log(10) using Decimal
        // For now, using f64 conversion for compatibility
        match self {
            Number::Finite(d) => {
                if d <= Decimal::ZERO {
                    if d.is_zero() {
                        Number::NegativeInfinity
                    } else {
                        Number::NaN // log of negative number
                    }
                } else {
                    let f = d.to_f64().unwrap_or(0.0);
                    Number::from(f.log10())
                }
            }
            Number::NegativeZero => Number::NegativeInfinity, // log10(-0) = -∞
            Number::NaN => Number::NaN,
            Number::PositiveInfinity => Number::PositiveInfinity,
            Number::NegativeInfinity => Number::NaN,
        }
    }

    pub fn log2(self) -> Number {
        // TODO: For high precision, implement as log(x) / log(2) using Decimal
        // For now, using f64 conversion for compatibility
        match self {
            Number::Finite(d) => {
                if d <= Decimal::ZERO {
                    if d.is_zero() {
                        Number::NegativeInfinity
                    } else {
                        Number::NaN // log of negative number
                    }
                } else {
                    let f = d.to_f64().unwrap_or(0.0);
                    Number::from(f.log2())
                }
            }
            Number::NegativeZero => Number::NegativeInfinity, // log2(-0) = -∞
            Number::NaN => Number::NaN,
            Number::PositiveInfinity => Number::PositiveInfinity,
            Number::NegativeInfinity => Number::NaN,
        }
    }

    pub fn exp(self) -> Number {
        // TODO: For high precision, implement using Taylor series with Decimal:
        // e^x = 1 + x + x²/2! + x³/3! + x⁴/4! + ...
        // For now, using f64 conversion for compatibility
        match self {
            Number::Finite(d) => {
                let f = d.to_f64().unwrap_or(0.0);
                let result = f.exp();
                Number::from(result)
            }
            Number::NegativeZero => Number::ONE, // exp(-0) = 1
            Number::NaN => Number::NaN,
            Number::PositiveInfinity => Number::PositiveInfinity,
            Number::NegativeInfinity => Number::ZERO,
        }
    }

    pub fn sin(self) -> Number {
        // TODO: For high precision, implement using Taylor series with Decimal:
        // sin(x) = x - x³/3! + x⁵/5! - x⁷/7! + ...
        // This would require implementing factorial and power series with Decimal precision
        // For now, using f64 conversion for compatibility
        match self {
            Number::Finite(d) => {
                let f = d.to_f64().unwrap_or(0.0);
                Number::from(f.sin())
            }
            Number::NegativeZero => Number::NegativeZero, // sin(-0) = -0
            Number::NaN => Number::NaN,
            Number::PositiveInfinity | Number::NegativeInfinity => Number::NaN,
        }
    }

    pub fn cos(self) -> Number {
        // TODO: For high precision, implement using Taylor series with Decimal:
        // cos(x) = 1 - x²/2! + x⁴/4! - x⁶/6! + ...
        // This would require implementing factorial and power series with Decimal precision
        // For now, using f64 conversion for compatibility
        match self {
            Number::Finite(d) => {
                let f = d.to_f64().unwrap_or(0.0);
                Number::from(f.cos())
            }
            Number::NegativeZero => Number::ONE, // cos(-0) = 1
            Number::NaN => Number::NaN,
            Number::PositiveInfinity | Number::NegativeInfinity => Number::NaN,
        }
    }

    pub fn tan(self) -> Number {
        // TODO: For high precision, implement as sin(x)/cos(x) using Decimal Taylor series
        // Need to handle asymptotes where cos(x) = 0
        // For now, using f64 conversion for compatibility
        match self {
            Number::Finite(d) => {
                let f = d.to_f64().unwrap_or(0.0);
                Number::from(f.tan())
            }
            Number::NegativeZero => Number::NegativeZero, // tan(-0) = -0
            Number::NaN => Number::NaN,
            Number::PositiveInfinity | Number::NegativeInfinity => Number::NaN,
        }
    }

    pub fn asin(self) -> Number {
        // TODO: For high precision, implement using Taylor series with Decimal:
        // asin(x) = x + x³/6 + 3x⁵/40 + ... (for |x| < 1)
        // Or use Newton's method: find y such that sin(y) = x
        // For now, using f64 conversion for compatibility
        match self {
            Number::Finite(d) => {
                let f = d.to_f64().unwrap_or(0.0);
                if f.abs() > 1.0 {
                    Number::NaN
                } else {
                    Number::from(f.asin())
                }
            }
            Number::NegativeZero => Number::NegativeZero, // asin(-0) = -0
            Number::NaN => Number::NaN,
            Number::PositiveInfinity | Number::NegativeInfinity => Number::NaN,
        }
    }

    pub fn acos(self) -> Number {
        // TODO: For high precision, implement using relationship acos(x) = π/2 - asin(x)
        // Or use Newton's method: find y such that cos(y) = x
        // For now, using f64 conversion for compatibility
        match self {
            Number::Finite(d) => {
                let f = d.to_f64().unwrap_or(0.0);
                if f.abs() > 1.0 {
                    Number::NaN
                } else {
                    Number::from(f.acos())
                }
            }
            Number::NegativeZero => Number::from(std::f64::consts::FRAC_PI_2), // acos(-0) = π/2
            Number::NaN => Number::NaN,
            Number::PositiveInfinity | Number::NegativeInfinity => Number::NaN,
        }
    }

    pub fn atan(self) -> Number {
        // TODO: For high precision, implement using Taylor series with Decimal:
        // atan(x) = x - x³/3 + x⁵/5 - x⁷/7 + ... (for |x| < 1)
        // For |x| >= 1, use atan(x) = π/2 - atan(1/x)
        // For now, using f64 conversion for compatibility
        match self {
            Number::Finite(d) => {
                let f = d.to_f64().unwrap_or(0.0);
                Number::from(f.atan())
            }
            Number::NegativeZero => Number::NegativeZero, // atan(-0) = -0
            Number::NaN => Number::NaN,
            Number::PositiveInfinity => Number::from(std::f64::consts::FRAC_PI_2),
            Number::NegativeInfinity => Number::from(-std::f64::consts::FRAC_PI_2),
        }
    }

    pub fn atan2(self, x: Number) -> Number {
        // TODO: For high precision, implement using Decimal arithmetic:
        // atan2(y, x) handles all quadrants and edge cases
        // Would need to implement atan with Decimal and handle all JS edge cases
        // For now, using f64 conversion for compatibility
        match (self, x) {
            (Number::Finite(y), Number::Finite(x_val)) => {
                let y_f = y.to_f64().unwrap_or(0.0);
                let x_f = x_val.to_f64().unwrap_or(0.0);
                Number::from(y_f.atan2(x_f))
            }
            (Number::NegativeZero, Number::Finite(x_val)) => {
                let x_f = x_val.to_f64().unwrap_or(0.0);
                Number::from((-0.0_f64).atan2(x_f))
            }
            (Number::Finite(y), Number::NegativeZero) => {
                let y_f = y.to_f64().unwrap_or(0.0);
                Number::from(y_f.atan2(-0.0_f64))
            }
            (Number::NegativeZero, Number::NegativeZero) => {
                Number::from((-0.0_f64).atan2(-0.0_f64))
            }
            (Number::NaN, _) | (_, Number::NaN) => Number::NaN,
            // Handle infinity cases according to JS Math.atan2
            (Number::PositiveInfinity, Number::PositiveInfinity) => {
                Number::from(std::f64::consts::FRAC_PI_4)
            }
            (Number::PositiveInfinity, Number::NegativeInfinity) => {
                Number::from(3.0 * std::f64::consts::FRAC_PI_4)
            }
            (Number::NegativeInfinity, Number::PositiveInfinity) => {
                Number::from(-std::f64::consts::FRAC_PI_4)
            }
            (Number::NegativeInfinity, Number::NegativeInfinity) => {
                Number::from(-3.0 * std::f64::consts::FRAC_PI_4)
            }
            (Number::PositiveInfinity, _) => Number::from(std::f64::consts::FRAC_PI_2),
            (Number::NegativeInfinity, _) => Number::from(-std::f64::consts::FRAC_PI_2),
            (_, Number::PositiveInfinity) => Number::from(0.0),
            (Number::Finite(y), Number::NegativeInfinity) => {
                if y >= Decimal::ZERO {
                    Number::from(std::f64::consts::PI)
                } else {
                    Number::from(-std::f64::consts::PI)
                }
            }
            (Number::NegativeZero, Number::NegativeInfinity) => Number::from(-std::f64::consts::PI),
        }
    }

    // JS-specific operations that don't have Rust traits
    pub fn unsigned_right_shift(self, bits: Number) -> Number {
        // JavaScript's >>> operator: unsigned 32-bit right shift
        let a = self.to_u32_js_coerce(); // Convert to unsigned 32-bit
        let b = bits.to_u32_js_coerce() & 0x1f; // Mask to 5 bits like other shifts
        Number::from(a >> b)
    }

    pub fn increment(self) -> Number {
        self + Number::ONE
    }

    pub fn decrement(self) -> Number {
        self - Number::ONE
    }

    // Type conversions following JS semantics
    /// This is primarily for following semantics during bit-wise operations
    pub fn to_i32_js_coerce(&self) -> i32 {
        match self {
            Number::Finite(d) => {
                // Try direct conversion first (fast path)
                if let Some(i) = d.to_i32() {
                    return i;
                }

                // Truncate to integer part and convert to i128
                let truncated = d.trunc();
                let i128_val = truncated
                    .to_i128()
                    .expect("Decimal should always fit in i128");

                // JavaScript ToInt32: modulo 2^32 and interpret as signed
                i128_val as i32 // Rust's `as` conversion handles the wrapping
            }
            Number::NegativeZero => 0,
            Number::NaN => 0,
            Number::PositiveInfinity | Number::NegativeInfinity => 0,
        }
    }
    /// This is used for conversion following semantics during bit-wise operations
    pub fn to_i64_js_coerce(&self) -> i64 {
        match self {
            Number::Finite(d) => {
                // Try direct conversion first (fast path)
                if let Some(i) = d.to_i64() {
                    return i;
                }

                // Truncate to integer part and convert to i128
                let truncated = d.trunc();
                let i128_val = truncated
                    .to_i128()
                    .expect("Decimal should always fit in i128");

                // JavaScript ToInt64: modulo 2^64 and interpret as signed
                i128_val as i64 // Rust's `as` conversion handles the wrapping
            }
            Number::NegativeZero => 0,
            Number::NaN => 0,
            Number::PositiveInfinity | Number::NegativeInfinity => 0,
        }
    }
    pub fn to_i32(&self) -> Option<i32> {
        match self {
            Number::Finite(d) => d.to_i32(),
            Number::NegativeZero => Some(0),
            Number::NaN => None,
            Number::PositiveInfinity => None,
            Number::NegativeInfinity => None,
        }
    }

    pub fn to_u32(&self) -> Option<u32> {
        match self {
            Number::Finite(d) => d.to_u32(),
            Number::NegativeZero => Some(0),
            Number::NaN => None,
            Number::PositiveInfinity => None,
            Number::NegativeInfinity => None,
        }
    }

    pub fn to_u32_js_coerce(&self) -> u32 {
        match self {
            Number::Finite(_) | Number::NegativeZero => {
                // JavaScript ToUint32: convert to i32 first, then reinterpret as u32
                // This handles the wrapping behavior correctly
                let i32_val = self.to_i32_js_coerce();
                i32_val as u32
            }
            Number::NaN => 0,
            Number::PositiveInfinity | Number::NegativeInfinity => 0,
        }
    }

    pub fn to_i64(&self) -> Option<i64> {
        match self {
            Number::Finite(d) => d.to_i64(),
            Number::NegativeZero => Some(0),
            Number::NaN => None,
            Number::PositiveInfinity => None,
            Number::NegativeInfinity => None,
        }
    }

    pub fn to_f64(&self) -> f64 {
        match self {
            Number::Finite(d) => d.to_f64().expect("Decimal always fits in f64"),
            Number::NegativeZero => -0.0,
            Number::NaN => f64::NAN,
            Number::PositiveInfinity => f64::INFINITY,
            Number::NegativeInfinity => f64::NEG_INFINITY,
        }
    }

    pub fn to_decimal(&self) -> Option<Decimal> {
        match self {
            Number::Finite(d) => Some(*d),
            Number::NegativeZero => Some(Decimal::ZERO),
            _ => None,
        }
    }

    // JS semantic operations
    pub fn is_truthy(&self) -> bool {
        match self {
            Number::Finite(d) => !d.is_zero(), // 0 is falsy, everything else is truthy
            Number::NegativeZero => false,     // -0 is falsy
            Number::NaN => false,              // NaN is falsy
            Number::PositiveInfinity | Number::NegativeInfinity => true, // ±∞ are truthy
        }
    }

    pub fn is_falsy(&self) -> bool {
        !self.is_truthy()
    }

    /// Convert to string following JavaScript's ToString algorithm
    pub fn to_js_string(&self) -> String {
        match self {
            Number::Finite(d) => {
                // JavaScript uses scientific notation for very large or very small numbers
                let f = d.to_f64().unwrap_or(0.0);
                if f.abs() >= 1e21 || (f.abs() < 1e-6 && f != 0.0) {
                    // Format with explicit + sign for positive exponents to match JS
                    let scientific = format!("{:e}", f);
                    if scientific.contains("e") && !scientific.contains("e-") {
                        scientific.replace("e", "e+")
                    } else {
                        scientific
                    }
                } else {
                    d.to_string()
                }
            }
            Number::NegativeZero => "0".to_string(), // -0 displays as "0" in JS
            Number::NaN => "NaN".to_string(),
            Number::PositiveInfinity => "Infinity".to_string(),
            Number::NegativeInfinity => "-Infinity".to_string(),
        }
    }

    /// Convert to primitive value (used in JS type coercion)
    pub fn to_primitive(&self) -> Number {
        self.clone() // Numbers are already primitive
    }

    // Comparison helpers for JS semantics
    pub fn js_equals(&self, other: &Number) -> bool {
        // This is JavaScript's == comparison (after type coercion)
        // For numbers, it's the same as strict equality
        self == other
    }

    pub fn js_strict_equals(&self, other: &Number) -> bool {
        // This is JavaScript's === comparison
        self == other
    }

    /// JavaScript's abstract comparison algorithm
    pub fn js_less_than(&self, other: &Number) -> Option<bool> {
        // In JavaScript, if either operand is NaN, comparison returns undefined (None)
        match self.partial_cmp(other) {
            Some(Ordering::Less) => Some(true),
            Some(Ordering::Greater) | Some(Ordering::Equal) => Some(false),
            None => None, // NaN comparisons
        }
    }
}
impl Add for Number {
    type Output = Number;
    fn add(self, rhs: Number) -> Number {
        match (self, rhs) {
            (Number::Finite(a), Number::Finite(b)) => Number::Finite(a + b),
            (Number::Finite(a), Number::NegativeZero) => Number::Finite(a),
            (Number::NegativeZero, Number::Finite(b)) => Number::Finite(b),
            (Number::NegativeZero, Number::NegativeZero) => Number::NegativeZero, // (-0) + (-0) = -0
            (Number::NaN, _) | (_, Number::NaN) => Number::NaN,
            (Number::PositiveInfinity, Number::NegativeInfinity)
            | (Number::NegativeInfinity, Number::PositiveInfinity) => Number::NaN, // ∞ + (-∞) = NaN
            (Number::PositiveInfinity, _) | (_, Number::PositiveInfinity) => {
                Number::PositiveInfinity
            }
            (Number::NegativeInfinity, _) | (_, Number::NegativeInfinity) => {
                Number::NegativeInfinity
            }
        }
    }
}

impl Sub for Number {
    type Output = Number;
    fn sub(self, rhs: Number) -> Number {
        match (self, rhs) {
            (Number::Finite(a), Number::Finite(b)) => Number::Finite(a - b),
            (Number::Finite(a), Number::NegativeZero) => Number::Finite(a), // x - (-0) = x
            (Number::NegativeZero, Number::Finite(b)) => Number::Finite(-b), // (-0) - x = -x
            (Number::NegativeZero, Number::NegativeZero) => Number::ZERO,   // (-0) - (-0) = +0
            (Number::NaN, _) | (_, Number::NaN) => Number::NaN,
            (Number::PositiveInfinity, Number::PositiveInfinity)
            | (Number::NegativeInfinity, Number::NegativeInfinity) => Number::NaN, // ∞ - ∞ = NaN
            (Number::PositiveInfinity, _) => Number::PositiveInfinity,
            (Number::NegativeInfinity, _) => Number::NegativeInfinity,
            (_, Number::PositiveInfinity) => Number::NegativeInfinity,
            (_, Number::NegativeInfinity) => Number::PositiveInfinity,
        }
    }
}

impl Mul for Number {
    type Output = Number;
    fn mul(self, rhs: Number) -> Number {
        match (self, rhs) {
            (Number::Finite(a), Number::Finite(b)) => Number::Finite(a * b),
            (Number::Finite(a), Number::NegativeZero) => {
                if a.is_zero() {
                    Number::NegativeZero // 0 * (-0) = -0 in JS
                } else if a > Decimal::ZERO {
                    Number::NegativeZero // positive * (-0) = -0
                } else {
                    Number::ZERO // negative * (-0) = +0
                }
            }
            (Number::NegativeZero, Number::Finite(b)) => {
                if b.is_zero() {
                    Number::NegativeZero // (-0) * 0 = -0 in JS
                } else if b > Decimal::ZERO {
                    Number::NegativeZero // (-0) * positive = -0
                } else {
                    Number::ZERO // (-0) * negative = +0
                }
            }
            (Number::NegativeZero, Number::NegativeZero) => Number::ZERO, // (-0) * (-0) = +0
            (Number::NaN, _) | (_, Number::NaN) => Number::NaN,
            // 0 * ∞ = NaN in JavaScript
            (Number::Finite(a), Number::PositiveInfinity)
            | (Number::Finite(a), Number::NegativeInfinity)
                if a.is_zero() =>
            {
                Number::NaN
            }
            (Number::PositiveInfinity, Number::Finite(b))
            | (Number::NegativeInfinity, Number::Finite(b))
                if b.is_zero() =>
            {
                Number::NaN
            }
            (Number::PositiveInfinity, Number::NegativeZero)
            | (Number::NegativeInfinity, Number::NegativeZero)
            | (Number::NegativeZero, Number::PositiveInfinity)
            | (Number::NegativeZero, Number::NegativeInfinity) => Number::NaN,
            // Handle infinity multiplication
            (Number::PositiveInfinity, Number::PositiveInfinity)
            | (Number::NegativeInfinity, Number::NegativeInfinity) => Number::PositiveInfinity,
            (Number::PositiveInfinity, Number::NegativeInfinity)
            | (Number::NegativeInfinity, Number::PositiveInfinity) => Number::NegativeInfinity,
            // Infinity * finite number
            (Number::PositiveInfinity, Number::Finite(b))
            | (Number::Finite(b), Number::PositiveInfinity) => {
                if b > Decimal::ZERO {
                    Number::PositiveInfinity
                } else {
                    Number::NegativeInfinity
                }
            }
            (Number::NegativeInfinity, Number::Finite(b))
            | (Number::Finite(b), Number::NegativeInfinity) => {
                if b > Decimal::ZERO {
                    Number::NegativeInfinity
                } else {
                    Number::PositiveInfinity
                }
            }
        }
    }
}

impl Div for Number {
    type Output = Number;
    fn div(self, rhs: Number) -> Number {
        match (self, rhs) {
            (Number::Finite(a), Number::Finite(b)) => {
                if b.is_zero() {
                    if a.is_zero() {
                        Number::NaN // 0/0 = NaN
                    } else if a > Decimal::ZERO {
                        Number::PositiveInfinity // positive/0 = +∞
                    } else {
                        Number::NegativeInfinity // negative/0 = -∞
                    }
                } else {
                    Number::Finite(a / b)
                }
            }
            (Number::Finite(a), Number::NegativeZero) => {
                if a.is_zero() {
                    Number::NaN // 0/(-0) = NaN
                } else if a > Decimal::ZERO {
                    Number::NegativeInfinity // positive/(-0) = -∞
                } else {
                    Number::PositiveInfinity // negative/(-0) = +∞
                }
            }
            (Number::NegativeZero, Number::Finite(b)) => {
                if b.is_zero() {
                    Number::NaN // (-0)/0 = NaN
                } else if b > Decimal::ZERO {
                    Number::NegativeZero // (-0)/positive = -0
                } else {
                    Number::ZERO // (-0)/negative = +0
                }
            }
            (Number::NegativeZero, Number::NegativeZero) => Number::NaN, // (-0)/(-0) = NaN
            (Number::NaN, _) | (_, Number::NaN) => Number::NaN,
            // ∞ / ∞ = NaN, 0 / ∞ = 0
            (Number::PositiveInfinity, Number::PositiveInfinity)
            | (Number::PositiveInfinity, Number::NegativeInfinity)
            | (Number::NegativeInfinity, Number::PositiveInfinity)
            | (Number::NegativeInfinity, Number::NegativeInfinity) => Number::NaN,
            // finite / ∞ = 0 (with appropriate sign)
            (Number::Finite(_), Number::PositiveInfinity)
            | (Number::Finite(_), Number::NegativeInfinity) => Number::Finite(Decimal::ZERO),
            (Number::NegativeZero, Number::PositiveInfinity)
            | (Number::NegativeZero, Number::NegativeInfinity) => Number::NegativeZero,
            // ∞ / finite
            (Number::PositiveInfinity, Number::Finite(b)) => {
                if b > Decimal::ZERO {
                    Number::PositiveInfinity
                } else {
                    Number::NegativeInfinity
                }
            }
            (Number::NegativeInfinity, Number::Finite(b)) => {
                if b > Decimal::ZERO {
                    Number::NegativeInfinity
                } else {
                    Number::PositiveInfinity
                }
            }
            (Number::PositiveInfinity, Number::NegativeZero) => Number::NegativeInfinity,
            (Number::NegativeInfinity, Number::NegativeZero) => Number::PositiveInfinity,
        }
    }
}

impl Rem for Number {
    type Output = Number;
    fn rem(self, rhs: Number) -> Number {
        match (self, rhs) {
            (Number::Finite(a), Number::Finite(b)) => {
                if b.is_zero() {
                    Number::NaN // x % 0 = NaN
                } else {
                    Number::Finite(a % b)
                }
            }
            (Number::Finite(_), Number::NegativeZero) => Number::NaN, // x % (-0) = NaN
            (Number::NegativeZero, Number::Finite(b)) => {
                if b.is_zero() {
                    Number::NaN // (-0) % 0 = NaN
                } else {
                    Number::NegativeZero // (-0) % x = -0
                }
            }
            (Number::NegativeZero, Number::NegativeZero) => Number::NaN, // (-0) % (-0) = NaN
            (Number::NaN, _) | (_, Number::NaN) => Number::NaN,
            // ∞ % anything = NaN, anything % ∞ = the anything
            (Number::PositiveInfinity, _) | (Number::NegativeInfinity, _) => Number::NaN,
            (Number::Finite(a), Number::PositiveInfinity)
            | (Number::Finite(a), Number::NegativeInfinity) => Number::Finite(a),
            (Number::NegativeZero, Number::PositiveInfinity)
            | (Number::NegativeZero, Number::NegativeInfinity) => Number::NegativeZero,
        }
    }
}

impl Neg for Number {
    type Output = Number;
    fn neg(self) -> Number {
        match self {
            Number::Finite(d) => {
                if d.is_zero() {
                    Number::NegativeZero // -(+0) = -0
                } else {
                    Number::Finite(-d)
                }
            }
            Number::NegativeZero => Number::ZERO, // -(-0) = +0
            Number::NaN => Number::NaN,
            Number::PositiveInfinity => Number::NegativeInfinity,
            Number::NegativeInfinity => Number::PositiveInfinity,
        }
    }
}

// Assignment operators
impl AddAssign for Number {
    fn add_assign(&mut self, rhs: Number) {
        *self = self.clone() + rhs;
    }
}

impl SubAssign for Number {
    fn sub_assign(&mut self, rhs: Number) {
        *self = self.clone() - rhs;
    }
}

impl MulAssign for Number {
    fn mul_assign(&mut self, rhs: Number) {
        *self = self.clone() * rhs;
    }
}

impl DivAssign for Number {
    fn div_assign(&mut self, rhs: Number) {
        *self = self.clone() / rhs;
    }
}

impl RemAssign for Number {
    fn rem_assign(&mut self, rhs: Number) {
        *self = self.clone() % rhs;
    }
}

// Bitwise operators (convert to i32 like JS)
impl BitAnd for Number {
    type Output = Number;
    fn bitand(self, rhs: Number) -> Number {
        // JavaScript converts numbers to 32-bit signed integers for bitwise operations
        // NegativeZero is handled in to_i32() which returns 0
        let a = self.to_i32_js_coerce();
        let b = rhs.to_i32_js_coerce();
        Number::from(a & b)
    }
}

impl BitOr for Number {
    type Output = Number;
    fn bitor(self, rhs: Number) -> Number {
        let a = self.to_i32_js_coerce();
        let b = rhs.to_i32_js_coerce();
        Number::from(a | b)
    }
}

impl BitXor for Number {
    type Output = Number;
    fn bitxor(self, rhs: Number) -> Number {
        let a = self.to_i32_js_coerce();
        let b = rhs.to_i32_js_coerce();
        Number::from(a ^ b)
    }
}

impl Not for Number {
    type Output = Number;
    fn not(self) -> Number {
        let a = self.to_i32_js_coerce();
        Number::from(!a)
    }
}

impl Shl<Number> for Number {
    type Output = Number;
    fn shl(self, rhs: Number) -> Number {
        let a = self.to_i32_js_coerce();
        let b = rhs.to_u32_js_coerce() & 0x1f; // JavaScript masks to 5 bits
        Number::from(a << b)
    }
}

impl Shr<Number> for Number {
    type Output = Number;
    fn shr(self, rhs: Number) -> Number {
        let a = self.to_i32_js_coerce();
        let b = rhs.to_u32_js_coerce() & 0x1f; // JavaScript masks to 5 bits
        Number::from(a >> b)
    }
}

// Bitwise assignment operators
impl BitAndAssign for Number {
    fn bitand_assign(&mut self, rhs: Number) {
        *self = self.clone() & rhs;
    }
}

impl BitOrAssign for Number {
    fn bitor_assign(&mut self, rhs: Number) {
        *self = self.clone() | rhs;
    }
}

impl BitXorAssign for Number {
    fn bitxor_assign(&mut self, rhs: Number) {
        *self = self.clone() ^ rhs;
    }
}

impl ShlAssign<Number> for Number {
    fn shl_assign(&mut self, rhs: Number) {
        *self = self.clone() << rhs;
    }
}

impl ShrAssign<Number> for Number {
    fn shr_assign(&mut self, rhs: Number) {
        *self = self.clone() >> rhs;
    }
}

// Display with JS string conversion semantics
impl Display for Number {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Number::Finite(d) => write!(f, "{}", d),
            Number::NegativeZero => write!(f, "0"), // -0 displays as "0"
            Number::NaN => write!(f, "NaN"),
            Number::PositiveInfinity => write!(f, "Infinity"),
            Number::NegativeInfinity => write!(f, "-Infinity"),
        }
    }
}

// Parse from string with JS semantics
impl FromStr for Number {
    type Err = ();

    fn from_str(s: &str) -> Result<Number, Self::Err> {
        let s = s.trim();

        // Handle special JavaScript string values
        match s {
            "NaN" => Ok(Number::NaN),
            "Infinity" => Ok(Number::PositiveInfinity),
            "-Infinity" => Ok(Number::NegativeInfinity),
            "-0" => Ok(Number::NegativeZero),
            "" => Ok(Number::zero()), // Empty string converts to 0 in JS
            _ => {
                // Try to parse as Decimal first
                if let Ok(d) = Decimal::from_str(s) {
                    Ok(Number::Finite(d))
                } else {
                    // Try to parse as f64 for cases Decimal can't handle
                    if let Ok(f) = f64::from_str(s) {
                        Ok(Number::from(f))
                    } else {
                        // TODO: JavaScript has complex string-to-number conversion rules
                        // This is a simplified version - JS would parse partial numbers
                        // For now, leaving as todo since proper JS string-to-number conversion
                        // requires implementing the full ECMAScript ToNumber algorithm
                        // todo!("Need full JavaScript string-to-number conversion (ECMAScript ToNumber): {:?}", s)
                        Err(())
                    }
                }
            }
        }
    }
}

// Macro to generate From implementations for primitive types
macro_rules! impl_from_primitives {
    ($($t:ty),*) => {
        $(
            impl From<$t> for Number {
                fn from(n: $t) -> Number {
                    Number::Finite(Decimal::from(n))
                }
            }
        )*
    };
}

// Generate From implementations for all primitive number types
impl_from_primitives!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);

// Special From implementations that need custom logic
impl From<Decimal> for Number {
    fn from(d: Decimal) -> Number {
        Number::Finite(d)
    }
}

impl From<f64> for Number {
    fn from(f: f64) -> Number {
        if f.is_nan() {
            Number::NaN
        } else if f.is_infinite() {
            if f.is_sign_positive() {
                Number::PositiveInfinity
            } else {
                Number::NegativeInfinity
            }
        } else if f == 0.0 {
            if f.is_sign_negative() {
                Number::NegativeZero
            } else {
                Number::ZERO
            }
        } else {
            // Convert f64 to Decimal - this might lose precision for very large numbers
            if let Some(d) = Decimal::from_f64(f) {
                Number::Finite(d)
            } else {
                // If conversion fails, fall back to NaN
                Number::NaN
            }
        }
    }
}

impl From<f32> for Number {
    fn from(f: f32) -> Number {
        Number::from(f as f64)
    }
}

impl TryFrom<Number> for i32 {
    type Error = ();

    fn try_from(js_num: Number) -> Result<i32, Self::Error> {
        match js_num {
            Number::Finite(d) => d.to_i32().ok_or(()),
            _ => Err(()), // Can't convert NaN or Infinity
        }
    }
}

impl TryFrom<Number> for u32 {
    type Error = ();

    fn try_from(js_num: Number) -> Result<u32, Self::Error> {
        match js_num {
            Number::Finite(d) => d.to_u32().ok_or(()),
            _ => Err(()),
        }
    }
}

impl TryFrom<Number> for i64 {
    type Error = ();

    fn try_from(js_num: Number) -> Result<i64, Self::Error> {
        match js_num {
            Number::Finite(d) => d.to_i64().ok_or(()),
            _ => Err(()),
        }
    }
}

// Special case for f64 which can represent all our values
impl TryFrom<Number> for f64 {
    type Error = ();

    fn try_from(js_num: Number) -> Result<f64, Self::Error> {
        Ok(js_num.to_f64()) // Never fails
    }
}

// Special case for Decimal
impl TryFrom<Number> for Decimal {
    type Error = ();

    fn try_from(js_num: Number) -> Result<Decimal, Self::Error> {
        js_num.to_decimal().ok_or(())
    }
}

// Macro to generate reference variants for binary operators
macro_rules! forward_ref_binop {
    (impl $trait:ident, $method:ident for $type:ty) => {
        impl $trait<&$type> for $type {
            type Output = $type;
            fn $method(self, rhs: &$type) -> $type {
                $trait::$method(self, rhs.clone())
            }
        }

        impl $trait<$type> for &$type {
            type Output = $type;
            fn $method(self, rhs: $type) -> $type {
                $trait::$method(self.clone(), rhs)
            }
        }

        impl $trait<&$type> for &$type {
            type Output = $type;
            fn $method(self, rhs: &$type) -> $type {
                $trait::$method(self.clone(), rhs.clone())
            }
        }
    };
}

// Generate all reference variants for arithmetic operators
forward_ref_binop!(impl Add, add for Number);
forward_ref_binop!(impl Sub, sub for Number);
forward_ref_binop!(impl Mul, mul for Number);
forward_ref_binop!(impl Div, div for Number);
forward_ref_binop!(impl Rem, rem for Number);

// Generate reference variants for bitwise operators
forward_ref_binop!(impl BitAnd, bitand for Number);
forward_ref_binop!(impl BitOr, bitor for Number);
forward_ref_binop!(impl BitXor, bitxor for Number);
forward_ref_binop!(impl Shl, shl for Number);
forward_ref_binop!(impl Shr, shr for Number);

// num_traits implementations for mathematical operations
impl Zero for Number {
    fn zero() -> Self {
        Number::ZERO
    }

    fn is_zero(&self) -> bool {
        match self {
            Number::Finite(d) => d.is_zero(),
            Number::NegativeZero => true,
            _ => false,
        }
    }
}

impl One for Number {
    fn one() -> Self {
        Number::ONE
    }
}

impl Signed for Number {
    fn abs(&self) -> Self {
        self.clone().abs()
    }

    fn abs_sub(&self, other: &Self) -> Self {
        let diff = self.clone() - other.clone();
        if diff.is_positive() {
            diff
        } else {
            Number::zero()
        }
    }

    fn signum(&self) -> Self {
        match self {
            Number::Finite(d) => {
                if d.is_zero() {
                    Number::zero()
                } else if *d > Decimal::ZERO {
                    Number::one()
                } else {
                    -Number::one()
                }
            }
            Number::NegativeZero => Number::NegativeZero, // signum(-0) = -0
            Number::NaN => Number::NaN,
            Number::PositiveInfinity => Number::one(),
            Number::NegativeInfinity => -Number::one(),
        }
    }

    fn is_positive(&self) -> bool {
        match self {
            Number::Finite(d) => d.is_sign_positive(),
            Number::NegativeZero => false, // -0 is not positive
            Number::PositiveInfinity => true,
            _ => false,
        }
    }

    fn is_negative(&self) -> bool {
        match self {
            Number::Finite(d) => d.is_sign_negative(),
            Number::NegativeZero => true, // -0 is negative
            Number::NegativeInfinity => true,
            _ => false,
        }
    }
}

impl Num for Number {
    type FromStrRadixErr = ();

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        // JavaScript's parseInt-like behavior
        if radix < 2 || radix > 36 {
            return Err(());
        }

        // Try to parse as i64 first, then fall back to f64 if needed
        if let Ok(i) = i64::from_str_radix(str.trim(), radix) {
            Ok(Number::from(i))
        } else {
            // For non-integer values or very large numbers, this is more complex
            // JavaScript parseInt has specific rules about parsing partial numbers
            // TODO: Implement full JavaScript parseInt semantics
            todo!() // Need proper JavaScript parseInt implementation
        }
    }
}

impl ToPrimitive for Number {
    fn to_i64(&self) -> Option<i64> {
        match self {
            Number::Finite(d) => d.to_i64(),
            Number::NegativeZero => Some(0),
            _ => None,
        }
    }

    fn to_u64(&self) -> Option<u64> {
        match self {
            Number::Finite(d) => d.to_u64(),
            Number::NegativeZero => Some(0),
            _ => None,
        }
    }

    fn to_f64(&self) -> Option<f64> {
        Some(self.to_f64()) // Our to_f64 method handles all cases
    }
}

impl FromPrimitive for Number {
    fn from_i64(n: i64) -> Option<Self> {
        Some(Number::from(n))
    }

    fn from_u64(n: u64) -> Option<Self> {
        Some(Number::from(n))
    }

    fn from_f64(n: f64) -> Option<Self> {
        Some(Number::from(n))
    }
}

use std::hash::{Hash, Hasher};

impl Hash for Number {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Number::Finite(d) => {
                0u8.hash(state); // Discriminant
                d.hash(state);
            }
            Number::NaN => {
                1u8.hash(state); // All NaN values hash the same
            }
            Number::PositiveInfinity => {
                2u8.hash(state);
            }
            Number::NegativeInfinity => {
                3u8.hash(state);
            }
            Number::NegativeZero => {
                4u8.hash(state);
            }
        }
    }
}

impl PartialEq for Number {
    fn eq(&self, other: &Number) -> bool {
        match (self, other) {
            // For Rust compatibility, NaN equals itself (breaking JS semantics)
            (Number::NaN, Number::NaN) => true,
            (Number::NaN, _) | (_, Number::NaN) => false,
            (Number::Finite(a), Number::Finite(b)) => a == b,
            (Number::PositiveInfinity, Number::PositiveInfinity) => true,
            (Number::NegativeInfinity, Number::NegativeInfinity) => true,
            (Number::NegativeZero, Number::NegativeZero) => true,
            // +0 equals -0 (maintaining this JS semantic for simplicity)
            (Number::Finite(a), Number::NegativeZero)
            | (Number::NegativeZero, Number::Finite(a)) => a.is_zero(),
            _ => false,
        }
    }
}

impl PartialOrd for Number {
    fn partial_cmp(&self, other: &Number) -> Option<Ordering> {
        match (self, other) {
            // NaN comparisons now return Some when both are NaN
            (Number::NaN, Number::NaN) => Some(Ordering::Equal),
            (Number::NaN, _) => Some(Ordering::Less),
            (_, Number::NaN) => Some(Ordering::Greater),

            (Number::Finite(a), Number::Finite(b)) => a.partial_cmp(b),
            (Number::NegativeInfinity, Number::NegativeInfinity)
            | (Number::PositiveInfinity, Number::PositiveInfinity)
            | (Number::NegativeZero, Number::NegativeZero) => Some(Ordering::Equal),

            // Handle zero equality: +0 == -0
            (Number::Finite(a), Number::NegativeZero) => {
                if a.is_zero() {
                    Some(Ordering::Equal)
                } else {
                    a.partial_cmp(&Decimal::ZERO)
                }
            }
            (Number::NegativeZero, Number::Finite(a)) => {
                if a.is_zero() {
                    Some(Ordering::Equal)
                } else {
                    Decimal::ZERO.partial_cmp(a)
                }
            }

            // Infinities
            (Number::NegativeInfinity, _) => Some(Ordering::Less),
            (_, Number::NegativeInfinity) => Some(Ordering::Greater),
            (Number::PositiveInfinity, _) => Some(Ordering::Greater),
            (_, Number::PositiveInfinity) => Some(Ordering::Less),
        }
    }
}

impl Eq for Number {}

// Total ordering: NaN < -Infinity < finite numbers < +Infinity
// Note: -0 and +0 are treated as equal in this ordering
impl Ord for Number {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            // NaN handling - consistent with PartialEq
            (Number::NaN, Number::NaN) => Ordering::Equal,
            (Number::NaN, _) => Ordering::Less,
            (_, Number::NaN) => Ordering::Greater,

            // Infinities
            (Number::NegativeInfinity, Number::NegativeInfinity) => Ordering::Equal,
            (Number::PositiveInfinity, Number::PositiveInfinity) => Ordering::Equal,
            (Number::NegativeInfinity, _) => Ordering::Less,
            (_, Number::NegativeInfinity) => Ordering::Greater,
            (Number::PositiveInfinity, _) => Ordering::Greater,
            (_, Number::PositiveInfinity) => Ordering::Less,

            // Finite numbers and zeros
            (Number::Finite(a), Number::Finite(b)) => a.cmp(b),
            (Number::NegativeZero, Number::NegativeZero) => Ordering::Equal,

            // Zero equality: treat +0 and -0 as equal
            (Number::Finite(a), Number::NegativeZero) => {
                if a.is_zero() {
                    Ordering::Equal
                } else {
                    a.cmp(&Decimal::ZERO)
                }
            }
            (Number::NegativeZero, Number::Finite(a)) => {
                if a.is_zero() {
                    Ordering::Equal
                } else {
                    Decimal::ZERO.cmp(a)
                }
            }
        }
    }
}

// Default implementation - this is debatable
// JavaScript doesn't have a "default" number, but 0 is reasonable
impl Default for Number {
    fn default() -> Self {
        Number::ZERO
    }
}

// Additional useful traits for collections and generic code:

// Implementing Send and Sync (Decimal is Send + Sync)
unsafe impl Send for Number {}
unsafe impl Sync for Number {}

// Convenience macro for creating Number literals
#[macro_export]
macro_rules! js_dec {
    (NaN) => {
        Number::NaN
    };
    (Infinity) => {
        Number::PositiveInfinity
    };
    (-Infinity) => {
        Number::NegativeInfinity
    };
    (-0) => {
        Number::NegativeZero
    };
    ($n:expr) => {
        Number::from($n)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constants() {
        assert!(Number::NAN.is_nan());
        assert!(Number::POSITIVE_INFINITY.is_positive_infinity());
        assert!(Number::NEGATIVE_INFINITY.is_negative_infinity());
    }

    #[test]
    fn test_arithmetic() {
        let a = js_dec!(5);
        let b = js_dec!(3);
        let _result = a + b; // Should be 8
    }

    #[test]
    fn test_nan_semantics() {
        let nan = Number::NAN;
        assert_ne!(nan, nan); // NaN != NaN in JS
    }

    #[test]
    fn test_macro_convenience() {
        let _a = js_dec!(3.14);
        let _b = js_dec!(42);
        let _nan = js_dec!(NaN);
        let _inf = js_dec!(Infinity);
        let _neg_inf = js_dec!(-Infinity);
    }

    #[test]
    fn test_ergonomic_usage() {
        use num_traits::{One, Zero};

        // Natural arithmetic
        let a = Number::from(10);
        let b = Number::from(3);
        let _result = &a + &b * Number::one(); // Reference operations work

        // num_traits integration
        let _zero = Number::zero();
        let _one = Number::one();

        // Method chaining
        let _result = Number::from(16).sqrt().abs();
    }
}

#[cfg(test)]
mod js_semantics_tests {
    use super::*;
    use std::str::FromStr;

    macro_rules! assert_js_eq {
        ($left:expr, $right:expr) => {
            assert!(
                $left.js_strict_equals(&$right),
                "Expected {:?} === {:?} (JS strict equality)",
                $left,
                $right
            );
        };
    }

    macro_rules! assert_js_ne {
        ($left:expr, $right:expr) => {
            assert!(
                !$left.js_strict_equals(&$right),
                "Expected {:?} !== {:?} (JS strict inequality)",
                $left,
                $right
            );
        };
    }

    // =================== CONSTANTS AND BASIC PROPERTIES ===================

    #[test]
    fn test_constants() {
        assert!(Number::NAN.is_nan());
        assert!(Number::POSITIVE_INFINITY.is_positive_infinity());
        assert!(Number::NEGATIVE_INFINITY.is_negative_infinity());
        assert!(Number::ZERO.is_finite());
        assert!(Number::ONE.is_finite());

        assert!(!Number::NAN.is_finite());
        assert!(!Number::POSITIVE_INFINITY.is_finite());
        assert!(!Number::NEGATIVE_INFINITY.is_finite());
    }

    #[test]
    fn test_type_predicates() {
        let finite = js_dec!(42.5);
        let nan = Number::NAN;
        let pos_inf = Number::POSITIVE_INFINITY;
        let neg_inf = Number::NEGATIVE_INFINITY;

        // is_finite
        assert!(finite.is_finite());
        assert!(!nan.is_finite());
        assert!(!pos_inf.is_finite());
        assert!(!neg_inf.is_finite());

        // is_infinite
        assert!(!finite.is_infinite());
        assert!(!nan.is_infinite());
        assert!(pos_inf.is_infinite());
        assert!(neg_inf.is_infinite());

        // is_nan
        assert!(!finite.is_nan());
        assert!(nan.is_nan());
        assert!(!pos_inf.is_nan());
        assert!(!neg_inf.is_nan());
    }

    // =================== BASIC ARITHMETIC ===================

    #[test]
    fn test_basic_arithmetic() {
        let a = js_dec!(5);
        let b = js_dec!(3);

        assert_js_eq!(a + b, js_dec!(8));
        assert_js_eq!(a - b, js_dec!(2));
        assert_js_eq!(a * b, js_dec!(15));
        // Division precision test - just check it's approximately correct
        let result = a / b;
        assert!(result.is_finite());
        let expected = Number::from(Decimal::from_str("1.6666666666666666667").unwrap());
        let diff = (result - expected).abs();
        assert!(diff < Number::from(Decimal::from_str("0.0000000000000000001").unwrap()));
        assert_js_eq!(a % b, js_dec!(2));

        assert_js_eq!(-a, js_dec!(-5));
        assert_js_eq!(-(-a), js_dec!(5));
    }

    #[test]
    fn test_decimal_precision() {
        let a = Number::from(Decimal::from_str("0.1").unwrap());
        let b = Number::from(Decimal::from_str("0.2").unwrap());
        let expected = Number::from(Decimal::from_str("0.3").unwrap());

        // This should work with Decimal (unlike floating point)
        assert_js_eq!(a + b, expected);
    }

    // =================== SPECIAL VALUE ARITHMETIC ===================

    #[test]
    fn test_nan_arithmetic() {
        let nan = Number::NAN;
        let finite = js_dec!(5);
        let inf = Number::POSITIVE_INFINITY;

        // NaN + anything = NaN
        assert!((nan + finite).is_nan());
        assert!((finite + nan).is_nan());
        assert!((nan + inf).is_nan());
        assert!((nan + nan).is_nan());

        // NaN with all operations
        assert!((nan - finite).is_nan());
        assert!((nan * finite).is_nan());
        assert!((nan / finite).is_nan());
        assert!((nan % finite).is_nan());
        assert!((-nan).is_nan());
    }

    #[test]
    fn test_infinity_arithmetic() {
        let pos_inf = Number::POSITIVE_INFINITY;
        let neg_inf = Number::NEGATIVE_INFINITY;
        let finite = js_dec!(5);
        let zero = js_dec!(0);

        // Infinity + finite = Infinity
        assert_js_eq!(pos_inf + finite, pos_inf);
        assert_js_eq!(neg_inf + finite, neg_inf);
        assert_js_eq!(finite + pos_inf, pos_inf);

        // Infinity - Infinity = NaN
        assert!((pos_inf - pos_inf).is_nan());
        assert!((neg_inf - neg_inf).is_nan());

        // Infinity + (-Infinity) = NaN
        assert!((pos_inf + neg_inf).is_nan());

        // Infinity * finite = Infinity (with sign rules)
        assert_js_eq!(pos_inf * finite, pos_inf);
        assert_js_eq!(neg_inf * finite, neg_inf);
        assert_js_eq!(pos_inf * js_dec!(-5), neg_inf);

        // Infinity * 0 = NaN
        assert!((pos_inf * zero).is_nan());
        assert!((neg_inf * zero).is_nan());

        // Infinity / finite = Infinity
        assert_js_eq!(pos_inf / finite, pos_inf);
        assert_js_eq!(neg_inf / finite, neg_inf);

        // Infinity / Infinity = NaN
        assert!((pos_inf / pos_inf).is_nan());
        assert!((pos_inf / neg_inf).is_nan());

        // finite / Infinity = 0
        assert_js_eq!(finite / pos_inf, zero);
        assert_js_eq!(finite / neg_inf, zero);
    }

    #[test]
    fn test_division_by_zero() {
        let pos = js_dec!(5);
        let neg = js_dec!(-5);
        let zero = js_dec!(0);

        // Positive / 0 = +Infinity
        assert_js_eq!(pos / zero, Number::POSITIVE_INFINITY);

        // Negative / 0 = -Infinity
        assert_js_eq!(neg / zero, Number::NEGATIVE_INFINITY);

        // 0 / 0 = NaN
        assert!((zero / zero).is_nan());
    }

    #[test]
    fn test_modulo_special_cases() {
        let finite = js_dec!(5);
        let zero = js_dec!(0);
        let inf = Number::POSITIVE_INFINITY;

        // x % 0 = NaN
        assert!((finite % zero).is_nan());

        // Infinity % x = NaN
        assert!((inf % finite).is_nan());

        // x % Infinity = x
        assert_js_eq!(finite % inf, finite);

        // Test negative modulo behavior (JS-specific)
        assert_js_eq!(js_dec!(-5) % js_dec!(3), js_dec!(-2));
        assert_js_eq!(js_dec!(5) % js_dec!(-3), js_dec!(2));
        assert_js_eq!(js_dec!(-5) % js_dec!(-3), js_dec!(-2));
    }

    // =================== COMPARISON SEMANTICS ===================

    #[test]
    fn test_nan_comparison_semantics() {
        let nan = Number::NAN;
        let finite = js_dec!(5);

        // NaN != NaN (most important JS quirk)
        assert_js_ne!(nan, nan);
        assert_ne!(nan, nan); // Rust PartialEq should also follow this

        // NaN != anything
        assert_js_ne!(nan, finite);
        assert_js_ne!(finite, nan);
        assert_js_ne!(nan, Number::POSITIVE_INFINITY);

        // NaN comparisons always return None/false
        assert_eq!(nan.partial_cmp(&finite), None);
        assert_eq!(finite.partial_cmp(&nan), None);
        assert_eq!(nan.partial_cmp(&nan), None);

        assert_eq!(nan.js_less_than(&finite), None);
        assert_eq!(finite.js_less_than(&nan), None);
    }

    #[test]
    fn test_infinity_comparison() {
        let pos_inf = Number::POSITIVE_INFINITY;
        let neg_inf = Number::NEGATIVE_INFINITY;
        let finite = js_dec!(1000000);

        // +Infinity > everything except +Infinity
        assert!(pos_inf > finite);
        assert!(pos_inf > neg_inf);
        assert_js_eq!(pos_inf, pos_inf);

        // -Infinity < everything except -Infinity
        assert!(neg_inf < finite);
        assert!(neg_inf < pos_inf);
        assert_js_eq!(neg_inf, neg_inf);
    }

    #[test]
    fn test_finite_comparison() {
        let a = js_dec!(5);
        let b = js_dec!(3);
        let c = js_dec!(5);

        assert!(a > b);
        assert!(b < a);
        assert_js_eq!(a, c);
        assert!(a >= c);
        assert!(a <= c);
        assert!(a >= b);
        assert!(b <= a);
    }

    #[test]
    fn test_js_equality_vs_strict_equality() {
        let a = js_dec!(5);
        let b = js_dec!(5.0);
        let c = js_dec!(3);

        // Should be the same for numbers
        assert_eq!(a.js_equals(&b), a.js_strict_equals(&b));
        assert_eq!(a.js_equals(&c), a.js_strict_equals(&c));

        // Test with special values
        let nan = Number::NAN;
        assert!(!nan.js_equals(&nan));
        assert!(!nan.js_strict_equals(&nan));
    }

    // =================== BITWISE OPERATIONS ===================

    #[test]
    fn test_bitwise_operations() {
        let a = js_dec!(12); // 0b1100
        let b = js_dec!(5); // 0b0101

        // Basic bitwise operations
        assert_js_eq!(a & b, js_dec!(4)); // 0b0100
        assert_js_eq!(a | b, js_dec!(13)); // 0b1101
        assert_js_eq!(a ^ b, js_dec!(9)); // 0b1001

        // Bitwise NOT
        assert_js_eq!(!a, js_dec!(-13)); // ~12 = -13 in two's complement

        // Shifts
        assert_js_eq!(a << js_dec!(1), js_dec!(24)); // 12 << 1 = 24
        assert_js_eq!(a >> js_dec!(1), js_dec!(6)); // 12 >> 1 = 6
    }

    #[test]
    fn test_bitwise_with_decimals() {
        // JS converts to i32 for bitwise ops, truncating decimals
        let decimal = js_dec!(12.7);
        let integer = js_dec!(5);

        assert_js_eq!(decimal & integer, js_dec!(4)); // 12 & 5 = 4
        assert_js_eq!(decimal | integer, js_dec!(13)); // 12 | 5 = 13
    }

    #[test]
    fn test_bitwise_with_special_values() {
        let nan = Number::NAN;
        let inf = Number::POSITIVE_INFINITY;
        let finite = js_dec!(5);

        // Bitwise with NaN -> 0 (NaN converts to 0 in bitwise operations)
        assert_js_eq!(nan & finite, js_dec!(0));
        assert_js_eq!(finite & nan, js_dec!(0));

        // Bitwise with Infinity -> treat as 0
        assert_js_eq!(inf & finite, js_dec!(0));
        assert_js_eq!(finite & inf, js_dec!(0));
    }

    #[test]
    fn test_unsigned_right_shift() {
        let a = js_dec!(-1);
        let shift = js_dec!(1);

        // >>> is different from >> for negative numbers
        assert_js_eq!(a.unsigned_right_shift(shift), js_dec!(2147483647));

        let b = js_dec!(8);
        assert_js_eq!(b.unsigned_right_shift(js_dec!(2)), js_dec!(2));
    }

    // =================== MATHEMATICAL FUNCTIONS ===================

    #[test]
    fn test_abs() {
        assert_js_eq!(js_dec!(5).abs(), js_dec!(5));
        assert_js_eq!(js_dec!(-5).abs(), js_dec!(5));
        assert_js_eq!(js_dec!(0).abs(), js_dec!(0));

        // Special values
        assert!(Number::NAN.abs().is_nan());
        assert_js_eq!(Number::POSITIVE_INFINITY.abs(), Number::POSITIVE_INFINITY);
        assert_js_eq!(Number::NEGATIVE_INFINITY.abs(), Number::POSITIVE_INFINITY);
    }

    #[test]
    fn test_floor_ceil_round_trunc() {
        let positive = js_dec!(3.7);
        let negative = js_dec!(-3.7);

        // Floor
        assert_js_eq!(positive.floor(), js_dec!(3));
        assert_js_eq!(negative.floor(), js_dec!(-4));

        // Ceil
        assert_js_eq!(positive.ceil(), js_dec!(4));
        assert_js_eq!(negative.ceil(), js_dec!(-3));

        // Round (ties to even in JS)
        assert_js_eq!(js_dec!(3.5).round(), js_dec!(4));
        assert_js_eq!(js_dec!(-3.5).round(), js_dec!(-3));

        // Trunc
        assert_js_eq!(positive.trunc(), js_dec!(3));
        assert_js_eq!(negative.trunc(), js_dec!(-3));

        // Special values
        assert!(Number::NAN.floor().is_nan());
        assert_js_eq!(Number::POSITIVE_INFINITY.floor(), Number::POSITIVE_INFINITY);
    }

    #[test]
    fn test_sqrt() {
        assert_js_eq!(js_dec!(9).sqrt(), js_dec!(3));
        assert_js_eq!(js_dec!(0).sqrt(), js_dec!(0));

        // Negative -> NaN
        assert!(js_dec!(-1).sqrt().is_nan());

        // Special values
        assert!(Number::NAN.sqrt().is_nan());
        assert_js_eq!(Number::POSITIVE_INFINITY.sqrt(), Number::POSITIVE_INFINITY);
        assert!(Number::NEGATIVE_INFINITY.sqrt().is_nan());
    }

    #[test]
    fn test_pow() {
        assert_js_eq!(js_dec!(2).pow(js_dec!(3)), js_dec!(8));
        assert_js_eq!(js_dec!(9).pow(js_dec!(0.5)), js_dec!(3)); // sqrt

        // Special cases
        assert_js_eq!(js_dec!(0).pow(js_dec!(0)), js_dec!(1)); // 0^0 = 1 in JS
        assert_js_eq!(js_dec!(1).pow(Number::POSITIVE_INFINITY), js_dec!(1));
        assert!(js_dec!(-1).pow(js_dec!(0.5)).is_nan()); // Negative^fractional = NaN

        // Infinity cases
        assert_js_eq!(
            Number::POSITIVE_INFINITY.pow(js_dec!(2)),
            Number::POSITIVE_INFINITY
        );
        assert_js_eq!(Number::POSITIVE_INFINITY.pow(js_dec!(0)), js_dec!(1));
        assert!(Number::POSITIVE_INFINITY.pow(Number::NAN).is_nan());
    }

    #[test]
    fn test_trigonometric_functions() {
        // Basic trig (approximate due to decimal precision)
        let pi_half = Number::from(Decimal::from_str("1.5707963267948966").unwrap());

        assert!((js_dec!(0).sin() - js_dec!(0)).abs().to_f64() < 1e-10);
        assert!((pi_half.sin() - js_dec!(1)).abs().to_f64() < 1e-10);
        assert!((js_dec!(0).cos() - js_dec!(1)).abs().to_f64() < 1e-10);

        // Special values
        assert!(Number::NAN.sin().is_nan());
        assert!(Number::POSITIVE_INFINITY.sin().is_nan());
        assert!(Number::NEGATIVE_INFINITY.cos().is_nan());
    }

    #[test]
    fn test_logarithms() {
        assert_js_eq!(js_dec!(1).log(), js_dec!(0));
        assert!((js_dec!(10).log10() - js_dec!(1)).abs().to_f64() < 1e-10);
        assert!((js_dec!(8).log2() - js_dec!(3)).abs().to_f64() < 1e-10);

        // Negative -> NaN
        assert!(js_dec!(-1).log().is_nan());
        assert!(js_dec!(0).log().is_negative_infinity());

        // Special values
        assert!(Number::NAN.log().is_nan());
        assert_js_eq!(Number::POSITIVE_INFINITY.log(), Number::POSITIVE_INFINITY);
    }

    // =================== TYPE CONVERSIONS ===================

    #[test]
    fn test_to_i32_conversion() {
        assert_eq!(js_dec!(42).to_i32_js_coerce(), 42);
        assert_eq!(js_dec!(42.7).to_i32_js_coerce(), 42); // Truncation
        assert_eq!(js_dec!(-42.7).to_i32_js_coerce(), -42);

        // Large numbers wrap
        // assert_eq!(js_num!(4294967296).to_i32(), 0); // 2^32 wraps to 0
        // assert_eq!(js_num!(4294967297).to_i32(), 1);

        // Special values
        assert_eq!(Number::NAN.to_i32_js_coerce(), 0);
        assert_eq!(Number::POSITIVE_INFINITY.to_i32_js_coerce(), 0);
        assert_eq!(Number::NEGATIVE_INFINITY.to_i32_js_coerce(), 0);
    }

    #[test]
    fn test_to_u32_conversion() {
        assert_eq!(js_dec!(42).to_u32_js_coerce(), 42);
        assert_eq!(js_dec!(-1).to_u32_js_coerce(), 4294967295); // Wraps to max u32

        // Special values become 0
        assert_eq!(Number::NAN.to_u32_js_coerce(), 0);
        assert_eq!(Number::POSITIVE_INFINITY.to_u32_js_coerce(), 0);
    }

    #[test]
    fn test_to_f64_conversion() {
        assert_eq!(js_dec!(42.5).to_f64(), 42.5);
        assert!(Number::NAN.to_f64().is_nan());
        assert_eq!(Number::POSITIVE_INFINITY.to_f64(), f64::INFINITY);
        assert_eq!(Number::NEGATIVE_INFINITY.to_f64(), f64::NEG_INFINITY);
    }

    #[test]
    fn test_from_f64_conversion() {
        assert_js_eq!(Number::from(42.5), js_dec!(42.5));
        assert!(Number::from(f64::NAN).is_nan());
        assert_js_eq!(Number::from(f64::INFINITY), Number::POSITIVE_INFINITY);
        assert_js_eq!(Number::from(f64::NEG_INFINITY), Number::NEGATIVE_INFINITY);
    }

    #[test]
    fn test_try_from_conversions() {
        let finite = js_dec!(42);
        let nan = Number::NAN;

        assert_eq!(i64::try_from(finite), Ok(42));
        assert_eq!(i64::try_from(nan), Err(()));
        assert_eq!(i64::try_from(Number::POSITIVE_INFINITY), Err(()));
    }

    // =================== STRING CONVERSIONS ===================

    #[test]
    fn test_to_js_string() {
        assert_eq!(js_dec!(42).to_js_string(), "42");
        assert_eq!(js_dec!(42.5).to_js_string(), "42.5");
        assert_eq!(js_dec!(0).to_js_string(), "0");
        assert_eq!(js_dec!(-0).to_js_string(), "0"); // -0 becomes "0"

        // Special values
        assert_eq!(Number::NAN.to_js_string(), "NaN");
        assert_eq!(Number::POSITIVE_INFINITY.to_js_string(), "Infinity");
        assert_eq!(Number::NEGATIVE_INFINITY.to_js_string(), "-Infinity");

        // Large/small numbers (scientific notation handling)
        assert_eq!(js_dec!(1e21).to_js_string(), "1e+21");
        assert_eq!(js_dec!(1e-7).to_js_string(), "1e-7");
    }

    #[test]
    fn test_display_trait() {
        assert_eq!(format!("{}", js_dec!(42)), "42");
        assert_eq!(format!("{}", Number::NAN), "NaN");
        assert_eq!(format!("{}", Number::POSITIVE_INFINITY), "Infinity");
    }

    #[test]
    fn test_from_str() {
        assert_js_eq!(Number::from_str("42").unwrap(), js_dec!(42));
        assert_js_eq!(Number::from_str("42.5").unwrap(), js_dec!(42.5));
        assert_js_eq!(Number::from_str("-42").unwrap(), js_dec!(-42));

        // Whitespace handling (JS trims)
        assert_js_eq!(Number::from_str("  42  ").unwrap(), js_dec!(42));

        // Special values
        assert!(Number::from_str("NaN").unwrap().is_nan());
        assert_js_eq!(
            Number::from_str("Infinity").unwrap(),
            Number::POSITIVE_INFINITY
        );
        assert_js_eq!(
            Number::from_str("-Infinity").unwrap(),
            Number::NEGATIVE_INFINITY
        );

        // Invalid strings
        assert!(Number::from_str("not a number").is_err());
        assert!(Number::from_str("").is_err());
    }

    // =================== TRUTHINESS SEMANTICS ===================

    #[test]
    fn test_truthiness() {
        // Falsy values
        assert!(js_dec!(0).is_falsy());
        assert!(js_dec!(-0).is_falsy());
        assert!(Number::NAN.is_falsy());

        // Truthy values
        assert!(js_dec!(1).is_truthy());
        assert!(js_dec!(-1).is_truthy());
        assert!(js_dec!(0.1).is_truthy());
        assert!(Number::POSITIVE_INFINITY.is_truthy());
        assert!(Number::NEGATIVE_INFINITY.is_truthy());

        // Inverse relationship
        assert_eq!(js_dec!(0).is_truthy(), !js_dec!(0).is_falsy());
        assert_eq!(js_dec!(42).is_truthy(), !js_dec!(42).is_falsy());
    }

    // =================== ASSIGNMENT OPERATORS ===================

    #[test]
    fn test_assignment_operators() {
        let mut a = js_dec!(5);

        a += js_dec!(3);
        assert_js_eq!(a, js_dec!(8));

        a -= js_dec!(2);
        assert_js_eq!(a, js_dec!(6));

        a *= js_dec!(2);
        assert_js_eq!(a, js_dec!(12));

        a /= js_dec!(3);
        assert_js_eq!(a, js_dec!(4));

        a %= js_dec!(3);
        assert_js_eq!(a, js_dec!(1));
    }

    #[test]
    fn test_bitwise_assignment_operators() {
        let mut a = js_dec!(12); // 0b1100

        a &= js_dec!(5); // 0b0101
        assert_js_eq!(a, js_dec!(4)); // 0b0100

        a |= js_dec!(8); // 0b1000
        assert_js_eq!(a, js_dec!(12)); // 0b1100

        a ^= js_dec!(3); // 0b0011
        assert_js_eq!(a, js_dec!(15)); // 0b1111

        a <<= js_dec!(1);
        assert_js_eq!(a, js_dec!(30));

        a >>= js_dec!(2);
        assert_js_eq!(a, js_dec!(7));
    }

    // =================== INCREMENT/DECREMENT ===================

    #[test]
    fn test_increment_decrement() {
        let a = js_dec!(5);

        assert_js_eq!(a.increment(), js_dec!(6));
        assert_js_eq!(a.decrement(), js_dec!(4));

        // Special values
        assert!(Number::NAN.increment().is_nan());
        assert_js_eq!(
            Number::POSITIVE_INFINITY.increment(),
            Number::POSITIVE_INFINITY
        );
        assert_js_eq!(
            Number::NEGATIVE_INFINITY.decrement(),
            Number::NEGATIVE_INFINITY
        );
    }

    // =================== EDGE CASES AND COMPLEX SCENARIOS ===================

    #[test]
    fn test_complex_arithmetic_chains() {
        // Test operator precedence and associativity
        let result = js_dec!(2) + js_dec!(3) * js_dec!(4); // Should be 14, not 20
        assert_js_eq!(result, js_dec!(14));

        // Mixed special values
        let complex = Number::POSITIVE_INFINITY - Number::POSITIVE_INFINITY + js_dec!(5);
        assert!(complex.is_nan()); // Inf - Inf = NaN, NaN + 5 = NaN
    }

    #[test]
    fn test_precision_edge_cases() {
        // Very large numbers - use a number that Decimal can handle
        let large = Number::from(Decimal::from_str("999999999999999999999999999").unwrap());
        assert!(large.is_finite());

        // Very small numbers
        let small = Number::from(Decimal::from_str("0.000000000000000000000000001").unwrap());
        assert!(small.is_finite());
        assert!(small > js_dec!(0));
    }

    #[test]
    fn test_js_safe_integer_range() {
        // JavaScript's MAX_SAFE_INTEGER is 2^53 - 1
        let max_safe = js_dec!(9007199254740991_i64);
        let beyond_safe = js_dec!(9007199254740992_i64);

        assert!(max_safe.is_finite());
        assert!(beyond_safe.is_finite());
        // Note: Decimal should handle these accurately unlike f64
    }

    #[test]
    fn test_zero_edge_cases() {
        let pos_zero = js_dec!(0);
        let neg_zero = js_dec!(-0);

        // Should be equal in most contexts
        assert_js_eq!(pos_zero, neg_zero);

        // But different in some operations
        assert_js_eq!(js_dec!(1) / pos_zero, Number::POSITIVE_INFINITY);
        assert_js_eq!(js_dec!(1) / neg_zero, Number::NEGATIVE_INFINITY);
    }

    #[test]
    fn test_reference_operations() {
        // Test that reference operations work correctly
        let a = js_dec!(5);
        let b = js_dec!(3);

        assert_js_eq!(&a + &b, js_dec!(8));
        assert_js_eq!(a + &b, js_dec!(8));
        assert_js_eq!(&a + b, js_dec!(8));
    }

    #[test]
    fn test_error_conditions() {
        // Operations that should never panic
        let nan = Number::NAN;
        let inf = Number::POSITIVE_INFINITY;
        let finite = js_dec!(42);

        // These should all return well-defined results, never panic
        let _results = vec![
            nan + inf,
            inf - inf,
            nan * js_dec!(0),
            finite / js_dec!(0),
            js_dec!(0) / js_dec!(0),
            inf % finite,
            nan.sqrt(),
            js_dec!(-1).sqrt(),
            js_dec!(0).log(),
            js_dec!(-1).log(),
        ];

        // All should succeed without panicking
    }

    // =================== PROPERTY-BASED TESTS ===================

    #[test]
    fn test_arithmetic_properties() {
        let a = js_dec!(7);
        let b = js_dec!(3);
        let c = js_dec!(2);

        // Associativity (when no special values involved)
        assert_js_eq!((a + b) + c, a + (b + c));
        assert_js_eq!((a * b) * c, a * (b * c));

        // Commutativity
        assert_js_eq!(a + b, b + a);
        assert_js_eq!(a * b, b * a);

        // Distributivity
        assert_js_eq!(a * (b + c), (a * b) + (a * c));

        // Identity elements
        assert_js_eq!(a + js_dec!(0), a);
        assert_js_eq!(a * js_dec!(1), a);

        // Inverse elements
        assert_js_eq!(a + (-a), js_dec!(0));
        assert_js_eq!(a / a, js_dec!(1)); // when a != 0
    }

    #[test]
    fn test_comparison_properties() {
        let a = js_dec!(5);
        let b = js_dec!(3);

        // Transitivity
        if a > b && b > js_dec!(1) {
            assert!(a > js_dec!(1));
        }

        // Antisymmetry
        assert!(!(a > b && b > a));

        // Reflexivity for equality
        assert_js_eq!(a, a);

        // Note: NaN breaks many of these properties, which is expected
    }
}
