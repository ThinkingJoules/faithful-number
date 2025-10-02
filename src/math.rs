use crate::{Number, NumericValue};
use num_rational::Ratio;
use rust_decimal::Decimal;

use num_traits::{FromPrimitive, Signed, ToPrimitive, Zero};
use std::str::FromStr;

#[cfg(feature = "high_precision")]
use bigdecimal::BigDecimal;
#[cfg(feature = "high_precision")]
use rug::Float;
#[cfg(feature = "high_precision")]
use rug::ops::Pow;

/// Helper function to convert NumericValue to rug::Float for high-precision operations
#[cfg(feature = "high_precision")]
fn to_rug_float(value: &NumericValue, precision: u32) -> Option<Float> {
    match value {
        NumericValue::Rational(r) => {
            let numer = *r.numer();
            let denom = *r.denom();
            Some(Float::with_val(precision, numer) / Float::with_val(precision, denom))
        }
        NumericValue::Decimal(d) => {
            // Convert Decimal to string, then to Float
            let s = d.to_string();
            Float::parse(&s).ok().map(|f| Float::with_val(precision, f))
        }
        NumericValue::BigDecimal(bd) => {
            // Convert BigDecimal to string, then to Float
            let s = bd.to_string();
            Float::parse(&s).ok().map(|f| Float::with_val(precision, f))
        }
        NumericValue::NegativeZero => Some(Float::with_val(precision, 0)),
        _ => None,
    }
}

/// Helper function to convert rug::Float to BigDecimal
#[cfg(feature = "high_precision")]
fn rug_float_to_bigdecimal(f: &Float) -> BigDecimal {
    use std::str::FromStr;
    // Convert to string representation and parse as BigDecimal
    let s = f.to_string();
    BigDecimal::from_str(&s).unwrap_or_else(|_| BigDecimal::from(0))
}

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
                    (is_perfect_square(numer), is_perfect_square(denom))
                {
                    return NumericValue::Rational(Ratio::new(numer_sqrt, denom_sqrt));
                }

                // Not a perfect square - convert to Decimal for approximation
                let decimal = Decimal::from(numer) / Decimal::from(denom);
                NumericValue::Decimal(decimal).sqrt()
            }
            NumericValue::Decimal(d) => {
                if d < Decimal::ZERO {
                    return NumericValue::NaN; // sqrt of negative number is NaN in JS
                } else if d.is_zero() {
                    return NumericValue::Decimal(Decimal::ZERO); // sqrt(0) = 0
                }

                #[cfg(feature = "high_precision")]
                {
                    // Use high-precision rug::Float for sqrt
                    let precision = crate::precision::get_default_precision();
                    if let Some(f) = to_rug_float(&NumericValue::Decimal(d), precision) {
                        let result = f.sqrt();
                        return NumericValue::BigDecimal(rug_float_to_bigdecimal(&result));
                    }
                }

                // Fallback: Babylonian method (Newton-Raphson) for square root
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
            NumericValue::BigDecimal(bd) => {
                if bd.is_zero() {
                    NumericValue::BigDecimal(bigdecimal::BigDecimal::from(0))
                } else if bd < bigdecimal::BigDecimal::from(0) {
                    NumericValue::NaN
                } else {
                    // Use BigDecimal's built-in sqrt with default precision
                    NumericValue::BigDecimal(bd.sqrt().unwrap_or(bd))
                }
            }
            NumericValue::NegativeZero => NumericValue::ZERO, // sqrt(-0) = +0 in JS
            NumericValue::NaN => NumericValue::NaN,
            NumericValue::PositiveInfinity => NumericValue::PositiveInfinity,
            NumericValue::NegativeInfinity => NumericValue::NaN, // sqrt(-Infinity) = NaN
        }
    }

    pub fn pow(self, exponent: NumericValue) -> NumericValue {
        match (self, exponent) {
            // Rational base: handle sqrt specially, otherwise convert to Decimal
            (NumericValue::Rational(base), exp) => {
                // Check if exponent is 0.5 (sqrt case)
                if let NumericValue::Rational(exp_r) = &exp {
                    if *exp_r.numer() == 1 && *exp_r.denom() == 2 {
                        // Use Rational sqrt which preserves exactness for perfect squares
                        return NumericValue::Rational(base).sqrt();
                    }
                } else if let NumericValue::Decimal(exp_d) = &exp {
                    if *exp_d == Decimal::from_str("0.5").unwrap_or(Decimal::ZERO) {
                        return NumericValue::Rational(base).sqrt();
                    }
                }
                // General case: convert to Decimal
                let base_decimal = Decimal::from(*base.numer()) / Decimal::from(*base.denom());
                NumericValue::Decimal(base_decimal).pow(exp)
            }
            // BigDecimal base: use high-precision or convert to f64
            (NumericValue::BigDecimal(base), exp) => {
                #[cfg(feature = "high_precision")]
                {
                    let precision = crate::precision::get_default_precision();
                    if let (Some(base_f), Some(exp_f)) = (to_rug_float(&NumericValue::BigDecimal(base.clone()), precision), to_rug_float(&exp, precision)) {
                        let result = base_f.pow(exp_f);
                        return NumericValue::BigDecimal(rug_float_to_bigdecimal(&result));
                    }
                }
                // Fallback to f64
                let base_f64 = base.to_f64().unwrap_or(0.0);
                let exp_f64 = exp.to_f64();
                NumericValue::from(base_f64.powf(exp_f64))
            }
            // Rational exponent: convert to Decimal and use Decimal pow
            (base, NumericValue::Rational(exp)) => {
                let exp_decimal = Decimal::from(*exp.numer()) / Decimal::from(*exp.denom());
                base.pow(NumericValue::Decimal(exp_decimal))
            }
            // BigDecimal exponent: use high-precision or convert to f64
            (base, NumericValue::BigDecimal(exp)) => {
                #[cfg(feature = "high_precision")]
                {
                    let precision = crate::precision::get_default_precision();
                    if let (Some(base_f), Some(exp_f)) = (to_rug_float(&base, precision), to_rug_float(&NumericValue::BigDecimal(exp.clone()), precision)) {
                        let result = base_f.pow(exp_f);
                        return NumericValue::BigDecimal(rug_float_to_bigdecimal(&result));
                    }
                }
                // Fallback to f64
                let base_f64 = base.to_f64();
                let exp_f64 = exp.to_f64().unwrap_or(0.0);
                NumericValue::from(base_f64.powf(exp_f64))
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

                        // Fast exponentiation by squaring with overflow checking
                        while current_exp > 0 {
                            if current_exp % 2 == 1 {
                                match result.checked_mul(current_base) {
                                    Some(r) => result = r,
                                    None => {
                                        // Overflow - graduate to BigDecimal and continue
                                        use bigdecimal::BigDecimal;
                                        use crate::ops::arithmetic::decimal_to_bigdecimal;
                                        let mut result_bd = decimal_to_bigdecimal(result);
                                        let mut base_bd = decimal_to_bigdecimal(current_base);
                                        result_bd = result_bd * base_bd.clone();
                                        current_exp /= 2;

                                        // Continue with BigDecimal arithmetic
                                        while current_exp > 0 {
                                            if current_exp % 2 == 1 {
                                                result_bd = result_bd * base_bd.clone();
                                            }
                                            base_bd = base_bd.clone() * base_bd.clone();
                                            current_exp /= 2;
                                        }
                                        return NumericValue::BigDecimal(result_bd);
                                    }
                                }
                            }
                            match current_base.checked_mul(current_base) {
                                Some(b) => current_base = b,
                                None => {
                                    // Overflow on base squaring - graduate to BigDecimal
                                    use bigdecimal::BigDecimal;
                                    use crate::ops::arithmetic::decimal_to_bigdecimal;
                                    let mut result_bd = decimal_to_bigdecimal(result);
                                    let mut base_bd = decimal_to_bigdecimal(current_base);
                                    base_bd = base_bd.clone() * base_bd.clone();
                                    current_exp /= 2;

                                    // Continue with BigDecimal arithmetic
                                    while current_exp > 0 {
                                        if current_exp % 2 == 1 {
                                            result_bd = result_bd * base_bd.clone();
                                        }
                                        base_bd = base_bd.clone() * base_bd.clone();
                                        current_exp /= 2;
                                    }
                                    return NumericValue::BigDecimal(result_bd);
                                }
                            }
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
                    // Check both 0.5 and 1/2 representations
                    let half = Decimal::from_str("0.5").unwrap_or(Decimal::ZERO);
                    if exp == half {
                        return NumericValue::Decimal(base).sqrt();
                    }

                    // Fractional exponent - use a^b = e^(b * ln(a))
                    // When high_precision is enabled, log and exp will use rug::Float automatically
                    #[cfg(feature = "high_precision")]
                    {
                        let precision = crate::precision::get_default_precision();
                        if let Some(base_f) = to_rug_float(&NumericValue::Decimal(base), precision) {
                            if let Some(exp_f) = to_rug_float(&NumericValue::Decimal(exp), precision) {
                                let result = base_f.pow(exp_f);
                                return NumericValue::BigDecimal(rug_float_to_bigdecimal(&result));
                            }
                        }
                    }

                    // Fallback to f64
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
        // Special value handling first
        match &self {
            NumericValue::NegativeZero => return NumericValue::NegativeInfinity,
            NumericValue::NaN => return NumericValue::NaN,
            NumericValue::PositiveInfinity => return NumericValue::PositiveInfinity,
            NumericValue::NegativeInfinity => return NumericValue::NaN,
            _ => {}
        }

        #[cfg(feature = "high_precision")]
        {
            // Use high-precision rug::Float
            let precision = crate::precision::get_default_precision();
            if let Some(f) = to_rug_float(&self, precision) {
                if f.is_sign_negative() {
                    return NumericValue::NaN;
                }
                if f.is_zero() {
                    return NumericValue::NegativeInfinity;
                }
                let result = f.ln();
                return NumericValue::BigDecimal(rug_float_to_bigdecimal(&result));
            }
        }

        // Fallback to f64 (when high_precision is disabled or conversion failed)
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
                        NumericValue::NaN
                    }
                } else {
                    let f = d.to_f64().unwrap_or(0.0);
                    NumericValue::from(f.ln())
                }
            }
            _ => unreachable!(),
        }
    }

    pub fn log10(self) -> NumericValue {
        // Special value handling first
        match &self {
            NumericValue::NegativeZero => return NumericValue::NegativeInfinity,
            NumericValue::NaN => return NumericValue::NaN,
            NumericValue::PositiveInfinity => return NumericValue::PositiveInfinity,
            NumericValue::NegativeInfinity => return NumericValue::NaN,
            _ => {}
        }

        #[cfg(feature = "high_precision")]
        {
            // Use high-precision rug::Float
            let precision = crate::precision::get_default_precision();
            if let Some(f) = to_rug_float(&self, precision) {
                if f.is_sign_negative() {
                    return NumericValue::NaN;
                }
                if f.is_zero() {
                    return NumericValue::NegativeInfinity;
                }
                let result = f.log10();
                return NumericValue::BigDecimal(rug_float_to_bigdecimal(&result));
            }
        }

        // Fallback to f64
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
                        NumericValue::NaN
                    }
                } else {
                    let f = d.to_f64().unwrap_or(0.0);
                    NumericValue::from(f.log10())
                }
            }
            _ => unreachable!(),
        }
    }

    pub fn log2(self) -> NumericValue {
        // Special value handling first
        match &self {
            NumericValue::NegativeZero => return NumericValue::NegativeInfinity,
            NumericValue::NaN => return NumericValue::NaN,
            NumericValue::PositiveInfinity => return NumericValue::PositiveInfinity,
            NumericValue::NegativeInfinity => return NumericValue::NaN,
            _ => {}
        }

        #[cfg(feature = "high_precision")]
        {
            // Use high-precision rug::Float
            let precision = crate::precision::get_default_precision();
            if let Some(f) = to_rug_float(&self, precision) {
                if f.is_sign_negative() {
                    return NumericValue::NaN;
                }
                if f.is_zero() {
                    return NumericValue::NegativeInfinity;
                }
                let result = f.log2();
                return NumericValue::BigDecimal(rug_float_to_bigdecimal(&result));
            }
        }

        // Fallback to f64
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
                        NumericValue::NaN
                    }
                } else {
                    let f = d.to_f64().unwrap_or(0.0);
                    NumericValue::from(f.log2())
                }
            }
            _ => unreachable!(),
        }
    }

    pub fn exp(self) -> NumericValue {
        // Special value handling first
        match &self {
            NumericValue::NegativeZero => return NumericValue::ONE,
            NumericValue::NaN => return NumericValue::NaN,
            NumericValue::PositiveInfinity => return NumericValue::PositiveInfinity,
            NumericValue::NegativeInfinity => return NumericValue::ZERO,
            _ => {}
        }

        #[cfg(feature = "high_precision")]
        {
            // Use high-precision rug::Float
            let precision = crate::precision::get_default_precision();
            if let Some(f) = to_rug_float(&self, precision) {
                let result = f.exp();
                return NumericValue::BigDecimal(rug_float_to_bigdecimal(&result));
            }
        }

        // Fallback to f64
        match self {
            NumericValue::Rational(_) | NumericValue::BigDecimal(_) => {
                let f = self.to_f64();
                NumericValue::from(f.exp())
            }
            NumericValue::Decimal(d) => {
                let f = d.to_f64().unwrap_or(0.0);
                NumericValue::from(f.exp())
            }
            _ => unreachable!(),
        }
    }

    pub fn sin(self) -> NumericValue {
        // Special value handling first
        match &self {
            NumericValue::NegativeZero => return NumericValue::NegativeZero,
            NumericValue::NaN => return NumericValue::NaN,
            NumericValue::PositiveInfinity | NumericValue::NegativeInfinity => return NumericValue::NaN,
            _ => {}
        }

        #[cfg(feature = "high_precision")]
        {
            // Use high-precision rug::Float
            let precision = crate::precision::get_default_precision();
            if let Some(f) = to_rug_float(&self, precision) {
                let result = f.sin();
                return NumericValue::BigDecimal(rug_float_to_bigdecimal(&result));
            }
        }

        // Fallback to f64
        match self {
            NumericValue::Rational(_) | NumericValue::BigDecimal(_) => {
                let f = self.to_f64();
                NumericValue::from(f.sin())
            }
            NumericValue::Decimal(d) => {
                let f = d.to_f64().unwrap_or(0.0);
                NumericValue::from(f.sin())
            }
            _ => unreachable!(),
        }
    }

    pub fn cos(self) -> NumericValue {
        // Special value handling first
        match &self {
            NumericValue::NegativeZero => return NumericValue::ONE,
            NumericValue::NaN => return NumericValue::NaN,
            NumericValue::PositiveInfinity | NumericValue::NegativeInfinity => return NumericValue::NaN,
            _ => {}
        }

        #[cfg(feature = "high_precision")]
        {
            // Use high-precision rug::Float
            let precision = crate::precision::get_default_precision();
            if let Some(f) = to_rug_float(&self, precision) {
                let result = f.cos();
                return NumericValue::BigDecimal(rug_float_to_bigdecimal(&result));
            }
        }

        // Fallback to f64
        match self {
            NumericValue::Rational(_) | NumericValue::BigDecimal(_) => {
                let f = self.to_f64();
                NumericValue::from(f.cos())
            }
            NumericValue::Decimal(d) => {
                let f = d.to_f64().unwrap_or(0.0);
                NumericValue::from(f.cos())
            }
            _ => unreachable!(),
        }
    }

    pub fn tan(self) -> NumericValue {
        // Special value handling first
        match &self {
            NumericValue::NegativeZero => return NumericValue::NegativeZero,
            NumericValue::NaN => return NumericValue::NaN,
            NumericValue::PositiveInfinity | NumericValue::NegativeInfinity => return NumericValue::NaN,
            _ => {}
        }

        #[cfg(feature = "high_precision")]
        {
            // Use high-precision rug::Float
            let precision = crate::precision::get_default_precision();
            if let Some(f) = to_rug_float(&self, precision) {
                let result = f.tan();
                return NumericValue::BigDecimal(rug_float_to_bigdecimal(&result));
            }
        }

        // Fallback to f64
        match self {
            NumericValue::Rational(_) | NumericValue::BigDecimal(_) => {
                let f = self.to_f64();
                NumericValue::from(f.tan())
            }
            NumericValue::Decimal(d) => {
                let f = d.to_f64().unwrap_or(0.0);
                NumericValue::from(f.tan())
            }
            _ => unreachable!(),
        }
    }

    pub fn asin(self) -> NumericValue {
        // Special value handling first
        match &self {
            NumericValue::NegativeZero => return NumericValue::NegativeZero,
            NumericValue::NaN => return NumericValue::NaN,
            NumericValue::PositiveInfinity | NumericValue::NegativeInfinity => return NumericValue::NaN,
            _ => {}
        }

        #[cfg(feature = "high_precision")]
        {
            // Use high-precision rug::Float
            let precision = crate::precision::get_default_precision();
            if let Some(f) = to_rug_float(&self, precision) {
                if f.clone().abs() > 1.0 {
                    return NumericValue::NaN;
                }
                let result = f.asin();
                return NumericValue::BigDecimal(rug_float_to_bigdecimal(&result));
            }
        }

        // Fallback to f64
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
            _ => unreachable!(),
        }
    }

    pub fn acos(self) -> NumericValue {
        // Special value handling first
        match &self {
            NumericValue::NegativeZero => return NumericValue::from(std::f64::consts::FRAC_PI_2),
            NumericValue::NaN => return NumericValue::NaN,
            NumericValue::PositiveInfinity | NumericValue::NegativeInfinity => return NumericValue::NaN,
            _ => {}
        }

        #[cfg(feature = "high_precision")]
        {
            // Use high-precision rug::Float
            let precision = crate::precision::get_default_precision();
            if let Some(f) = to_rug_float(&self, precision) {
                if f.clone().abs() > 1.0 {
                    return NumericValue::NaN;
                }
                let result = f.acos();
                return NumericValue::BigDecimal(rug_float_to_bigdecimal(&result));
            }
        }

        // Fallback to f64
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
            _ => unreachable!(),
        }
    }

    pub fn atan(self) -> NumericValue {
        // Special value handling first
        match &self {
            NumericValue::NegativeZero => return NumericValue::NegativeZero,
            NumericValue::NaN => return NumericValue::NaN,
            NumericValue::PositiveInfinity => return NumericValue::from(std::f64::consts::FRAC_PI_2),
            NumericValue::NegativeInfinity => return NumericValue::from(-std::f64::consts::FRAC_PI_2),
            _ => {}
        }

        #[cfg(feature = "high_precision")]
        {
            // Use high-precision rug::Float
            let precision = crate::precision::get_default_precision();
            if let Some(f) = to_rug_float(&self, precision) {
                let result = f.atan();
                return NumericValue::BigDecimal(rug_float_to_bigdecimal(&result));
            }
        }

        // Fallback to f64
        match self {
            NumericValue::Rational(_) | NumericValue::BigDecimal(_) => {
                let f = self.to_f64();
                NumericValue::from(f.atan())
            }
            NumericValue::Decimal(d) => {
                let f = d.to_f64().unwrap_or(0.0);
                NumericValue::from(f.atan())
            }
            _ => unreachable!(),
        }
    }

    pub fn atan2(self, x: NumericValue) -> NumericValue {
        // Handle NaN cases
        match (&self, &x) {
            (NumericValue::NaN, _) | (_, NumericValue::NaN) => return NumericValue::NaN,
            _ => {}
        }

        #[cfg(feature = "high_precision")]
        {
            // Try high-precision path
            let precision = crate::precision::get_default_precision();
            if let (Some(y_f), Some(x_f)) = (to_rug_float(&self, precision), to_rug_float(&x, precision)) {
                let result = y_f.atan2(&x_f);
                return NumericValue::BigDecimal(rug_float_to_bigdecimal(&result));
            }
        }

        // Fallback to f64 for all cases (special handling for -0.0)
        let y_f64 = match &self {
            NumericValue::NegativeZero => -0.0_f64,
            _ => self.to_f64(),
        };
        let x_f64 = match &x {
            NumericValue::NegativeZero => -0.0_f64,
            _ => x.to_f64(),
        };

        match (self, x) {
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
            // Default case: use pre-computed f64 values
            _ => NumericValue::from(y_f64.atan2(x_f64))
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
            apprx: self.apprx,
        }
    }

    pub fn floor(self) -> Number {
        Number {
            value: self.value.floor(),
            // Rounding removes approximate decimal digits - result is exact
            apprx: None,
        }
    }

    pub fn ceil(self) -> Number {
        Number {
            value: self.value.ceil(),
            // Rounding removes approximate decimal digits - result is exact
            apprx: None,
        }
    }

    pub fn round(self) -> Number {
        Number {
            value: self.value.round(),
            // Rounding removes approximate decimal digits - result is exact
            apprx: None,
        }
    }

    pub fn round_dp(self, dp: u32) -> Number {
        Number {
            value: self.value.round_dp(dp),
            // Rounding removes approximate decimal digits - result is exact
            apprx: None,
        }
    }

    pub fn trunc(self) -> Number {
        Number {
            value: self.value.trunc(),
            // Truncation removes approximate decimal digits - result is exact
            apprx: None,
        }
    }

    pub fn sqrt(self) -> Number {
        use crate::ApproximationType;
        let result_value = self.value.sqrt();

        // Transcendental if result is Decimal or BigDecimal (approximation)
        // If result is Rational (like sqrt(4) = 2), it's exact
        let apprx = if matches!(result_value, NumericValue::Decimal(_) | NumericValue::BigDecimal(_)) {
            Some(ApproximationType::Transcendental)
        } else {
            None
        };

        Number {
            value: result_value,
            apprx,
        }
    }

    pub fn pow(self, exponent: Number) -> Number {
        use crate::ApproximationType;
        let is_approximated = self.is_transcendental()
            || exponent.is_transcendental()
            || self.is_transcendental_pow(&exponent);

        Number {
            value: self.value.pow(exponent.value),
            apprx: if is_approximated {
                Some(ApproximationType::Transcendental)
            } else {
                None
            },
        }
    }

    // Transcendental functions - mark as transcendental only if result is approximated
    pub fn log(self) -> Number {
        use crate::ApproximationType;
        let result_value = self.value.log();
        Number {
            value: result_value.clone(),
            apprx: if matches!(result_value, NumericValue::Decimal(_) | NumericValue::BigDecimal(_)) {
                Some(ApproximationType::Transcendental)
            } else {
                None
            },
        }
    }

    pub fn log10(self) -> Number {
        use crate::ApproximationType;
        let result_value = self.value.log10();
        Number {
            value: result_value.clone(),
            apprx: if matches!(result_value, NumericValue::Decimal(_) | NumericValue::BigDecimal(_)) {
                Some(ApproximationType::Transcendental)
            } else {
                None
            },
        }
    }

    pub fn log2(self) -> Number {
        use crate::ApproximationType;
        let result_value = self.value.log2();
        Number {
            value: result_value.clone(),
            apprx: if matches!(result_value, NumericValue::Decimal(_) | NumericValue::BigDecimal(_)) {
                Some(ApproximationType::Transcendental)
            } else {
                None
            },
        }
    }

    pub fn exp(self) -> Number {
        use crate::ApproximationType;
        let result_value = self.value.exp();
        Number {
            value: result_value.clone(),
            apprx: if matches!(result_value, NumericValue::Decimal(_) | NumericValue::BigDecimal(_)) {
                Some(ApproximationType::Transcendental)
            } else {
                None
            },
        }
    }

    pub fn sin(self) -> Number {
        use crate::ApproximationType;
        let result_value = self.value.sin();
        Number {
            value: result_value.clone(),
            apprx: if matches!(result_value, NumericValue::Decimal(_) | NumericValue::BigDecimal(_)) {
                Some(ApproximationType::Transcendental)
            } else {
                None
            },
        }
    }

    pub fn cos(self) -> Number {
        use crate::ApproximationType;
        let result_value = self.value.cos();
        Number {
            value: result_value.clone(),
            apprx: if matches!(result_value, NumericValue::Decimal(_) | NumericValue::BigDecimal(_)) {
                Some(ApproximationType::Transcendental)
            } else {
                None
            },
        }
    }

    pub fn tan(self) -> Number {
        use crate::ApproximationType;
        let result_value = self.value.tan();
        Number {
            value: result_value.clone(),
            apprx: if matches!(result_value, NumericValue::Decimal(_) | NumericValue::BigDecimal(_)) {
                Some(ApproximationType::Transcendental)
            } else {
                None
            },
        }
    }

    pub fn asin(self) -> Number {
        use crate::ApproximationType;
        let result_value = self.value.asin();
        Number {
            value: result_value.clone(),
            apprx: if matches!(result_value, NumericValue::Decimal(_) | NumericValue::BigDecimal(_)) {
                Some(ApproximationType::Transcendental)
            } else {
                None
            },
        }
    }

    pub fn acos(self) -> Number {
        use crate::ApproximationType;
        let result_value = self.value.acos();
        Number {
            value: result_value.clone(),
            apprx: if matches!(result_value, NumericValue::Decimal(_) | NumericValue::BigDecimal(_)) {
                Some(ApproximationType::Transcendental)
            } else {
                None
            },
        }
    }

    pub fn atan(self) -> Number {
        use crate::ApproximationType;
        let result_value = self.value.atan();
        Number {
            value: result_value.clone(),
            apprx: if matches!(result_value, NumericValue::Decimal(_) | NumericValue::BigDecimal(_)) {
                Some(ApproximationType::Transcendental)
            } else {
                None
            },
        }
    }

    pub fn atan2(self, x: Number) -> Number {
        use crate::ApproximationType;
        let result_value = self.value.atan2(x.value);
        Number {
            value: result_value.clone(),
            apprx: if matches!(result_value, NumericValue::Decimal(_) | NumericValue::BigDecimal(_)) {
                Some(ApproximationType::Transcendental)
            } else {
                None
            },
        }
    }

    pub fn increment(self) -> Number {
        Number {
            value: self.value.increment(),
            apprx: self.apprx,
        }
    }

    pub fn decrement(self) -> Number {
        Number {
            value: self.value.decrement(),
            apprx: self.apprx,
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
