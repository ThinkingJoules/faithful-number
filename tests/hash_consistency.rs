//! Tests for hash consistency across representations.
//!
//! The key invariant: if a == b, then hash(a) == hash(b)

use faithful_number::{Number, OrderedNumber};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn hash_of<T: Hash>(t: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    t.hash(&mut hasher);
    hasher.finish()
}

#[test]
fn zero_variants_hash_equal() {
    let zero_int = Number::from(0);
    let neg_zero = Number::neg_zero();

    // They are equal
    assert_eq!(zero_int, neg_zero);

    // They must hash the same (via OrderedNumber)
    let h1 = hash_of(&OrderedNumber::from(zero_int));
    let h2 = hash_of(&OrderedNumber::from(neg_zero));
    assert_eq!(h1, h2);
}

#[test]
fn nan_values_hash_equal() {
    let nan1 = Number::nan();
    let nan2 = Number::nan();

    // Via OrderedNumber, NaN == NaN
    let on1 = OrderedNumber::from(nan1);
    let on2 = OrderedNumber::from(nan2);

    assert_eq!(on1, on2);
    assert_eq!(hash_of(&on1), hash_of(&on2));
}

#[test]
fn same_value_different_creation() {
    // Same value created different ways
    let a = Number::from(42);
    let b = Number::from(42i64);
    let c = Number::from(42u32);

    assert_eq!(a, b);
    assert_eq!(b, c);

    assert_eq!(
        hash_of(&OrderedNumber::from(a.clone())),
        hash_of(&OrderedNumber::from(b.clone()))
    );
    assert_eq!(
        hash_of(&OrderedNumber::from(b)),
        hash_of(&OrderedNumber::from(c))
    );
}

#[test]
fn infinity_hashes() {
    let inf1 = Number::infinity();
    let inf2 = Number::infinity();

    assert_eq!(inf1, inf2);
    assert_eq!(
        hash_of(&OrderedNumber::from(inf1)),
        hash_of(&OrderedNumber::from(inf2))
    );
}

#[test]
fn neg_infinity_hashes() {
    let inf1 = Number::neg_infinity();
    let inf2 = Number::neg_infinity();

    assert_eq!(inf1, inf2);
    assert_eq!(
        hash_of(&OrderedNumber::from(inf1)),
        hash_of(&OrderedNumber::from(inf2))
    );
}

#[test]
fn different_values_different_hashes() {
    // While not guaranteed, different values should usually hash differently
    let a = OrderedNumber::from(Number::from(1));
    let b = OrderedNumber::from(Number::from(2));
    let c = OrderedNumber::from(Number::from(3));

    // These should be different (with very high probability)
    assert_ne!(hash_of(&a), hash_of(&b));
    assert_ne!(hash_of(&b), hash_of(&c));
}

#[test]
fn hashmap_with_ordered_number() {
    use std::collections::HashMap;

    let mut map: HashMap<OrderedNumber, i32> = HashMap::new();

    map.insert(OrderedNumber::from(Number::from(1)), 100);
    map.insert(OrderedNumber::from(Number::from(2)), 200);
    map.insert(OrderedNumber::from(Number::nan()), 300);

    assert_eq!(map.get(&OrderedNumber::from(Number::from(1))), Some(&100));
    assert_eq!(map.get(&OrderedNumber::from(Number::from(2))), Some(&200));
    assert_eq!(map.get(&OrderedNumber::from(Number::nan())), Some(&300));
}

#[test]
fn hashset_deduplication() {
    use std::collections::HashSet;

    let mut set: HashSet<OrderedNumber> = HashSet::new();

    set.insert(OrderedNumber::from(Number::from(1)));
    set.insert(OrderedNumber::from(Number::from(1))); // duplicate
    set.insert(OrderedNumber::from(Number::nan()));
    set.insert(OrderedNumber::from(Number::nan())); // duplicate

    assert_eq!(set.len(), 2); // Only 2 unique values
}

#[test]
fn zero_decimal_and_rational_hash_same() {
    use rust_decimal::Decimal;

    let zero_rational = Number::from(0);
    let zero_decimal = Number::from_decimal(Decimal::ZERO);

    // They are equal
    assert_eq!(zero_rational, zero_decimal);

    // They must hash the same
    assert_eq!(
        hash_of(&OrderedNumber::from(zero_rational)),
        hash_of(&OrderedNumber::from(zero_decimal))
    );
}
