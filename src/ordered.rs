//! OrderedNumber - A wrapper for Number that implements Eq and Hash.
//!
//! Since IEEE 754 requires NaN != NaN, Number cannot implement Eq by default.
//! OrderedNumber wraps Number and provides total ordering where NaN == NaN,
//! enabling use in HashMap, HashSet, and other collections.

use crate::Number;
use crate::core::NumericValue;
use num_traits::Zero;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

/// A wrapper around Number that provides Eq and Hash implementations.
///
/// Use this when you need to store Numbers in HashMap, HashSet, or other
/// collections that require Eq.
///
/// # Example
///
/// ```
/// use faithful_number::{Number, OrderedNumber};
/// use std::collections::HashMap;
///
/// let mut map: HashMap<OrderedNumber, i32> = HashMap::new();
/// map.insert(OrderedNumber::from(Number::from(1)), 100);
/// map.insert(OrderedNumber::from(Number::nan()), 200);
///
/// assert_eq!(map.get(&OrderedNumber::from(Number::from(1))), Some(&100));
/// assert_eq!(map.get(&OrderedNumber::from(Number::nan())), Some(&200));
/// ```
#[derive(Debug, Clone)]
pub struct OrderedNumber(pub Number);

impl OrderedNumber {
    /// Create a new OrderedNumber from a Number.
    pub fn new(n: Number) -> Self {
        OrderedNumber(n)
    }

    /// Get a reference to the inner Number.
    pub fn inner(&self) -> &Number {
        &self.0
    }

    /// Consume the wrapper and return the inner Number.
    pub fn into_inner(self) -> Number {
        self.0
    }
}

impl From<Number> for OrderedNumber {
    fn from(n: Number) -> Self {
        OrderedNumber(n)
    }
}

impl From<OrderedNumber> for Number {
    fn from(on: OrderedNumber) -> Self {
        on.0
    }
}

impl PartialEq for OrderedNumber {
    fn eq(&self, other: &Self) -> bool {
        match (self.0.value(), other.0.value()) {
            // NaN == NaN for collection use
            (NumericValue::NaN, NumericValue::NaN) => true,
            // Delegate to Number's PartialEq for everything else
            _ => self.0 == other.0,
        }
    }
}

impl Eq for OrderedNumber {}

impl PartialOrd for OrderedNumber {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for OrderedNumber {
    fn cmp(&self, other: &Self) -> Ordering {
        // Total ordering: NaN < -Infinity < finite numbers < +Infinity
        match (self.0.value(), other.0.value()) {
            (NumericValue::NaN, NumericValue::NaN) => Ordering::Equal,
            (NumericValue::NaN, _) => Ordering::Less,
            (_, NumericValue::NaN) => Ordering::Greater,
            // Use partial_cmp and default to Equal for same values
            _ => self.0.partial_cmp(&other.0).unwrap_or(Ordering::Equal),
        }
    }
}

impl Hash for OrderedNumber {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Hash must be consistent with Eq: equal values must have equal hashes
        match self.0.value() {
            NumericValue::NaN => {
                0u8.hash(state); // discriminant
                // All NaN values hash the same
            }
            NumericValue::PositiveInfinity => {
                1u8.hash(state);
            }
            NumericValue::NegativeInfinity => {
                2u8.hash(state);
            }
            NumericValue::NegativeZero => {
                // -0 == +0, so they must hash the same
                3u8.hash(state);
                0i64.hash(state);
            }
            NumericValue::Rational(r, _) => {
                // For consistent hashing, normalize the representation
                // Convert to canonical form for hashing
                if r.is_zero() {
                    3u8.hash(state);
                    0i64.hash(state);
                } else {
                    3u8.hash(state);
                    // Use numerator and denominator directly (already reduced)
                    r.numer().hash(state);
                    r.denom().hash(state);
                }
            }
            NumericValue::Decimal(d) => {
                // Check if it equals zero (same hash as Rational zero and -0)
                if d.is_zero() {
                    3u8.hash(state);
                    0i64.hash(state);
                } else {
                    // For decimals, normalize and hash
                    3u8.hash(state);
                    d.normalize().hash(state);
                }
            }
            NumericValue::BigDecimal(bd) => {
                if bd.is_zero() {
                    3u8.hash(state);
                    0i64.hash(state);
                } else {
                    // BigDecimal doesn't have a great hash, use string repr
                    3u8.hash(state);
                    bd.to_string().hash(state);
                }
            }
        }
    }
}

// Convenience: allow arithmetic on OrderedNumber
impl std::ops::Deref for OrderedNumber {
    type Target = Number;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::Decimal;
    use std::collections::{HashMap, HashSet};

    #[test]
    fn nan_equals_nan() {
        let nan1 = OrderedNumber::from(Number::nan());
        let nan2 = OrderedNumber::from(Number::nan());
        assert_eq!(nan1, nan2);
    }

    #[test]
    fn can_use_in_hashmap() {
        let mut map: HashMap<OrderedNumber, i32> = HashMap::new();
        map.insert(OrderedNumber::from(Number::from(1)), 100);
        map.insert(OrderedNumber::from(Number::nan()), 200);

        assert_eq!(map.get(&OrderedNumber::from(Number::from(1))), Some(&100));
        assert_eq!(map.get(&OrderedNumber::from(Number::nan())), Some(&200));
    }

    #[test]
    fn can_use_in_hashset() {
        let mut set: HashSet<OrderedNumber> = HashSet::new();
        set.insert(OrderedNumber::from(Number::from(1)));
        set.insert(OrderedNumber::from(Number::nan()));
        set.insert(OrderedNumber::from(Number::nan())); // duplicate

        assert_eq!(set.len(), 2);
    }

    #[test]
    fn hash_consistency_zero_variants() {
        use std::hash::DefaultHasher;

        fn hash_of(on: &OrderedNumber) -> u64 {
            let mut hasher = DefaultHasher::new();
            on.hash(&mut hasher);
            hasher.finish()
        }

        let zero_int = OrderedNumber::from(Number::from(0));
        let neg_zero = OrderedNumber::from(Number::neg_zero());
        let zero_dec = OrderedNumber::from(Number::from_decimal(Decimal::ZERO));

        // All zeros should be equal
        assert_eq!(zero_int, neg_zero);
        assert_eq!(zero_int, zero_dec);

        // All zeros should hash the same
        assert_eq!(hash_of(&zero_int), hash_of(&neg_zero));
        assert_eq!(hash_of(&zero_int), hash_of(&zero_dec));
    }
}
