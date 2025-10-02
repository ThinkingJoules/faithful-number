use std::str::FromStr;

use num_traits::{FromPrimitive, ToPrimitive};
use rust_decimal::Decimal;

use crate::{Number, NumericValue};

// Parse from string with JS semantics
impl FromStr for NumericValue {
    type Err = ();

    fn from_str(s: &str) -> Result<NumericValue, Self::Err> {
        let s = s.trim();

        // Handle special JavaScript string values
        match s {
            "NaN" => Ok(NumericValue::NaN),
            "Infinity" => Ok(NumericValue::PositiveInfinity),
            "-Infinity" => Ok(NumericValue::NegativeInfinity),
            "-0" => Ok(NumericValue::NegativeZero),
            "" => Ok(NumericValue::ZERO), // Empty string converts to 0 in JS
            _ => {
                // Try to parse as Decimal first
                if let Ok(d) = Decimal::from_str(s) {
                    Ok(NumericValue::Decimal(d))
                } else {
                    // Try to parse as f64 for cases Decimal can't handle
                    if let Ok(f) = f64::from_str(s) {
                        Ok(NumericValue::from(f))
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

// Generate From implementations for all primitive number types
impl_from_primitives_inner!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);

// Special From implementations that need custom logic
impl From<Decimal> for NumericValue {
    fn from(d: Decimal) -> NumericValue {
        NumericValue::Decimal(d)
    }
}

impl From<f64> for NumericValue {
    fn from(f: f64) -> NumericValue {
        if f.is_nan() {
            NumericValue::NaN
        } else if f.is_infinite() {
            if f.is_sign_positive() {
                NumericValue::PositiveInfinity
            } else {
                NumericValue::NegativeInfinity
            }
        } else if f == 0.0 {
            if f.is_sign_negative() {
                NumericValue::NegativeZero
            } else {
                NumericValue::ZERO
            }
        } else {
            // Convert f64 to Decimal - this might lose precision for very large numbers
            if let Some(d) = Decimal::from_f64(f) {
                NumericValue::Decimal(d)
            } else {
                // If conversion fails, fall back to NaN
                NumericValue::NaN
            }
        }
    }
}

impl From<f32> for NumericValue {
    fn from(f: f32) -> NumericValue {
        NumericValue::from(f as f64)
    }
}

impl TryFrom<NumericValue> for i32 {
    type Error = ();

    fn try_from(js_num: NumericValue) -> Result<i32, Self::Error> {
        match js_num {
            NumericValue::Decimal(d) => d.to_i32().ok_or(()),
            NumericValue::Rational(r, _) => {
                if r.is_integer() {
                    r.to_integer().to_i32().ok_or(())
                } else {
                    Err(())
                }
            }
            NumericValue::BigDecimal(_) => {
                unimplemented!("BigDecimal to i32 conversion not yet implemented")
            }
            _ => Err(()), // Can't convert NaN or Infinity
        }
    }
}

impl TryFrom<NumericValue> for u32 {
    type Error = ();

    fn try_from(js_num: NumericValue) -> Result<u32, Self::Error> {
        match js_num {
            NumericValue::Decimal(d) => d.to_u32().ok_or(()),
            NumericValue::Rational(r, _) => {
                if r.is_integer() {
                    r.to_integer().to_u32().ok_or(())
                } else {
                    Err(())
                }
            }
            NumericValue::BigDecimal(_) => {
                unimplemented!("BigDecimal to u32 conversion not yet implemented")
            }
            _ => Err(()),
        }
    }
}

impl TryFrom<NumericValue> for i64 {
    type Error = ();

    fn try_from(js_num: NumericValue) -> Result<i64, Self::Error> {
        match js_num {
            NumericValue::Decimal(d) => d.to_i64().ok_or(()),
            NumericValue::Rational(r, _) => {
                if r.is_integer() {
                    Some(*r.numer()).ok_or(())
                } else {
                    Err(())
                }
            }
            NumericValue::BigDecimal(_) => {
                unimplemented!("BigDecimal to i64 conversion not yet implemented")
            }
            _ => Err(()),
        }
    }
}

// Special case for f64 which can represent all our values
impl TryFrom<NumericValue> for f64 {
    type Error = ();

    fn try_from(js_num: NumericValue) -> Result<f64, Self::Error> {
        Ok(js_num.to_f64()) // Never fails
    }
}

// Special case for Decimal
impl TryFrom<NumericValue> for Decimal {
    type Error = ();

    fn try_from(js_num: NumericValue) -> Result<Decimal, Self::Error> {
        js_num.to_decimal().ok_or(())
    }
}

// Parse from string with JS semantics
impl FromStr for Number {
    type Err = ();

    fn from_str(s: &str) -> Result<Number, Self::Err> {
        let s = s.trim();

        // Handle special JavaScript string values
        let value = match s {
            "NaN" => NumericValue::NaN,
            "Infinity" => NumericValue::PositiveInfinity,
            "-Infinity" => NumericValue::NegativeInfinity,
            "-0" => NumericValue::NegativeZero,
            "" => NumericValue::ZERO, // Empty string converts to 0 in JS
            _ => {
                // Try to parse as Decimal first
                if let Ok(d) = Decimal::from_str(s) {
                    NumericValue::Decimal(d)
                } else {
                    // Try to parse as BigDecimal for very large numbers
                    use bigdecimal::BigDecimal;
                    if let Ok(bd) = s.parse::<BigDecimal>() {
                        NumericValue::BigDecimal(bd)
                    } else {
                        // Try to parse as f64 for cases neither can handle
                        if let Ok(f) = f64::from_str(s) {
                            return Ok(Number::from(f));
                        } else {
                            return Err(());
                        }
                    }
                }
            }
        };

        Ok(Number { value, apprx: None })
    }
}

// Implement From for primitives - all integers start as Rational
impl From<i8> for Number {
    fn from(n: i8) -> Number {
        use num_rational::Ratio;
        Number::from_rational(Ratio::from_integer(n as i64))
    }
}

impl From<i16> for Number {
    fn from(n: i16) -> Number {
        use num_rational::Ratio;
        Number::from_rational(Ratio::from_integer(n as i64))
    }
}

impl From<i32> for Number {
    fn from(n: i32) -> Number {
        use num_rational::Ratio;
        Number::from_rational(Ratio::from_integer(n as i64))
    }
}

impl From<i64> for Number {
    fn from(n: i64) -> Number {
        use num_rational::Ratio;
        Number::from_rational(Ratio::from_integer(n))
    }
}

impl From<isize> for Number {
    fn from(n: isize) -> Number {
        use num_rational::Ratio;
        Number::from_rational(Ratio::from_integer(n as i64))
    }
}

impl From<u8> for Number {
    fn from(n: u8) -> Number {
        use num_rational::Ratio;
        Number::from_rational(Ratio::from_integer(n as i64))
    }
}

impl From<u16> for Number {
    fn from(n: u16) -> Number {
        use num_rational::Ratio;
        Number::from_rational(Ratio::from_integer(n as i64))
    }
}

impl From<u32> for Number {
    fn from(n: u32) -> Number {
        use num_rational::Ratio;
        Number::from_rational(Ratio::from_integer(n as i64))
    }
}

impl From<u64> for Number {
    fn from(n: u64) -> Number {
        // u64 might not fit in i64, check first
        if let Ok(n_i64) = i64::try_from(n) {
            use num_rational::Ratio;
            Number::from_rational(Ratio::from_integer(n_i64))
        } else {
            // Fallback to Decimal for large u64
            Number::from_decimal(Decimal::from(n))
        }
    }
}

impl From<usize> for Number {
    fn from(n: usize) -> Number {
        // usize might not fit in i64, check first
        if let Ok(n_i64) = i64::try_from(n) {
            use num_rational::Ratio;
            Number::from_rational(Ratio::from_integer(n_i64))
        } else {
            // Fallback to Decimal for large usize
            Number::from_decimal(Decimal::from(n as u64))
        }
    }
}

impl From<Decimal> for Number {
    fn from(d: Decimal) -> Number {
        Number::from_decimal(d)
    }
}

impl From<f64> for Number {
    fn from(f: f64) -> Number {
        let value = if f.is_nan() {
            NumericValue::NaN
        } else if f.is_infinite() {
            if f.is_sign_positive() {
                NumericValue::PositiveInfinity
            } else {
                NumericValue::NegativeInfinity
            }
        } else if f == 0.0 {
            if f.is_sign_negative() {
                NumericValue::NegativeZero
            } else {
                NumericValue::ZERO
            }
        } else {
            // Try to extract rational representation from f64
            // Many f64 values can be exactly represented as rationals
            use num_rational::Ratio;

            // Extract mantissa and exponent
            let bits = f.to_bits();
            let sign = if bits >> 63 == 0 { 1i64 } else { -1i64 };
            let exponent = ((bits >> 52) & 0x7ff) as i32 - 1023;
            let mantissa = if exponent == -1023 {
                (bits & 0xfffffffffffff) << 1
            } else {
                (bits & 0xfffffffffffff) | 0x10000000000000
            };

            // Try to represent as rational
            if exponent >= 0 {
                // Positive exponent: mantissa * 2^exponent / 2^52
                let numerator = mantissa as i128 * sign as i128;
                let shift = exponent - 52;
                if shift >= 0 {
                    // multiply numerator by 2^shift
                    if let Some(shifted) = numerator.checked_shl(shift as u32) {
                        if let Ok(num_i64) = i64::try_from(shifted) {
                            return Number::from_rational(Ratio::from_integer(num_i64));
                        }
                    }
                } else {
                    // numerator / 2^(-shift)
                    let denom = 1i64 << (-shift);
                    if let Ok(num_i64) = i64::try_from(numerator) {
                        return Number::from_rational(Ratio::new(num_i64, denom));
                    }
                }
            } else {
                // Negative exponent: mantissa / 2^(52 - exponent)
                let numerator = mantissa as i128 * sign as i128;
                let denom_exp = 52 - exponent;
                if denom_exp <= 63 {
                    let denom = 1i64 << denom_exp;
                    if let Ok(num_i64) = i64::try_from(numerator) {
                        return Number::from_rational(Ratio::new(num_i64, denom));
                    }
                }
            }

            // Fallback: Convert f64 to Decimal
            if let Some(d) = Decimal::from_f64(f) {
                NumericValue::Decimal(d)
            } else {
                // If conversion fails, fall back to NaN
                NumericValue::NaN
            }
        };

        Number { value, apprx: None }
    }
}

impl From<f32> for Number {
    fn from(f: f32) -> Number {
        Number::from(f as f64)
    }
}

// TryFrom implementations to extract primitives
impl TryFrom<Number> for i32 {
    type Error = ();

    fn try_from(num: Number) -> Result<i32, Self::Error> {
        match num.value {
            NumericValue::Decimal(d) => d.to_i32().ok_or(()),
            NumericValue::Rational(r, _) => {
                if r.is_integer() {
                    r.to_integer().to_i32().ok_or(())
                } else {
                    Err(())
                }
            }
            NumericValue::BigDecimal(_) => {
                unimplemented!("BigDecimal to i32 conversion not yet implemented")
            }
            _ => Err(()), // Can't convert NaN or Infinity
        }
    }
}

impl TryFrom<Number> for u32 {
    type Error = ();

    fn try_from(num: Number) -> Result<u32, Self::Error> {
        match num.value {
            NumericValue::Decimal(d) => d.to_u32().ok_or(()),
            NumericValue::Rational(r, _) => {
                if r.is_integer() {
                    r.to_integer().to_u32().ok_or(())
                } else {
                    Err(())
                }
            }
            NumericValue::BigDecimal(_) => {
                unimplemented!("BigDecimal to u32 conversion not yet implemented")
            }
            _ => Err(()),
        }
    }
}

impl TryFrom<Number> for i64 {
    type Error = ();

    fn try_from(num: Number) -> Result<i64, Self::Error> {
        match num.value {
            NumericValue::Decimal(d) => d.to_i64().ok_or(()),
            NumericValue::Rational(r, _) => {
                if r.is_integer() {
                    Some(*r.numer()).ok_or(())
                } else {
                    Err(())
                }
            }
            NumericValue::BigDecimal(_) => {
                unimplemented!("BigDecimal to i64 conversion not yet implemented")
            }
            _ => Err(()),
        }
    }
}

// Special case for f64 which can represent all our values
impl TryFrom<Number> for f64 {
    type Error = ();

    fn try_from(num: Number) -> Result<f64, Self::Error> {
        Ok(num.to_f64()) // Never fails
    }
}

// Special case for Decimal
impl TryFrom<Number> for Decimal {
    type Error = ();

    fn try_from(num: Number) -> Result<Decimal, Self::Error> {
        num.to_decimal().ok_or(())
    }
}
