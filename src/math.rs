use crate::{Number, NumericValue};
use rust_decimal::Decimal;
use num_rational::Ratio;

use num_traits::{FromPrimitive, Signed, ToPrimitive, Zero};
use std::str::FromStr;

impl NumericValue {
    // Mathematical functions following JS semantics
    pub fn abs(self) -> NumericValue {
        match self {
            NumericValue::Rational(r) => NumericValue::Rational(r.abs()),
            NumericValue::Decimal(d) => NumericValue::Decimal(d.abs()),
            NumericValue::BigDecimal(bd) => NumericValue::BigDecimal(bd.abs()),
            NumericValue::NegativeZero => NumericValue::ZERO, // abs(-0) = +0
            NumericValue::NaN => NumericValue::NaN,
            NumericValue::PositiveInfinity => NumericValue::PositiveInfinity,
            NumericValue::NegativeInfinity => NumericValue::PositiveInfinity,
        }
    }

    pub fn floor(self) -> NumericValue {
        match self {
            NumericValue::Rational(_) | NumericValue::Decimal(_) | NumericValue::BigDecimal(_) => {
                let f = self.to_f64();
                NumericValue::from(f.floor())
            }
            NumericValue::NegativeZero => NumericValue::NegativeZero, // floor(-0) = -0
            NumericValue::NaN => NumericValue::NaN,
            NumericValue::PositiveInfinity => NumericValue::PositiveInfinity,
            NumericValue::NegativeInfinity => NumericValue::NegativeInfinity,
        }
    }

    pub fn ceil(self) -> NumericValue {
        match self {
            NumericValue::Rational(_) | NumericValue::Decimal(_) | NumericValue::BigDecimal(_) => {
                let f = self.to_f64();
                NumericValue::from(f.ceil())
            }
            NumericValue::NegativeZero => NumericValue::NegativeZero, // ceil(-0) = -0
            NumericValue::NaN => NumericValue::NaN,
            NumericValue::PositiveInfinity => NumericValue::PositiveInfinity,
            NumericValue::NegativeInfinity => NumericValue::NegativeInfinity,
        }
    }

    pub fn round(self) -> NumericValue {
        match self {
            NumericValue::Rational(_) | NumericValue::Decimal(_) | NumericValue::BigDecimal(_) => {
                // JavaScript round: rounds to nearest integer, ties away from zero
                // For -3.5, should round to -3 (away from zero)
                let f = self.to_f64();
                let rounded = if f >= 0.0 {
                    (f + 0.5).floor()
                } else {
                    // For negative numbers, round ties away from zero
                    // -3.5 should become -3, not -4
                    // Use: (f + 0.5).ceil() for negative numbers
                    (f + 0.5).ceil()
                };
                NumericValue::from(rounded)
            }
            NumericValue::NegativeZero => NumericValue::NegativeZero, // round(-0) = -0
            NumericValue::NaN => NumericValue::NaN,
            NumericValue::PositiveInfinity => NumericValue::PositiveInfinity,
            NumericValue::NegativeInfinity => NumericValue::NegativeInfinity,
        }
    }

    pub fn round_dp(self, dp: u32) -> NumericValue {
        match self {
            NumericValue::Rational(_) => unimplemented!("Rational round_dp not yet implemented"),
            NumericValue::Decimal(d) => NumericValue::Decimal(d.round_dp(dp)),
            NumericValue::BigDecimal(_) => {
                unimplemented!("BigDecimal round_dp not yet implemented")
            }
            NumericValue::NegativeZero => NumericValue::NegativeZero, // round(-0) = -0
            NumericValue::NaN => NumericValue::NaN,
            NumericValue::PositiveInfinity => NumericValue::PositiveInfinity,
            NumericValue::NegativeInfinity => NumericValue::NegativeInfinity,
        }
    }

    pub fn trunc(self) -> NumericValue {
        match self {
            NumericValue::Rational(r) => {
                if r.is_integer() {
                    NumericValue::Rational(r)
                } else {
                    NumericValue::Rational(r.trunc())
                }
            }
            NumericValue::Decimal(d) => NumericValue::Decimal(d.trunc()),
            NumericValue::BigDecimal(_) => unimplemented!("BigDecimal trunc not yet implemented"),
            NumericValue::NegativeZero => NumericValue::NegativeZero, // trunc(-0) = -0
            NumericValue::NaN => NumericValue::NaN,
            NumericValue::PositiveInfinity => NumericValue::PositiveInfinity,
            NumericValue::NegativeInfinity => NumericValue::NegativeInfinity,
        }
    }

    pub fn sqrt(self) -> NumericValue {
        match self {
            NumericValue::Rational(r) => {
                // Check for negative (NaN) and zero first
                if r < Ratio::from_integer(0) {
                    return NumericValue::NaN;
                }
                if r.is_zero() {
                    return NumericValue::Rational(Ratio::from_integer(0));
                }

                // Check for perfect square using integer arithmetic only
                let numer = *r.numer();
                let denom = *r.denom();

                // Helper function to check if a number is a perfect square using binary search
                fn is_perfect_square(n: i64) -> Option<i64> {
                    if n < 0 {
                        return None;
                    }
                    if n == 0 || n == 1 {
                        return Some(n);
                    }

                    // Binary search for the square root
                    let mut low = 1i64;
                    let mut high = n;

                    while low <= high {
                        let mid = low + (high - low) / 2;

                        // Avoid overflow by checking mid * mid carefully
                        match mid.checked_mul(mid) {
                            Some(square) if square == n => return Some(mid),
                            Some(square) if square < n => low = mid + 1,
                            _ => high = mid - 1,
                        }
                    }
                    None
                }

                // Check if both numerator and denominator are perfect squares
                if let (Some(numer_sqrt), Some(denom_sqrt)) =
                    (is_perfect_square(numer), is_perfect_square(denom)) {
                    return NumericValue::Rational(Ratio::new(numer_sqrt, denom_sqrt));
                }

                // Not a perfect square - convert to Decimal for approximation
                let decimal = Decimal::from(numer) / Decimal::from(denom);
                NumericValue::Decimal(decimal).sqrt()
            }
            NumericValue::Decimal(d) => {
                if d < Decimal::ZERO {
                    NumericValue::NaN // sqrt of negative number is NaN in JS
                } else if d.is_zero() {
                    NumericValue::Decimal(Decimal::ZERO) // sqrt(0) = 0
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

                    NumericValue::Decimal(x)
                }
            }
            NumericValue::BigDecimal(_) => unimplemented!("BigDecimal sqrt not yet implemented"),
            NumericValue::NegativeZero => NumericValue::ZERO, // sqrt(-0) = +0 in JS
            NumericValue::NaN => NumericValue::NaN,
            NumericValue::PositiveInfinity => NumericValue::PositiveInfinity,
            NumericValue::NegativeInfinity => NumericValue::NaN, // sqrt(-Infinity) = NaN
        }
    }

    pub fn pow(self, exponent: NumericValue) -> NumericValue {
        match (self, exponent) {
            // Rational and BigDecimal - not fully implemented yet
            (NumericValue::Rational(_), _) => unimplemented!("Rational pow not yet implemented"),
            (NumericValue::BigDecimal(_), _) => {
                unimplemented!("BigDecimal pow not yet implemented")
            }
            (_, NumericValue::Rational(_)) => {
                unimplemented!("pow with Rational exponent not yet implemented")
            }
            (_, NumericValue::BigDecimal(_)) => {
                unimplemented!("pow with BigDecimal exponent not yet implemented")
            }

            // Handle NaN cases first
            (NumericValue::NaN, NumericValue::Decimal(exp)) if exp.is_zero() => NumericValue::ONE, // NaN**0 = 1 in JS
            (NumericValue::NaN, _) => NumericValue::NaN,
            (_, NumericValue::NaN) => NumericValue::NaN,

            // Handle 0**0 = 1 (special JS behavior)
            (NumericValue::Decimal(base), NumericValue::Decimal(exp))
                if base.is_zero() && exp.is_zero() =>
            {
                NumericValue::ONE
            }
            (NumericValue::NegativeZero, NumericValue::Decimal(exp)) if exp.is_zero() => {
                NumericValue::ONE
            }
            (NumericValue::Decimal(base), NumericValue::NegativeZero) if base.is_zero() => {
                NumericValue::ONE
            }
            (NumericValue::NegativeZero, NumericValue::NegativeZero) => NumericValue::ONE,

            // Handle zero base cases
            (NumericValue::Decimal(base), NumericValue::Decimal(exp)) if base.is_zero() => {
                if exp > Decimal::ZERO {
                    NumericValue::ZERO
                } else if exp < Decimal::ZERO {
                    NumericValue::POSITIVE_INFINITY
                } else {
                    NumericValue::ONE // 0**0 = 1
                }
            }
            (NumericValue::NegativeZero, NumericValue::Decimal(exp)) => {
                if exp > Decimal::ZERO {
                    // Check if exponent is odd or even
                    if exp.fract().is_zero() {
                        let exp_i64 = exp.to_i64().unwrap_or(0);
                        if exp_i64 % 2 == 1 {
                            NumericValue::NegativeZero // (-0)^odd = -0
                        } else {
                            NumericValue::ZERO // (-0)^even = +0
                        }
                    } else {
                        NumericValue::ZERO // (-0)^fractional = +0
                    }
                } else if exp < Decimal::ZERO {
                    NumericValue::NEGATIVE_INFINITY // (-0)^negative = -∞
                } else {
                    NumericValue::ONE // already handled above
                }
            }

            // Handle infinity exponent cases
            (NumericValue::Decimal(base), NumericValue::PositiveInfinity) => {
                let abs_base = base.abs();
                if abs_base > Decimal::ONE {
                    NumericValue::POSITIVE_INFINITY
                } else if abs_base < Decimal::ONE {
                    NumericValue::ZERO
                } else {
                    NumericValue::ONE // 1**Infinity = 1
                }
            }
            (NumericValue::Decimal(base), NumericValue::NegativeInfinity) => {
                let abs_base = base.abs();
                if abs_base > Decimal::ONE {
                    NumericValue::ZERO
                } else if abs_base < Decimal::ONE {
                    NumericValue::POSITIVE_INFINITY
                } else {
                    NumericValue::ONE // 1**(-Infinity) = 1
                }
            }
            (NumericValue::NegativeZero, NumericValue::PositiveInfinity) => NumericValue::ZERO,
            (NumericValue::NegativeZero, NumericValue::NegativeInfinity) => {
                NumericValue::POSITIVE_INFINITY
            }

            // Handle infinity base cases
            (NumericValue::PositiveInfinity, NumericValue::Decimal(exp)) => {
                if exp > Decimal::ZERO {
                    NumericValue::POSITIVE_INFINITY
                } else if exp < Decimal::ZERO {
                    NumericValue::ZERO
                } else {
                    NumericValue::ONE // Infinity**0 = 1
                }
            }
            (NumericValue::NegativeInfinity, NumericValue::Decimal(exp)) => {
                if exp > Decimal::ZERO {
                    // Check if exponent is odd or even
                    if exp.fract().is_zero() {
                        let exp_i64 = exp.to_i64().unwrap_or(0);
                        if exp_i64 % 2 == 1 {
                            NumericValue::NEGATIVE_INFINITY
                        } else {
                            NumericValue::POSITIVE_INFINITY
                        }
                    } else {
                        NumericValue::NaN // Fractional exponent of negative number
                    }
                } else if exp < Decimal::ZERO {
                    NumericValue::ZERO
                } else {
                    NumericValue::ONE // (-Infinity)**0 = 1
                }
            }

            // Handle infinity ** infinity cases
            (NumericValue::PositiveInfinity, NumericValue::PositiveInfinity) => {
                NumericValue::POSITIVE_INFINITY
            }
            (NumericValue::PositiveInfinity, NumericValue::NegativeInfinity) => NumericValue::ZERO,
            (NumericValue::NegativeInfinity, NumericValue::PositiveInfinity) => {
                NumericValue::POSITIVE_INFINITY
            }
            (NumericValue::NegativeInfinity, NumericValue::NegativeInfinity) => NumericValue::ZERO,

            // Handle finite base and exponent
            (NumericValue::Decimal(base), NumericValue::Decimal(exp)) => {
                // Check for negative base with fractional exponent
                if base < Decimal::ZERO && !exp.fract().is_zero() {
                    return NumericValue::NaN;
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

                        return NumericValue::Decimal(result);
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

                        NumericValue::Decimal(Decimal::ONE / result)
                    } else {
                        // Very large positive exponent - this will likely overflow
                        // For now, return positive infinity for very large results
                        NumericValue::POSITIVE_INFINITY
                    }
                } else {
                    // Handle special case of 0.5 exponent (square root)
                    if exp == Decimal::from_str("0.5").unwrap_or(Decimal::ZERO) {
                        return NumericValue::Decimal(base).sqrt();
                    }

                    // Fractional exponent - use a^b = e^(b * ln(a))
                    // TODO: For high precision, implement ln and exp with Decimal directly
                    let ln_base = NumericValue::Decimal(base).log();
                    let exp_arg = NumericValue::Decimal(exp) * ln_base;
                    exp_arg.exp()
                }
            }

            // Handle cases where exponent is NegativeZero
            (NumericValue::Decimal(_), NumericValue::NegativeZero) => NumericValue::ONE, // x^(-0) = 1
            (NumericValue::PositiveInfinity, NumericValue::NegativeZero) => NumericValue::ONE, // (+∞)^(-0) = 1
            (NumericValue::NegativeInfinity, NumericValue::NegativeZero) => NumericValue::ONE, // (-∞)^(-0) = 1
        }
    }

    pub fn log(self) -> NumericValue {
        // TODO: For high precision, implement using Newton's method or Taylor series:
        // ln(x) can be computed using the series: ln(1+u) = u - u²/2 + u³/3 - u⁴/4 + ...
        // Or use Newton's method: find y such that e^y = x
        // For now, using f64 conversion for compatibility
        match self {
            NumericValue::Rational(_) | NumericValue::BigDecimal(_) => {
                let f = self.to_f64();
                if f <= 0.0 {
                    if f == 0.0 {
                        NumericValue::NegativeInfinity
                    } else {
                        NumericValue::NaN
                    }
                } else {
                    NumericValue::from(f.ln())
                }
            }
            NumericValue::Decimal(d) => {
                if d <= Decimal::ZERO {
                    if d.is_zero() {
                        NumericValue::NegativeInfinity
                    } else {
                        NumericValue::NaN // log of negative number
                    }
                } else {
                    let f = d.to_f64().unwrap_or(0.0);
                    NumericValue::from(f.ln())
                }
            }
            NumericValue::NegativeZero => NumericValue::NegativeInfinity, // log(-0) = -∞
            NumericValue::NaN => NumericValue::NaN,
            NumericValue::PositiveInfinity => NumericValue::PositiveInfinity,
            NumericValue::NegativeInfinity => NumericValue::NaN,
        }
    }

    pub fn log10(self) -> NumericValue {
        // TODO: For high precision, implement as log(x) / log(10) using Decimal
        // For now, using f64 conversion for compatibility
        match self {
            NumericValue::Rational(_) | NumericValue::BigDecimal(_) => {
                let f = self.to_f64();
                if f <= 0.0 {
                    if f == 0.0 {
                        NumericValue::NegativeInfinity
                    } else {
                        NumericValue::NaN
                    }
                } else {
                    NumericValue::from(f.log10())
                }
            }
            NumericValue::Decimal(d) => {
                if d <= Decimal::ZERO {
                    if d.is_zero() {
                        NumericValue::NegativeInfinity
                    } else {
                        NumericValue::NaN // log of negative number
                    }
                } else {
                    let f = d.to_f64().unwrap_or(0.0);
                    NumericValue::from(f.log10())
                }
            }
            NumericValue::NegativeZero => NumericValue::NegativeInfinity, // log10(-0) = -∞
            NumericValue::NaN => NumericValue::NaN,
            NumericValue::PositiveInfinity => NumericValue::PositiveInfinity,
            NumericValue::NegativeInfinity => NumericValue::NaN,
        }
    }

    pub fn log2(self) -> NumericValue {
        // TODO: For high precision, implement as log(x) / log(2) using Decimal
        // For now, using f64 conversion for compatibility
        match self {
            NumericValue::Rational(_) | NumericValue::BigDecimal(_) => {
                let f = self.to_f64();
                if f <= 0.0 {
                    if f == 0.0 {
                        NumericValue::NegativeInfinity
                    } else {
                        NumericValue::NaN
                    }
                } else {
                    NumericValue::from(f.log2())
                }
            }
            NumericValue::Decimal(d) => {
                if d <= Decimal::ZERO {
                    if d.is_zero() {
                        NumericValue::NegativeInfinity
                    } else {
                        NumericValue::NaN // log of negative number
                    }
                } else {
                    let f = d.to_f64().unwrap_or(0.0);
                    NumericValue::from(f.log2())
                }
            }
            NumericValue::NegativeZero => NumericValue::NegativeInfinity, // log2(-0) = -∞
            NumericValue::NaN => NumericValue::NaN,
            NumericValue::PositiveInfinity => NumericValue::PositiveInfinity,
            NumericValue::NegativeInfinity => NumericValue::NaN,
        }
    }

    pub fn exp(self) -> NumericValue {
        // TODO: For high precision, implement using Taylor series with Decimal:
        // e^x = 1 + x + x²/2! + x³/3! + x⁴/4! + ...
        // For now, using f64 conversion for compatibility
        match self {
            NumericValue::Rational(_) | NumericValue::BigDecimal(_) => {
                let f = self.to_f64();
                NumericValue::from(f.exp())
            }
            NumericValue::Decimal(d) => {
                let f = d.to_f64().unwrap_or(0.0);
                let result = f.exp();
                NumericValue::from(result)
            }
            NumericValue::NegativeZero => NumericValue::ONE, // exp(-0) = 1
            NumericValue::NaN => NumericValue::NaN,
            NumericValue::PositiveInfinity => NumericValue::PositiveInfinity,
            NumericValue::NegativeInfinity => NumericValue::ZERO,
        }
    }

    pub fn sin(self) -> NumericValue {
        // TODO: For high precision, implement using Taylor series with Decimal:
        // sin(x) = x - x³/3! + x⁵/5! - x⁷/7! + ...
        // This would require implementing factorial and power series with Decimal precision
        // For now, using f64 conversion for compatibility
        match self {
            NumericValue::Rational(_) | NumericValue::BigDecimal(_) => {
                let f = self.to_f64();
                NumericValue::from(f.sin())
            }
            NumericValue::Decimal(d) => {
                let f = d.to_f64().unwrap_or(0.0);
                NumericValue::from(f.sin())
            }
            NumericValue::NegativeZero => NumericValue::NegativeZero, // sin(-0) = -0
            NumericValue::NaN => NumericValue::NaN,
            NumericValue::PositiveInfinity | NumericValue::NegativeInfinity => NumericValue::NaN,
        }
    }

    pub fn cos(self) -> NumericValue {
        // TODO: For high precision, implement using Taylor series with Decimal:
        // cos(x) = 1 - x²/2! + x⁴/4! - x⁶/6! + ...
        // This would require implementing factorial and power series with Decimal precision
        // For now, using f64 conversion for compatibility
        match self {
            NumericValue::Rational(_) | NumericValue::BigDecimal(_) => {
                let f = self.to_f64();
                NumericValue::from(f.cos())
            }
            NumericValue::Decimal(d) => {
                let f = d.to_f64().unwrap_or(0.0);
                NumericValue::from(f.cos())
            }
            NumericValue::NegativeZero => NumericValue::ONE, // cos(-0) = 1
            NumericValue::NaN => NumericValue::NaN,
            NumericValue::PositiveInfinity | NumericValue::NegativeInfinity => NumericValue::NaN,
        }
    }

    pub fn tan(self) -> NumericValue {
        // TODO: For high precision, implement as sin(x)/cos(x) using Decimal Taylor series
        // Need to handle asymptotes where cos(x) = 0
        // For now, using f64 conversion for compatibility
        match self {
            NumericValue::Rational(_) | NumericValue::BigDecimal(_) => {
                let f = self.to_f64();
                NumericValue::from(f.tan())
            }
            NumericValue::Decimal(d) => {
                let f = d.to_f64().unwrap_or(0.0);
                NumericValue::from(f.tan())
            }
            NumericValue::NegativeZero => NumericValue::NegativeZero, // tan(-0) = -0
            NumericValue::NaN => NumericValue::NaN,
            NumericValue::PositiveInfinity | NumericValue::NegativeInfinity => NumericValue::NaN,
        }
    }

    pub fn asin(self) -> NumericValue {
        // TODO: For high precision, implement using Taylor series with Decimal:
        // asin(x) = x + x³/6 + 3x⁵/40 + ... (for |x| < 1)
        // Or use Newton's method: find y such that sin(y) = x
        // For now, using f64 conversion for compatibility
        match self {
            NumericValue::Rational(_) | NumericValue::BigDecimal(_) => {
                let f = self.to_f64();
                if f.abs() > 1.0 {
                    NumericValue::NaN
                } else {
                    NumericValue::from(f.asin())
                }
            }
            NumericValue::Decimal(d) => {
                let f = d.to_f64().unwrap_or(0.0);
                if f.abs() > 1.0 {
                    NumericValue::NaN
                } else {
                    NumericValue::from(f.asin())
                }
            }
            NumericValue::NegativeZero => NumericValue::NegativeZero, // asin(-0) = -0
            NumericValue::NaN => NumericValue::NaN,
            NumericValue::PositiveInfinity | NumericValue::NegativeInfinity => NumericValue::NaN,
        }
    }

    pub fn acos(self) -> NumericValue {
        // TODO: For high precision, implement using relationship acos(x) = π/2 - asin(x)
        // Or use Newton's method: find y such that cos(y) = x
        // For now, using f64 conversion for compatibility
        match self {
            NumericValue::Rational(_) | NumericValue::BigDecimal(_) => {
                let f = self.to_f64();
                if f.abs() > 1.0 {
                    NumericValue::NaN
                } else {
                    NumericValue::from(f.acos())
                }
            }
            NumericValue::Decimal(d) => {
                let f = d.to_f64().unwrap_or(0.0);
                if f.abs() > 1.0 {
                    NumericValue::NaN
                } else {
                    NumericValue::from(f.acos())
                }
            }
            NumericValue::NegativeZero => NumericValue::from(std::f64::consts::FRAC_PI_2), // acos(-0) = π/2
            NumericValue::NaN => NumericValue::NaN,
            NumericValue::PositiveInfinity | NumericValue::NegativeInfinity => NumericValue::NaN,
        }
    }

    pub fn atan(self) -> NumericValue {
        // TODO: For high precision, implement using Taylor series with Decimal:
        // atan(x) = x - x³/3 + x⁵/5 - x⁷/7 + ... (for |x| < 1)
        // For |x| >= 1, use atan(x) = π/2 - atan(1/x)
        // For now, using f64 conversion for compatibility
        match self {
            NumericValue::Rational(_) | NumericValue::BigDecimal(_) => {
                let f = self.to_f64();
                NumericValue::from(f.atan())
            }
            NumericValue::Decimal(d) => {
                let f = d.to_f64().unwrap_or(0.0);
                NumericValue::from(f.atan())
            }
            NumericValue::NegativeZero => NumericValue::NegativeZero, // atan(-0) = -0
            NumericValue::NaN => NumericValue::NaN,
            NumericValue::PositiveInfinity => NumericValue::from(std::f64::consts::FRAC_PI_2),
            NumericValue::NegativeInfinity => NumericValue::from(-std::f64::consts::FRAC_PI_2),
        }
    }

    pub fn atan2(self, x: NumericValue) -> NumericValue {
        // TODO: For high precision, implement using Decimal arithmetic:
        // atan2(y, x) handles all quadrants and edge cases
        // Would need to implement atan with Decimal and handle all JS edge cases
        // For now, using f64 conversion for compatibility
        match (self, x) {
            (NumericValue::Rational(_), _) | (_, NumericValue::Rational(_)) => {
                unimplemented!("atan2 with Rational not yet implemented")
            }
            (NumericValue::BigDecimal(_), _) | (_, NumericValue::BigDecimal(_)) => {
                unimplemented!("atan2 with BigDecimal not yet implemented")
            }
            (NumericValue::Decimal(y), NumericValue::Decimal(x_val)) => {
                let y_f = y.to_f64().unwrap_or(0.0);
                let x_f = x_val.to_f64().unwrap_or(0.0);
                NumericValue::from(y_f.atan2(x_f))
            }
            (NumericValue::NegativeZero, NumericValue::Decimal(x_val)) => {
                let x_f = x_val.to_f64().unwrap_or(0.0);
                NumericValue::from((-0.0_f64).atan2(x_f))
            }
            (NumericValue::Decimal(y), NumericValue::NegativeZero) => {
                let y_f = y.to_f64().unwrap_or(0.0);
                NumericValue::from(y_f.atan2(-0.0_f64))
            }
            (NumericValue::NegativeZero, NumericValue::NegativeZero) => {
                NumericValue::from((-0.0_f64).atan2(-0.0_f64))
            }
            (NumericValue::NaN, _) | (_, NumericValue::NaN) => NumericValue::NaN,
            // Handle infinity cases according to JS Math.atan2
            (NumericValue::PositiveInfinity, NumericValue::PositiveInfinity) => {
                NumericValue::from(std::f64::consts::FRAC_PI_4)
            }
            (NumericValue::PositiveInfinity, NumericValue::NegativeInfinity) => {
                NumericValue::from(3.0 * std::f64::consts::FRAC_PI_4)
            }
            (NumericValue::NegativeInfinity, NumericValue::PositiveInfinity) => {
                NumericValue::from(-std::f64::consts::FRAC_PI_4)
            }
            (NumericValue::NegativeInfinity, NumericValue::NegativeInfinity) => {
                NumericValue::from(-3.0 * std::f64::consts::FRAC_PI_4)
            }
            (NumericValue::PositiveInfinity, _) => NumericValue::from(std::f64::consts::FRAC_PI_2),
            (NumericValue::NegativeInfinity, _) => NumericValue::from(-std::f64::consts::FRAC_PI_2),
            (_, NumericValue::PositiveInfinity) => NumericValue::from(0.0),
            (NumericValue::Decimal(y), NumericValue::NegativeInfinity) => {
                if y >= Decimal::ZERO {
                    NumericValue::from(std::f64::consts::PI)
                } else {
                    NumericValue::from(-std::f64::consts::PI)
                }
            }
            (NumericValue::NegativeZero, NumericValue::NegativeInfinity) => {
                NumericValue::from(-std::f64::consts::PI)
            }
        }
    }

    pub fn increment(self) -> NumericValue {
        self + NumericValue::ONE
    }

    pub fn decrement(self) -> NumericValue {
        self - NumericValue::ONE
    }

    pub fn to_i32(&self) -> Option<i32> {
        match self {
            NumericValue::Rational(r) => {
                if r.is_integer() {
                    r.to_integer().to_i32()
                } else {
                    None
                }
            }
            NumericValue::Decimal(d) => d.to_i32(),
            NumericValue::BigDecimal(_) => unimplemented!("BigDecimal to_i32 not yet implemented"),
            NumericValue::NegativeZero => Some(0),
            NumericValue::NaN => None,
            NumericValue::PositiveInfinity => None,
            NumericValue::NegativeInfinity => None,
        }
    }

    pub fn to_u32(&self) -> Option<u32> {
        match self {
            NumericValue::Rational(r) => {
                if r.is_integer() {
                    r.to_integer().to_u32()
                } else {
                    None
                }
            }
            NumericValue::Decimal(d) => d.to_u32(),
            NumericValue::BigDecimal(_) => unimplemented!("BigDecimal to_u32 not yet implemented"),
            NumericValue::NegativeZero => Some(0),
            NumericValue::NaN => None,
            NumericValue::PositiveInfinity => None,
            NumericValue::NegativeInfinity => None,
        }
    }

    pub fn to_i64(&self) -> Option<i64> {
        match self {
            NumericValue::Rational(r) => {
                if r.is_integer() {
                    Some(*r.numer())
                } else {
                    None
                }
            }
            NumericValue::Decimal(d) => d.to_i64(),
            NumericValue::BigDecimal(_) => unimplemented!("BigDecimal to_i64 not yet implemented"),
            NumericValue::NegativeZero => Some(0),
            NumericValue::NaN => None,
            NumericValue::PositiveInfinity => None,
            NumericValue::NegativeInfinity => None,
        }
    }

    pub fn to_f64(&self) -> f64 {
        match self {
            NumericValue::Rational(r) => {
                r.numer().to_f64().unwrap_or(0.0) / r.denom().to_f64().unwrap_or(1.0)
            }
            NumericValue::Decimal(d) => d.to_f64().expect("Decimal always fits in f64"),
            NumericValue::BigDecimal(bd) => {
                // BigDecimal to f64 conversion may lose precision
                bd.to_string().parse().unwrap_or(0.0)
            }
            NumericValue::NegativeZero => -0.0,
            NumericValue::NaN => f64::NAN,
            NumericValue::PositiveInfinity => f64::INFINITY,
            NumericValue::NegativeInfinity => f64::NEG_INFINITY,
        }
    }

    pub fn to_decimal(&self) -> Option<Decimal> {
        match self {
            NumericValue::Rational(r) => {
                // Try to convert rational to decimal
                // This may lose precision for repeating decimals
                let f = r.numer().to_f64()? / r.denom().to_f64()?;
                Decimal::from_f64(f)
            }
            NumericValue::Decimal(d) => Some(*d),
            NumericValue::BigDecimal(_) => None, // TODO: implement conversion
            NumericValue::NegativeZero => Some(Decimal::ZERO),
            _ => None,
        }
    }

    // /// Convert to primitive value (used in JS type coercion)
    // pub fn to_primitive(&self) -> NumericValue {
    //     self.clone() // Numbers are already primitive
    // }
}

// Add this implementation block for Number in math.rs
impl Number {
    // Mathematical functions - delegating to NumericValue

    pub fn abs(self) -> Number {
        Number {
            value: self.value.abs(),
            transcendental: self.transcendental,
            rational_approximation: self.rational_approximation,
        }
    }

    pub fn floor(self) -> Number {
        Number {
            value: self.value.floor(),
            transcendental: false, // Rounding removes approximate decimal digits
            rational_approximation: self.rational_approximation,
        }
    }

    pub fn ceil(self) -> Number {
        Number {
            value: self.value.ceil(),
            transcendental: false, // Rounding removes approximate decimal digits
            rational_approximation: self.rational_approximation,
        }
    }

    pub fn round(self) -> Number {
        Number {
            value: self.value.round(),
            transcendental: false, // Rounding removes approximate decimal digits
            rational_approximation: self.rational_approximation,
        }
    }

    pub fn round_dp(self, dp: u32) -> Number {
        Number {
            value: self.value.round_dp(dp),
            transcendental: false, // Rounding removes approximate decimal digits
            rational_approximation: self.rational_approximation,
        }
    }

    pub fn trunc(self) -> Number {
        Number {
            value: self.value.trunc(),
            transcendental: false, // Truncation removes approximate decimal digits
            rational_approximation: self.rational_approximation,
        }
    }

    pub fn sqrt(self) -> Number {
        let result_value = self.value.sqrt();

        // Only transcendental if result is a Decimal (approximation)
        // If result is Rational (like sqrt(4) = 2), it's exact
        let is_transcendental = matches!(result_value, NumericValue::Decimal(_));

        Number {
            value: result_value,
            transcendental: is_transcendental,
            rational_approximation: false,
        }
    }

    pub fn pow(self, exponent: Number) -> Number {
        let approximated =
            self.transcendental || exponent.transcendental || self.is_transcendental_pow(&exponent);
        Number {
            value: self.value.pow(exponent.value),
            transcendental: approximated,
            rational_approximation: false, // transcendental trumps rational_approximation
        }
    }

    // Transcendental functions - always mark as transcendental
    pub fn log(self) -> Number {
        Number {
            value: self.value.log(),
            transcendental: true,
            rational_approximation: false,
        }
    }

    pub fn log10(self) -> Number {
        Number {
            value: self.value.log10(),
            transcendental: true,
            rational_approximation: false,
        }
    }

    pub fn log2(self) -> Number {
        Number {
            value: self.value.log2(),
            transcendental: true,
            rational_approximation: false,
        }
    }

    pub fn exp(self) -> Number {
        Number {
            value: self.value.exp(),
            transcendental: true,
            rational_approximation: false,
        }
    }

    pub fn sin(self) -> Number {
        Number {
            value: self.value.sin(),
            transcendental: true,
            rational_approximation: false,
        }
    }

    pub fn cos(self) -> Number {
        Number {
            value: self.value.cos(),
            transcendental: true,
            rational_approximation: false,
        }
    }

    pub fn tan(self) -> Number {
        Number {
            value: self.value.tan(),
            transcendental: true,
            rational_approximation: false,
        }
    }

    pub fn asin(self) -> Number {
        Number {
            value: self.value.asin(),
            transcendental: true,
            rational_approximation: false,
        }
    }

    pub fn acos(self) -> Number {
        Number {
            value: self.value.acos(),
            transcendental: true,
            rational_approximation: false,
        }
    }

    pub fn atan(self) -> Number {
        Number {
            value: self.value.atan(),
            transcendental: true,
            rational_approximation: false,
        }
    }

    pub fn atan2(self, x: Number) -> Number {
        Number {
            value: self.value.atan2(x.value),
            transcendental: true,
            rational_approximation: false,
        }
    }

    pub fn increment(self) -> Number {
        Number {
            value: self.value.increment(),
            transcendental: self.transcendental,
            rational_approximation: self.rational_approximation,
        }
    }

    pub fn decrement(self) -> Number {
        Number {
            value: self.value.decrement(),
            transcendental: self.transcendental,
            rational_approximation: self.rational_approximation,
        }
    }

    pub fn to_primitive(&self) -> Number {
        self.clone() // Numbers are already primitive
    }

    // Helper to determine if a pow operation is transcendental
    fn is_transcendental_pow(&self, exponent: &Number) -> bool {
        // Integer powers are exact, fractional powers are approximated
        match &exponent.value {
            NumericValue::Decimal(d) => !d.fract().is_zero(),
            NumericValue::Rational(r) => !r.is_integer(),
            _ => false,
        }
    }
}
