//! Tests for associativity under overflow conditions.
//!
//! These tests document cases where arithmetic associativity may fail
//! due to representation changes during overflow.

use faithful_number::Number;

#[test]
fn addition_associativity_normal() {
    // (a + b) + c == a + (b + c) for normal values
    let a = Number::from(1);
    let b = Number::from(2);
    let c = Number::from(3);

    let left = (a.clone() + b.clone()) + c.clone();
    let right = a + (b + c);

    assert_eq!(left, right);
}

#[test]
fn multiplication_associativity_normal() {
    // (a * b) * c == a * (b * c) for normal values
    let a = Number::from(2);
    let b = Number::from(3);
    let c = Number::from(4);

    let left = (a.clone() * b.clone()) * c.clone();
    let right = a * (b * c);

    assert_eq!(left, right);
}

#[test]
fn addition_at_overflow_boundary() {
    // (a + b) + c vs a + (b + c) when overflow occurs
    let a = Number::from(i64::MAX - 1);
    let b = Number::from(2);
    let c = Number::from(-1);

    let left = (a.clone() + b.clone()) + c.clone(); // Overflows, then subtracts
    let right = a + (b + c); // No overflow (2-1=1)

    // Both should give the same numeric result
    assert_eq!(left.to_f64(), right.to_f64());
}

#[test]
fn distributive_property() {
    // a * (b + c) == a*b + a*c
    let a = Number::from(5);
    let b = Number::from(3);
    let c = Number::from(7);

    let left = a.clone() * (b.clone() + c.clone());
    let right = (a.clone() * b) + (a * c);

    assert_eq!(left, right);
}

#[test]
fn inverse_operations_return_original() {
    // (a + b) - b == a
    let a = Number::from(42);
    let b = Number::from(1000);

    let result = (a.clone() + b.clone()) - b;
    assert_eq!(result, a);
}

#[test]
fn division_multiplication_inverse() {
    // (a * b) / b == a (for non-zero b)
    let a = Number::from(42);
    let b = Number::from(7);

    let result = (a.clone() * b.clone()) / b;
    assert_eq!(result, a);
}

#[test]
fn repeated_operations_preserve_value() {
    // a + b - b + b - b == a
    let a = Number::from(100);
    let b = Number::from(50);

    let result = ((((a.clone() + b.clone()) - b.clone()) + b.clone()) - b);
    assert_eq!(result, a);
}

#[test]
fn large_denominator_arithmetic() {
    // Operations that create then reduce large denominators
    let a = Number::from(1) / Number::from(7);
    let b = Number::from(1) / Number::from(11);

    // a + b then subtract a should give b
    let sum = a.clone() + b.clone();
    let result = sum - a;

    // Should be equal to b
    assert_eq!(result, b);
}

#[test]
fn multiplication_by_zero() {
    let large = Number::from(i64::MAX);
    let zero = Number::ZERO();

    assert_eq!(large * zero, Number::ZERO());
}

#[test]
fn additive_identity() {
    let values = vec![
        Number::from(42),
        Number::from(-100),
        Number::from(0),
        Number::from_str("123.456").unwrap(),
    ];

    for v in values {
        assert_eq!(v.clone() + Number::ZERO(), v);
        assert_eq!(Number::ZERO() + v.clone(), v);
    }
}

use std::str::FromStr;
