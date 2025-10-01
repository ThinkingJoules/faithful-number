use crate::Number;
use std::cmp::{Ordering, PartialOrd};

use num_traits::ToPrimitive;

impl Number {
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
    // JS-specific operations that don't have Rust traits
    pub fn unsigned_right_shift(self, bits: Number) -> Number {
        // JavaScript's >>> operator: unsigned 32-bit right shift
        let a = self.to_u32_js_coerce(); // Convert to unsigned 32-bit
        let b = bits.to_u32_js_coerce() & 0x1f; // Mask to 5 bits like other shifts
        Number::from(a >> b)
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
