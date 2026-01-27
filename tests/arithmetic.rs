mod common;
use std::str::FromStr;

use common::*;
use faithful_number::Number;
use rust_decimal::Decimal;

// ============================================================================
// RATIONAL ARITHMETIC (most common case)
// ============================================================================

#[test]
fn test_rational_all_operations() {
    let a = rational!(2, 3);
    let b = rational!(1, 4);

    ArithmeticTestCase::new("2/3 op 1/4", a, b)
        .assert_add(rational!(11, 12), "Rational", exact()) // 2/3 + 1/4 = 11/12
        .assert_sub(rational!(5, 12), "Rational", exact()) // 2/3 - 1/4 = 5/12
        .assert_mul(rational!(1, 6), "Rational", exact()) // 2/3 * 1/4 = 1/6
        .assert_div(rational!(8, 3), "Rational", exact()) // 2/3 ÷ 1/4 = 8/3
        .assert_rem(rational!(1, 6), "Rational", exact()); // 2/3 % 1/4 = 1/6
}

#[test]
fn test_rational_addition() {
    ArithmeticTestCase::new("1/3 + 1/3", rational!(1, 3), rational!(1, 3)).assert_add(
        rational!(2, 3),
        "Rational",
        exact(),
    );
}

#[test]
fn test_rational_subtraction() {
    ArithmeticTestCase::new("2/3 - 1/3", rational!(2, 3), rational!(1, 3)).assert_sub(
        rational!(1, 3),
        "Rational",
        exact(),
    );

    ArithmeticTestCase::new("1/2 - 1/2", rational!(1, 2), rational!(1, 2)).assert_sub(
        Number::ZERO(),
        "Rational",
        exact(),
    );
}

#[test]
fn test_rational_multiplication() {
    ArithmeticTestCase::new("2/3 * 3/4", rational!(2, 3), rational!(3, 4)).assert_mul(
        rational!(1, 2),
        "Rational",
        exact(),
    );

    ArithmeticTestCase::new("2/3 * 3/2", rational!(2, 3), rational!(3, 2)).assert_mul(
        Number::ONE(),
        "Rational",
        exact(),
    );
}

#[test]
fn test_rational_division() {
    ArithmeticTestCase::new("2/3 ÷ 4/5", rational!(2, 3), rational!(4, 5)).assert_div(
        rational!(5, 6),
        "Rational",
        exact(),
    );

    ArithmeticTestCase::new("3/7 ÷ 3/7", rational!(3, 7), rational!(3, 7)).assert_div(
        Number::ONE(),
        "Rational",
        exact(),
    );
}

#[test]
fn test_integer_operations() {
    // Integers are stored as Rational(n, 1)
    ArithmeticTestCase::new("7 op 3", Number::from(7), Number::from(3))
        .assert_add(Number::from(10), "Rational", exact())
        .assert_sub(Number::from(4), "Rational", exact())
        .assert_mul(Number::from(21), "Rational", exact())
        .assert_div(rational!(7, 3), "Rational", exact())
        .assert_rem(Number::from(1), "Rational", exact()); // 7 % 3 = 1 (demotes to Rational)
}

#[test]
fn test_integer_basic_ops() {
    ArithmeticTestCase::new("1 + 1", Number::from(1), Number::from(1))
        .assert_add(Number::from(2), "Rational", exact())
        .assert_sub(Number::from(0), "Rational", exact())
        .assert_mul(Number::from(1), "Rational", exact())
        .assert_div(Number::from(1), "Rational", exact());
}

#[test]
fn test_negative_rationals() {
    ArithmeticTestCase::new("5 + -3", Number::from(5), Number::from(-3))
        .assert_add(Number::from(2), "Rational", exact())
        .assert_mul(Number::from(-15), "Rational", exact());

    let pos = rational!(3, 4);
    let neg = rational!(-1, 2);

    ArithmeticTestCase::new("3/4 + -1/2", pos, neg)
        .assert_add(rational!(1, 4), "Rational", exact())
        .assert_mul(rational!(-3, 8), "Rational", exact());
}

#[test]
fn test_simplification() {
    // 4/2 should simplify to 2/1
    let result = rational!(4, 2);
    assert_eq!(result.representation(), "Rational");
    assert_eq!(result, Number::from(2));
}

// ============================================================================
// DECIMAL ARITHMETIC (high precision, exceeds i64 for rational form)
// ============================================================================

#[test]
fn test_decimal_high_precision_addition() {
    // Use numbers with enough significant digits that rational form exceeds i64
    // Decimal can hold 28 significant digits
    let a = Number::from_decimal(Decimal::from_str("123456789012345678.90123456").unwrap());
    let b = Number::from_decimal(Decimal::from_str("987654321098765432.10987654").unwrap());

    // These should stay as Decimal because the rational representation would overflow i64
    assert_eq!(a.representation(), "Decimal");
    assert_eq!(b.representation(), "Decimal");

    let result = a + b;
    assert_eq!(result.representation(), "Decimal");
    assert!(result.is_exact());
}

#[test]
fn test_decimal_high_precision_multiplication() {
    // Create numbers that need Decimal precision
    let a = Number::from_decimal(Decimal::from_str("1234567890.123456789").unwrap());
    let b = Number::from_decimal(Decimal::from_str("9876543210.987654321").unwrap());

    let result = a * b;
    // Result should still be manageable (might be Decimal or BigDecimal depending on overflow)
    assert!(result.is_finite());
    assert!(result.is_exact());
}

#[test]
fn test_decimal_near_max_precision() {
    // Decimal has 96-bit mantissa (28-29 significant decimal digits)
    let max_safe =
        Number::from_decimal(Decimal::from_str("12345678901234567890123456.78").unwrap());

    assert_eq!(max_safe.representation(), "Decimal");

    let result = max_safe.clone() + Number::from(1);
    assert!(result.is_finite());
}

#[test]
fn test_decimal_fractional_high_precision() {
    // Many decimal places that can't be represented as simple rational
    let pi_approx =
        Number::from_decimal(Decimal::from_str("3.14159265358979323846264338").unwrap());

    // This might demote to Rational if the implementation finds a close rational
    // or stay as Decimal if it can't fit in i64 rational
    let result = pi_approx.clone() * Number::from(2);
    assert!(result.is_finite());
}

// ============================================================================
// BIGDECIMAL ARITHMETIC (exceeds Decimal capacity)
// ============================================================================

#[test]
fn test_bigdecimal_very_large() {
    // Create a number that exceeds Decimal's range
    let huge = Number::try_from_i128_with_scale(i128::MAX / 1000, 0).unwrap();

    // Should be BigDecimal or might fit in Decimal
    let result = huge.clone() + huge;
    assert!(result.is_finite());
}

#[test]
fn test_bigdecimal_arithmetic() {
    // Numbers beyond Decimal's 96-bit mantissa
    use bigdecimal::BigDecimal;
    use std::str::FromStr;

    let huge1 = Number::from_bigdecimal(
        BigDecimal::from_str("123456789012345678901234567890123456789").unwrap(),
    );
    let huge2 = Number::from_bigdecimal(
        BigDecimal::from_str("987654321098765432109876543210987654321").unwrap(),
    );

    let result = huge1 + huge2;
    assert!(result.is_finite());
    // Representation will be BigDecimal or might demote if result fits
}

#[test]
fn test_rational_overflow_to_decimal() {
    // Multiply large rationals that would overflow i64 in rational form
    let large1 = Number::from(i64::MAX / 2);
    let large2 = Number::from(3);

    let result = large1 * large2;

    // Should overflow rational representation
    assert!(result.is_finite());
    // Will be Decimal or BigDecimal depending on magnitude
}

// ============================================================================
// SPECIAL VALUES
// ============================================================================

#[test]
fn test_infinity_arithmetic() {
    let inf = Number::infinity();
    let neg_inf = Number::neg_infinity();

    ArithmeticTestCase::new("inf + 1", inf.clone(), Number::from(1)).assert_add(
        inf.clone(),
        "PositiveInfinity",
        exact(),
    );

    ArithmeticTestCase::new("inf + inf", inf.clone(), inf.clone()).assert_add(
        inf.clone(),
        "PositiveInfinity",
        exact(),
    );

    ArithmeticTestCase::new("inf + -inf", inf.clone(), neg_inf.clone()).assert_add(
        Number::nan(),
        "NaN",
        exact(),
    );

    ArithmeticTestCase::new("inf * 2", inf.clone(), Number::from(2)).assert_mul(
        inf.clone(),
        "PositiveInfinity",
        exact(),
    );

    ArithmeticTestCase::new("inf * -1", inf.clone(), Number::from(-1)).assert_mul(
        neg_inf.clone(),
        "NegativeInfinity",
        exact(),
    );

    ArithmeticTestCase::new("inf * 0", inf.clone(), Number::ZERO()).assert_mul(
        Number::nan(),
        "NaN",
        exact(),
    );
}

#[test]
fn test_nan_arithmetic() {
    let nan = Number::nan();

    ArithmeticTestCase::new("NaN + 1", nan.clone(), Number::from(1)).assert_add(
        nan.clone(),
        "NaN",
        exact(),
    );

    ArithmeticTestCase::new("NaN * 5", nan.clone(), Number::from(5)).assert_mul(
        nan.clone(),
        "NaN",
        exact(),
    );

    // NaN comparison depends on feature flag
    #[cfg(feature = "js_nan_equality")]
    assert_eq!(nan, nan);
    #[cfg(not(feature = "js_nan_equality"))]
    assert_ne!(nan, nan); // IEEE 754: NaN != NaN
}

#[test]
fn test_division_by_zero() {
    ArithmeticTestCase::new("5 ÷ 0", Number::from(5), Number::ZERO()).assert_div(
        Number::infinity(),
        "PositiveInfinity",
        exact(),
    );

    ArithmeticTestCase::new("-5 ÷ 0", Number::from(-5), Number::ZERO()).assert_div(
        Number::neg_infinity(),
        "NegativeInfinity",
        exact(),
    );

    ArithmeticTestCase::new("0 ÷ 0", Number::ZERO(), Number::ZERO()).assert_div(
        Number::nan(),
        "NaN",
        exact(),
    );
}

#[test]
fn test_negative_zero() {
    let neg_zero = Number::neg_zero();

    ArithmeticTestCase::new("-0 * 1", neg_zero.clone(), Number::ONE()).assert_mul(
        neg_zero.clone(),
        "NegativeZero",
        exact(),
    );
}

// ============================================================================
// PROPERTY-BASED TESTS
// ============================================================================

#[test]
fn test_rational_properties() {
    CombinatorialTest::new("rational_properties")
        .operand("1/2", rational!(1, 2))
        .operand("1/3", rational!(1, 3))
        .operand("2/5", rational!(2, 5))
        .assert_commutative_add()
        .assert_commutative_mul()
        .assert_associative_add()
        .assert_associative_mul()
        .assert_distributive()
        .assert_additive_identity()
        .assert_multiplicative_identity()
        .assert_additive_inverse()
        .assert_multiplicative_inverse();
}

#[test]
fn test_integer_properties() {
    CombinatorialTest::new("integer_properties")
        .operand("5", Number::from(5))
        .operand("-3", Number::from(-3))
        .operand("42", Number::from(42))
        .assert_commutative_add()
        .assert_commutative_mul()
        .assert_associative_add()
        .assert_associative_mul()
        .assert_distributive()
        .assert_additive_identity()
        .assert_multiplicative_identity()
        .assert_additive_inverse()
        .assert_multiplicative_inverse();
}

#[test]
fn test_mixed_magnitude_properties() {
    CombinatorialTest::new("mixed_magnitude")
        .operand("small_int", Number::from(7))
        .operand("rational", rational!(22, 7)) // pi approximation
        .operand("large_int", Number::from(1_000_000))
        .assert_commutative_add()
        .assert_commutative_mul()
        .assert_additive_identity()
        .assert_multiplicative_identity();
}
