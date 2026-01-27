//! Adversarial tests for overflow boundary conditions.

use faithful_number::Number;

#[test]
fn rational_near_i64_max_addition() {
    // (i64::MAX - 1) + 2 should overflow Rational and promote to BigDecimal
    let near_max = Number::from(i64::MAX - 1);
    let two = Number::from(2);

    let result = near_max + two;

    // Should not panic, should be correct value
    let expected = i64::MAX as i128 + 1;
    assert_eq!(result.to_string(), expected.to_string());
}

#[test]
fn rational_multiplication_overflow() {
    // Large numbers that overflow when multiplied
    let large = Number::from(i64::MAX / 2);
    let three = Number::from(3);

    let result = large * three;

    // Should not panic
    assert!(result.to_f64() > 0.0);
}

#[test]
fn denominator_overflow_in_addition() {
    // Adding rationals with large denominators that overflow
    // 1/i64::MAX + 1/i64::MAX could overflow denominator
    let small = Number::from(1) / Number::from(i64::MAX);
    let result = small.clone() + small;

    // Should handle gracefully
    assert!(!result.is_nan());
}

#[test]
fn division_creating_huge_denominator() {
    // 1 / very_small creates a large denominator
    let small = Number::from(1) / Number::from(i64::MAX);
    let result = Number::from(1) / small;

    // Should recover to approximately i64::MAX
    let f = result.to_f64();
    assert!(f > (i64::MAX / 2) as f64);
}

#[test]
fn subtraction_near_i64_min() {
    // Operations near i64::MIN should not panic
    let near_min = Number::from(i64::MIN + 100);
    let hundred = Number::from(200);

    let result = near_min - hundred;

    // Should not panic and produce a valid result
    assert!(!result.is_nan());
    assert!(result.to_f64() < 0.0);
}

#[test]
fn chained_operations_returning_to_rational() {
    // x * y / y should return to x (in Rational)
    let x = Number::from(42);
    let y = Number::from(i64::MAX / 100);

    let result = (x.clone() * y.clone()) / y;

    assert_eq!(result, x);
    assert_eq!(result.representation(), "Rational");
}

#[test]
fn u64_max_conversion() {
    let big = Number::from(u64::MAX);

    // Should not panic, should represent correctly
    assert!(big.to_f64() > 0.0);
}

#[test]
fn negative_zero_arithmetic_boundaries() {
    let neg_zero = Number::neg_zero();
    let large = Number::from(i64::MAX);

    // -0 + large = large
    let result = neg_zero + large.clone();
    assert_eq!(result, large);
}
