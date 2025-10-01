use rust_decimal::Decimal;
use std::cmp::{Ordering, PartialEq, PartialOrd};
use std::fmt::{Display, Formatter, Result as FmtResult};

use std::hash::{Hash, Hasher};

use crate::Number;

// num_traits for mathematical operations
use num_traits::{FromPrimitive, Num, One, Signed, ToPrimitive, Zero};

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
