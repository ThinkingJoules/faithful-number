use crate::{Number, NumericValue};
use num_traits::{ToPrimitive, Zero};
use std::cmp::{Ordering, PartialOrd};

impl Number {
    // Type conversions following JS semantics
    /// This is primarily for following semantics during bit-wise operations
    pub fn to_i32_js_coerce(&self) -> i32 {
        self.value.to_i32_js_coerce()
    }

    /// This is used for conversion following semantics during bit-wise operations
    pub fn to_i64_js_coerce(&self) -> i64 {
        self.value.to_i64_js_coerce()
    }

    pub fn to_u32_js_coerce(&self) -> u32 {
        self.value.to_u32_js_coerce()
    }

    // JS-specific operations that don't have Rust traits
    pub fn unsigned_right_shift(self, bits: Number) -> Number {
        Number {
            value: self.value.unsigned_right_shift(bits.value),
            transcendental: self.transcendental || bits.transcendental,
            rational_approximation: self.rational_approximation || bits.rational_approximation,
        }
    }

    // JS semantic operations
    pub fn is_truthy(&self) -> bool {
        self.value.is_truthy()
    }

    pub fn is_falsy(&self) -> bool {
        !self.is_truthy()
    }

    /// Convert to string following JavaScript's ToString algorithm
    pub fn to_js_string(&self) -> String {
        self.value.to_js_string()
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

// Keep the NumericValue implementations for internal use
impl NumericValue {
    pub(crate) fn to_i32_js_coerce(&self) -> i32 {
        match self {
            NumericValue::Rational(r) => {
                // Convert rational to integer (truncate)
                r.to_integer()
                    .to_i32()
                    .unwrap_or_else(|| (r.to_integer() as i64) as i32)
            }
            NumericValue::Decimal(d) => {
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
            NumericValue::BigDecimal(_) => {
                unimplemented!("BigDecimal to_i32_js_coerce not yet implemented")
            }
            NumericValue::NegativeZero => 0,
            NumericValue::NaN => 0,
            NumericValue::PositiveInfinity | NumericValue::NegativeInfinity => 0,
        }
    }

    pub(crate) fn to_i64_js_coerce(&self) -> i64 {
        match self {
            NumericValue::Rational(r) => {
                // Convert rational to integer (truncate)
                // to_integer() returns Ratio with denom=1, numer is the integer value
                r.to_integer().to_i64().unwrap_or(0)
            }
            NumericValue::Decimal(d) => {
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
            NumericValue::BigDecimal(_) => {
                unimplemented!("BigDecimal to_i64_js_coerce not yet implemented")
            }
            NumericValue::NegativeZero => 0,
            NumericValue::NaN => 0,
            NumericValue::PositiveInfinity | NumericValue::NegativeInfinity => 0,
        }
    }

    pub(crate) fn to_u32_js_coerce(&self) -> u32 {
        match self {
            NumericValue::Rational(_)
            | NumericValue::Decimal(_)
            | NumericValue::BigDecimal(_)
            | NumericValue::NegativeZero => {
                // JavaScript ToUint32: convert to i32 first, then reinterpret as u32
                // This handles the wrapping behavior correctly
                let i32_val = self.to_i32_js_coerce();
                i32_val as u32
            }
            NumericValue::NaN => 0,
            NumericValue::PositiveInfinity | NumericValue::NegativeInfinity => 0,
        }
    }

    pub(crate) fn unsigned_right_shift(self, bits: NumericValue) -> NumericValue {
        // JavaScript's >>> operator: unsigned 32-bit right shift
        let a = self.to_u32_js_coerce(); // Convert to unsigned 32-bit
        let b = bits.to_u32_js_coerce() & 0x1f; // Mask to 5 bits like other shifts
        NumericValue::from(a >> b)
    }

    pub(crate) fn is_truthy(&self) -> bool {
        match self {
            NumericValue::Rational(r) => !r.is_zero(), // 0 is falsy, everything else is truthy
            NumericValue::Decimal(d) => !d.is_zero(),  // 0 is falsy, everything else is truthy
            NumericValue::BigDecimal(bd) => !bd.is_zero(), // 0 is falsy, everything else is truthy
            NumericValue::NegativeZero => false,       // -0 is falsy
            NumericValue::NaN => false,                // NaN is falsy
            NumericValue::PositiveInfinity | NumericValue::NegativeInfinity => true, // ±∞ are truthy
        }
    }

    /// Convert to string following JavaScript's ToString algorithm
    pub(crate) fn to_js_string(&self) -> String {
        match self {
            NumericValue::Rational(r) => {
                // Display rational as decimal (convert to Decimal to maintain precision)
                if r.is_integer() {
                    r.to_integer().to_string()
                } else {
                    use rust_decimal::Decimal;
                    let decimal = Decimal::from(*r.numer()) / Decimal::from(*r.denom());
                    decimal.normalize().to_string()
                }
            }
            NumericValue::Decimal(d) => {
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
            NumericValue::BigDecimal(bd) => bd.to_string(),
            NumericValue::NegativeZero => "0".to_string(), // -0 displays as "0" in JS
            NumericValue::NaN => "NaN".to_string(),
            NumericValue::PositiveInfinity => "Infinity".to_string(),
            NumericValue::NegativeInfinity => "-Infinity".to_string(),
        }
    }

    // // Comparison helpers for JS semantics
    // pub(crate) fn js_equals(&self, other: &NumericValue) -> bool {
    //     // This is JavaScript's == comparison (after type coercion)
    //     // For numbers, it's the same as strict equality
    //     self == other
    // }

    // pub(crate) fn js_strict_equals(&self, other: &NumericValue) -> bool {
    //     // This is JavaScript's === comparison
    //     self == other
    // }

    // /// JavaScript's abstract comparison algorithm
    // pub(crate) fn js_less_than(&self, other: &NumericValue) -> Option<bool> {
    //     // In JavaScript, if either operand is NaN, comparison returns undefined (None)
    //     match self.partial_cmp(other) {
    //         Some(Ordering::Less) => Some(true),
    //         Some(Ordering::Greater) | Some(Ordering::Equal) => Some(false),
    //         None => None, // NaN comparisons
    //     }
    // }
}
