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
            NumericValue::Rational(r) => {
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
            NumericValue::Rational(r) => {
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
            NumericValue::Rational(r) => {
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
                    // Try to parse as f64 for cases Decimal can't handle
                    if let Ok(f) = f64::from_str(s) {
                        return Ok(Number::from(f));
                    } else {
                        return Err(());
                    }
                }
            }
        };

        Ok(Number {
            value,
            approximated: false,
        })
    }
}

// Implement From for primitives
impl From<i8> for Number {
    fn from(n: i8) -> Number {
        Number::from_decimal(Decimal::from(n))
    }
}

impl From<i16> for Number {
    fn from(n: i16) -> Number {
        Number::from_decimal(Decimal::from(n))
    }
}

impl From<i32> for Number {
    fn from(n: i32) -> Number {
        Number::from_decimal(Decimal::from(n))
    }
}

impl From<i64> for Number {
    fn from(n: i64) -> Number {
        Number::from_decimal(Decimal::from(n))
    }
}

impl From<isize> for Number {
    fn from(n: isize) -> Number {
        Number::from_decimal(Decimal::from(n))
    }
}

impl From<u8> for Number {
    fn from(n: u8) -> Number {
        Number::from_decimal(Decimal::from(n))
    }
}

impl From<u16> for Number {
    fn from(n: u16) -> Number {
        Number::from_decimal(Decimal::from(n))
    }
}

impl From<u32> for Number {
    fn from(n: u32) -> Number {
        Number::from_decimal(Decimal::from(n))
    }
}

impl From<u64> for Number {
    fn from(n: u64) -> Number {
        Number::from_decimal(Decimal::from(n))
    }
}

impl From<usize> for Number {
    fn from(n: usize) -> Number {
        Number::from_decimal(Decimal::from(n))
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
            // Convert f64 to Decimal - this might lose precision for very large numbers
            if let Some(d) = Decimal::from_f64(f) {
                NumericValue::Decimal(d)
            } else {
                // If conversion fails, fall back to NaN
                NumericValue::NaN
            }
        };

        Number {
            value,
            approximated: false,
        }
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
            NumericValue::Rational(r) => {
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
            NumericValue::Rational(r) => {
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
            NumericValue::Rational(r) => {
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
