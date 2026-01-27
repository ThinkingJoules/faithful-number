//! Tests for OrderedNumber wrapper type.

use faithful_number::{Number, OrderedNumber};
use std::collections::{BTreeSet, HashMap, HashSet};

#[test]
fn ordered_number_equality() {
    let a = OrderedNumber::from(Number::from(42));
    let b = OrderedNumber::from(Number::from(42));

    assert_eq!(a, b);
}

#[test]
fn ordered_number_nan_equality() {
    // Key property: NaN == NaN in OrderedNumber
    let nan1 = OrderedNumber::from(Number::nan());
    let nan2 = OrderedNumber::from(Number::nan());

    assert_eq!(nan1, nan2);
}

#[test]
fn ordered_number_ordering() {
    let a = OrderedNumber::from(Number::from(1));
    let b = OrderedNumber::from(Number::from(2));

    assert!(a < b);
    assert!(b > a);
}

#[test]
fn ordered_number_nan_ordering() {
    // NaN should be ordered (less than everything else)
    let nan = OrderedNumber::from(Number::nan());
    let one = OrderedNumber::from(Number::from(1));
    let neg_inf = OrderedNumber::from(Number::neg_infinity());

    assert!(nan < neg_inf);
    assert!(nan < one);
}

#[test]
fn ordered_number_in_hashmap() {
    let mut map: HashMap<OrderedNumber, &str> = HashMap::new();

    map.insert(OrderedNumber::from(Number::from(1)), "one");
    map.insert(OrderedNumber::from(Number::from(2)), "two");
    map.insert(OrderedNumber::from(Number::nan()), "nan");
    map.insert(OrderedNumber::from(Number::infinity()), "inf");

    assert_eq!(map.get(&OrderedNumber::from(Number::from(1))), Some(&"one"));
    assert_eq!(map.get(&OrderedNumber::from(Number::nan())), Some(&"nan"));
}

#[test]
fn ordered_number_in_hashset() {
    let mut set: HashSet<OrderedNumber> = HashSet::new();

    set.insert(OrderedNumber::from(Number::from(1)));
    set.insert(OrderedNumber::from(Number::from(2)));
    set.insert(OrderedNumber::from(Number::nan()));

    assert!(set.contains(&OrderedNumber::from(Number::from(1))));
    assert!(set.contains(&OrderedNumber::from(Number::nan())));
    assert!(!set.contains(&OrderedNumber::from(Number::from(3))));
}

#[test]
fn ordered_number_in_btreeset() {
    // BTreeSet requires Ord
    let mut set: BTreeSet<OrderedNumber> = BTreeSet::new();

    set.insert(OrderedNumber::from(Number::from(3)));
    set.insert(OrderedNumber::from(Number::from(1)));
    set.insert(OrderedNumber::from(Number::from(2)));
    set.insert(OrderedNumber::from(Number::nan()));

    // Should be sorted: NaN < 1 < 2 < 3
    let sorted: Vec<_> = set.iter().collect();
    assert!(sorted[0].inner().is_nan());
    assert_eq!(*sorted[1].inner(), Number::from(1));
    assert_eq!(*sorted[2].inner(), Number::from(2));
    assert_eq!(*sorted[3].inner(), Number::from(3));
}

#[test]
fn ordered_number_deref() {
    let on = OrderedNumber::from(Number::from(42));

    // Can use Number methods via Deref
    assert_eq!(on.to_f64(), 42.0);
    assert!(on.is_exact());
}

#[test]
fn ordered_number_into_inner() {
    let on = OrderedNumber::from(Number::from(42));
    let n: Number = on.into_inner();

    assert_eq!(n, Number::from(42));
}

#[test]
fn ordered_number_from_number() {
    let n = Number::from(42);
    let on = OrderedNumber::from(n.clone());

    assert_eq!(*on.inner(), n);
}

#[test]
fn ordered_number_hashmap_update() {
    let mut map: HashMap<OrderedNumber, i32> = HashMap::new();

    let key = OrderedNumber::from(Number::from(1));
    map.insert(key.clone(), 100);
    map.insert(key.clone(), 200); // Update

    assert_eq!(map.len(), 1);
    assert_eq!(map.get(&key), Some(&200));
}

#[test]
fn ordered_number_negative_zero() {
    let zero = OrderedNumber::from(Number::ZERO());
    let neg_zero = OrderedNumber::from(Number::neg_zero());

    // -0 == 0
    assert_eq!(zero, neg_zero);
}
