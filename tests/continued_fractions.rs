//! Adversarial tests for continued fractions algorithm.

use faithful_number::Number;
use std::str::FromStr;

#[test]
fn near_one_not_rounded() {
    // 0.999... with finite precision should NOT become exactly 1
    // Note: Due to parsing limits, very long decimals may round
    let near_one = Number::from_str("0.999999999").unwrap();

    // Should not equal 1 exactly
    assert!(near_one != Number::from(1));
}

#[test]
fn repeating_decimal_finds_fraction() {
    // 0.333... should be representable and close to 1/3
    let third = Number::from(1) / Number::from(3);
    let direct = Number::from_str("0.333333333333333333").unwrap();

    // They should be very close (within Decimal precision)
    let diff = (third.to_f64() - direct.to_f64()).abs();
    assert!(diff < 1e-15);
}

#[test]
fn one_seventh_pattern() {
    // 1/7 = 0.142857142857...
    let seventh = Number::from(1) / Number::from(7);

    // Check it's stored as Rational
    assert_eq!(seventh.representation(), "Rational");

    // Verify the pattern
    let f = seventh.to_f64();
    assert!((f - 1.0 / 7.0).abs() < 1e-15);
}

#[test]
fn terminating_decimal_exact() {
    // 0.125 = 1/8 exactly
    let eighth = Number::from_str("0.125").unwrap();

    // Should be exact
    assert!(eighth.is_exact());

    // Should equal 1/8
    let one_eighth = Number::from(1) / Number::from(8);
    assert_eq!(eighth, one_eighth);
}

#[test]
fn non_terminating_stays_non_terminating() {
    // 1/3 is non-terminating, should stay that way
    let third = Number::from(1) / Number::from(3);

    // Check it's Rational (non-terminating flag internal)
    assert_eq!(third.representation(), "Rational");

    // 3 * (1/3) should be exactly 1
    let result = third * Number::from(3);
    assert_eq!(result, Number::from(1));
}

#[test]
fn very_small_decimal() {
    // Very small values shouldn't cause issues
    let tiny = Number::from_str("0.0000000000000000000000000001").unwrap();

    assert!(!tiny.is_nan());
    assert!(tiny.to_f64() > 0.0);
}

#[test]
fn phi_approximation() {
    // Golden ratio has slow CF convergence
    // phi = (1 + sqrt(5)) / 2 ≈ 1.618033988749895
    let phi_approx = Number::from_str("1.618033988749895").unwrap();

    assert!(!phi_approx.is_nan());
    assert!(phi_approx.to_f64() > 1.6);
    assert!(phi_approx.to_f64() < 1.62);
}

#[test]
fn exact_decimal_recovery() {
    // Numbers that are exactly representable should remain exact
    let exact = Number::from_str("123.456").unwrap();

    // Should be exact (no approximation flag)
    assert!(exact.is_exact());
}
