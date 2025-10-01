use crate::Number;
use rust_decimal::Decimal;

use num_traits::ToPrimitive;
use std::str::FromStr;

impl Number {
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

    pub fn increment(self) -> Number {
        self + Number::ONE
    }

    pub fn decrement(self) -> Number {
        self - Number::ONE
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

    /// Convert to primitive value (used in JS type coercion)
    pub fn to_primitive(&self) -> Number {
        self.clone() // Numbers are already primitive
    }
}
