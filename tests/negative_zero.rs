//! Adversarial tests for negative zero handling.

use faithful_number::Number;

#[test]
fn neg_zero_identity() {
    let neg_zero = Number::neg_zero();
    assert!(neg_zero.is_neg_zero());
}

#[test]
fn neg_zero_plus_zero() {
    // (-0) + 0 = 0 (positive zero)
    let result = Number::neg_zero() + Number::ZERO;
    assert!(!result.is_neg_zero());
    assert!(result.is_zero());
}

#[test]
fn zero_plus_neg_zero() {
    // 0 + (-0) = 0 (positive zero per IEEE)
    let result = Number::ZERO + Number::neg_zero();
    assert!(!result.is_neg_zero());
}

#[test]
fn neg_zero_plus_neg_zero() {
    // (-0) + (-0) = -0
    let result = Number::neg_zero() + Number::neg_zero();
    assert!(result.is_neg_zero());
}

#[test]
fn neg_zero_times_positive() {
    // (-0) * 5 = -0
    let result = Number::neg_zero() * Number::from(5);
    assert!(result.is_neg_zero());
}

#[test]
fn neg_zero_times_negative() {
    // (-0) * (-5) = 0 (positive)
    let result = Number::neg_zero() * Number::from(-5);
    assert!(!result.is_neg_zero());
    assert!(result.is_zero());
}

#[test]
fn neg_zero_times_neg_zero() {
    // (-0) * (-0) = +0
    let result = Number::neg_zero() * Number::neg_zero();
    assert!(!result.is_neg_zero());
}

#[test]
fn one_divided_by_neg_zero() {
    // 1 / (-0) = -Infinity
    let result = Number::from(1) / Number::neg_zero();
    assert!(result.is_neg_infinity());
}

#[test]
fn neg_one_divided_by_neg_zero() {
    // (-1) / (-0) = +Infinity
    let result = Number::from(-1) / Number::neg_zero();
    assert!(result.is_infinite());
    assert!(result.to_f64() > 0.0);
}

#[test]
fn neg_zero_equals_zero() {
    // -0 == 0 per IEEE
    let neg_zero = Number::neg_zero();
    let zero = Number::ZERO;

    assert_eq!(neg_zero, zero);
}

#[test]
fn neg_zero_display() {
    // -0 displays as "0"
    assert_eq!(Number::neg_zero().to_string(), "0");
}

#[test]
fn neg_zero_is_falsy() {
    assert!(Number::neg_zero().is_falsy());
}

#[test]
fn neg_zero_subtraction() {
    // 0 - 0 = 0 (not -0)
    let result = Number::ZERO - Number::ZERO;
    assert!(!result.is_neg_zero());
}

#[test]
fn neg_zero_preserved_in_operations() {
    // Adding a non-zero value then subtracting it should not create -0
    let x = Number::from(5);
    let result = Number::neg_zero() + x.clone() - x;

    // Result should be 0 (positive), not preserved as -0
    assert!(result.is_zero());
}
