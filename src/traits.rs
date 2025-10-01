use rust_decimal::Decimal;
use std::cmp::{Ordering, PartialEq, PartialOrd};
use std::fmt::{Display, Formatter, Result as FmtResult};

use std::hash::{Hash, Hasher};

use crate::Number;
use crate::core::NumericValue;

// num_traits for mathematical operations
use num_traits::{FromPrimitive, Num, One, Signed, ToPrimitive, Zero};

// num_traits implementations for mathematical operations
impl Zero for Number {
    fn zero() -> Self {
        Number::ZERO
    }

    fn is_zero(&self) -> bool {
        match &self.value {
            NumericValue::Rational(r) => r.is_zero(),
            NumericValue::Decimal(d) => d.is_zero(),
            NumericValue::BigDecimal(bd) => bd.is_zero(),
            NumericValue::NegativeZero => true,
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
        match &self.value {
            NumericValue::Rational(_r) => unimplemented!("Rational signum not yet implemented"),
            NumericValue::Decimal(d) => {
                if d.is_zero() {
                    Number::zero()
                } else if *d > Decimal::ZERO {
                    Number::one()
                } else {
                    -Number::one()
                }
            }
            NumericValue::BigDecimal(_) => unimplemented!("BigDecimal signum not yet implemented"),
            NumericValue::NegativeZero => Number::neg_zero(), // signum(-0) = -0
            NumericValue::NaN => Number::nan(),
            NumericValue::PositiveInfinity => Number::one(),
            NumericValue::NegativeInfinity => -Number::one(),
        }
    }

    fn is_positive(&self) -> bool {
        match &self.value {
            NumericValue::Rational(r) => r.is_positive(),
            NumericValue::Decimal(d) => d.is_sign_positive(),
            NumericValue::BigDecimal(bd) => bd.is_positive(),
            NumericValue::NegativeZero => false, // -0 is not positive
            NumericValue::PositiveInfinity => true,
            _ => false,
        }
    }

    fn is_negative(&self) -> bool {
        match &self.value {
            NumericValue::Rational(r) => r.is_negative(),
            NumericValue::Decimal(d) => d.is_sign_negative(),
            NumericValue::BigDecimal(bd) => bd.is_negative(),
            NumericValue::NegativeZero => true, // -0 is negative
            NumericValue::NegativeInfinity => true,
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
        match &self.value {
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
            _ => None,
        }
    }

    fn to_u64(&self) -> Option<u64> {
        match &self.value {
            NumericValue::Rational(r) => {
                if r.is_integer() && r.is_positive() {
                    r.numer().to_u64()
                } else {
                    None
                }
            }
            NumericValue::Decimal(d) => d.to_u64(),
            NumericValue::BigDecimal(_) => unimplemented!("BigDecimal to_u64 not yet implemented"),
            NumericValue::NegativeZero => Some(0),
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
        match &self.value {
            NumericValue::Rational(r) => write!(f, "{}/{}", r.numer(), r.denom()),
            NumericValue::Decimal(d) => write!(f, "{}", d),
            NumericValue::BigDecimal(bd) => write!(f, "{}", bd),
            NumericValue::NegativeZero => write!(f, "0"), // -0 displays as "0"
            NumericValue::NaN => write!(f, "NaN"),
            NumericValue::PositiveInfinity => write!(f, "Infinity"),
            NumericValue::NegativeInfinity => write!(f, "-Infinity"),
        }
    }
}

impl Hash for Number {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match &self.value {
            NumericValue::Rational(_r) => unimplemented!("Rational hash not yet implemented"),
            NumericValue::Decimal(d) => {
                0u8.hash(state); // Discriminant
                d.hash(state);
            }
            NumericValue::BigDecimal(_bd) => unimplemented!("BigDecimal hash not yet implemented"),
            NumericValue::NaN => {
                1u8.hash(state); // All NaN values hash the same
            }
            NumericValue::PositiveInfinity => {
                2u8.hash(state);
            }
            NumericValue::NegativeInfinity => {
                3u8.hash(state);
            }
            NumericValue::NegativeZero => {
                4u8.hash(state);
            }
        }
    }
}

impl PartialEq for Number {
    fn eq(&self, other: &Number) -> bool {
        match (self.value(), other.value()) {
            // For Rust compatibility, NaN equals itself (breaking JS semantics)
            (NumericValue::NaN, NumericValue::NaN) => true,
            (NumericValue::NaN, _) | (_, NumericValue::NaN) => false,
            (NumericValue::Rational(a), NumericValue::Rational(b)) => a == b,
            (NumericValue::Decimal(a), NumericValue::Decimal(b)) => a == b,
            (NumericValue::BigDecimal(a), NumericValue::BigDecimal(b)) => a == b,
            (NumericValue::PositiveInfinity, NumericValue::PositiveInfinity) => true,
            (NumericValue::NegativeInfinity, NumericValue::NegativeInfinity) => true,
            (NumericValue::NegativeZero, NumericValue::NegativeZero) => true,
            // +0 equals -0 (maintaining this JS semantic for simplicity)
            (NumericValue::Decimal(a), NumericValue::NegativeZero)
            | (NumericValue::NegativeZero, NumericValue::Decimal(a)) => a.is_zero(),
            // Mixed-type comparisons - not yet implemented
            _ => unimplemented!("Mixed-type equality not yet implemented"),
        }
    }
}

impl PartialOrd for Number {
    fn partial_cmp(&self, other: &Number) -> Option<Ordering> {
        match (self.value(), other.value()) {
            // Rational and BigDecimal - not yet implemented (must come first to catch all combinations)
            (NumericValue::Rational(_), _) | (_, NumericValue::Rational(_)) => {
                unimplemented!("Rational partial_cmp not yet implemented")
            }
            (NumericValue::BigDecimal(_), _) | (_, NumericValue::BigDecimal(_)) => {
                unimplemented!("BigDecimal partial_cmp not yet implemented")
            }

            // NaN comparisons now return Some when both are NaN
            (NumericValue::NaN, NumericValue::NaN) => Some(Ordering::Equal),
            (NumericValue::NaN, _) => Some(Ordering::Less),
            (_, NumericValue::NaN) => Some(Ordering::Greater),

            (NumericValue::Decimal(a), NumericValue::Decimal(b)) => a.partial_cmp(b),
            (NumericValue::NegativeInfinity, NumericValue::NegativeInfinity)
            | (NumericValue::PositiveInfinity, NumericValue::PositiveInfinity)
            | (NumericValue::NegativeZero, NumericValue::NegativeZero) => Some(Ordering::Equal),

            // Handle zero equality: +0 == -0
            (NumericValue::Decimal(a), NumericValue::NegativeZero) => {
                if a.is_zero() {
                    Some(Ordering::Equal)
                } else {
                    a.partial_cmp(&Decimal::ZERO)
                }
            }
            (NumericValue::NegativeZero, NumericValue::Decimal(a)) => {
                if a.is_zero() {
                    Some(Ordering::Equal)
                } else {
                    Decimal::ZERO.partial_cmp(a)
                }
            }

            // Infinities
            (NumericValue::NegativeInfinity, _) => Some(Ordering::Less),
            (_, NumericValue::NegativeInfinity) => Some(Ordering::Greater),
            (NumericValue::PositiveInfinity, _) => Some(Ordering::Greater),
            (_, NumericValue::PositiveInfinity) => Some(Ordering::Less),
        }
    }
}

impl Eq for Number {}

// Total ordering: NaN < -Infinity < finite numbers < +Infinity
// Note: -0 and +0 are treated as equal in this ordering
impl Ord for Number {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.value(), other.value()) {
            // Rational and BigDecimal - not yet implemented
            (NumericValue::Rational(_), _) | (_, NumericValue::Rational(_)) => {
                unimplemented!("Rational cmp not yet implemented")
            }
            (NumericValue::BigDecimal(_), _) | (_, NumericValue::BigDecimal(_)) => {
                unimplemented!("BigDecimal cmp not yet implemented")
            }

            // NaN handling - consistent with PartialEq
            (NumericValue::NaN, NumericValue::NaN) => Ordering::Equal,
            (NumericValue::NaN, _) => Ordering::Less,
            (_, NumericValue::NaN) => Ordering::Greater,

            // Infinities
            (NumericValue::NegativeInfinity, NumericValue::NegativeInfinity) => Ordering::Equal,
            (NumericValue::PositiveInfinity, NumericValue::PositiveInfinity) => Ordering::Equal,
            (NumericValue::NegativeInfinity, _) => Ordering::Less,
            (_, NumericValue::NegativeInfinity) => Ordering::Greater,
            (NumericValue::PositiveInfinity, _) => Ordering::Greater,
            (_, NumericValue::PositiveInfinity) => Ordering::Less,

            // Finite numbers and zeros
            (NumericValue::Decimal(a), NumericValue::Decimal(b)) => a.cmp(b),
            (NumericValue::NegativeZero, NumericValue::NegativeZero) => Ordering::Equal,

            // Zero equality: treat +0 and -0 as equal
            (NumericValue::Decimal(a), NumericValue::NegativeZero) => {
                if a.is_zero() {
                    Ordering::Equal
                } else {
                    a.cmp(&Decimal::ZERO)
                }
            }
            (NumericValue::NegativeZero, NumericValue::Decimal(a)) => {
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
