use std::str::FromStr;

use num_traits::{FromPrimitive, ToPrimitive, Zero};
use rust_decimal::Decimal;

use crate::Number;

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
