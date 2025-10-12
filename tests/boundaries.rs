mod common;
use common::*;
use faithful_number::Number;

// ============================================================================
// RATIONAL → DECIMAL GRADUATION
// ============================================================================

#[test]
fn test_rational_stays_rational_when_possible() {
    // Operations that should keep rational representation
    let one_third = rational!(1, 3);
    let two_thirds = rational!(2, 3);

    // 1/3 + 2/3 = 1 (should demote to simplest form)
    ArithmeticTestCase::new("1/3 + 2/3", one_third.clone(), two_thirds).assert_add(
        Number::from(1),
        "Rational",
        exact(),
    );

    // 1/3 * 3 = 1 (should demote)
    ArithmeticTestCase::new("1/3 * 3", one_third, Number::from(3)).assert_mul(
        Number::from(1),
        "Rational",
        exact(),
    );
}

#[test]
fn test_rational_overflow_to_bigdecimal() {
    // When rational numerator/denominator would overflow i64
    let large = Number::from(i64::MAX);
    let small_rational = rational!(1, 3);

    // This might overflow and graduate to BigDecimal
    let result = large * small_rational;

    // Just verify it doesn't panic and stays finite
    assert!(result.is_finite());
    assert!(result.is_exact() || result.is_rational_approximation());
}

#[test]
fn test_non_terminating_rational_stays_rational() {
    // 1/3 cannot be exactly represented as decimal, stays rational
    let result = Number::from(1) / Number::from(3);

    assert_eq!(result.representation(), "Rational");
    assert!(result.is_exact());
    assert_eq!(result, rational!(1, 3));
}

// ============================================================================
// DECIMAL → RATIONAL DEMOTION
// ============================================================================

#[test]
fn test_decimal_demotes_to_rational() {
    // 0.5 should be recognized as 1/2
    let half_decimal = decimal!(5, 1); // 0.5
    let two = Number::from(2);

    // 0.5 * 2 = 1 (should demote to rational or integer)
    ArithmeticTestCase::new("0.5 * 2", half_decimal, two).assert_mul(
        Number::from(1),
        "Rational",
        exact(),
    );
}

#[test]
fn test_decimal_addition_stays_exact() {
    // Terminating decimals stay exact (demotes to Rational when result is simple)
    let a = decimal!(125, 2); // 1.25
    let b = decimal!(375, 2); // 3.75

    ArithmeticTestCase::new("1.25 + 3.75", a, b).assert_add(Number::from(5), "Rational", exact());
}

#[test]
fn test_decimal_to_rational_recovery() {
    // After operations, decimals that represent simple fractions should demote
    let quarter = decimal!(25, 2); // 0.25 = 1/4
    let result = quarter * Number::from(4);

    // Should demote to exact integer
    assert_eq!(result, Number::from(1));
    assert!(result.is_exact());
}

// ============================================================================
// BIGDECIMAL BOUNDARIES
// ============================================================================

#[test]
fn test_very_large_numbers_use_bigdecimal() {
    // Numbers beyond Decimal's range should use BigDecimal
    // Create a very large number by multiplying i64::MAX repeatedly
    let huge = Number::from(i64::MAX) * Number::from(i64::MAX);

    // Should still compute without panic
    let result = huge.clone() + Number::ONE;
    assert!(result.is_finite());
    assert_eq!(result.representation(), "BigDecimal");
}

#[test]
fn test_bigdecimal_demotes_when_small_enough() {
    // If BigDecimal result fits in Decimal, it should demote
    // This is implementation-dependent, but we can test the invariant
    let big = Number::from_bigdecimal(bigdecimal::BigDecimal::from(1000));

    let result = big + Number::from(1);

    // Should still be manageable
    assert!(result.is_finite());
    assert_eq!(result, Number::from(1001));
}

// ============================================================================
// MIXED REPRESENTATION OPERATIONS
// ============================================================================

#[test]
fn test_rational_plus_decimal() {
    let rational = rational!(1, 2); // 0.5
    let decimal = decimal!(25, 2); // 0.25

    // 0.5 + 0.25 = 0.75 = 3/4
    ArithmeticTestCase::new("1/2 + 0.25", rational, decimal).assert_add(
        rational!(3, 4),
        "Rational",
        exact(),
    );
}

#[test]
fn test_rational_times_decimal() {
    let rational = rational!(2, 3);
    let decimal = decimal!(15, 1); // 1.5 = 3/2

    // 2/3 * 3/2 = 1
    ArithmeticTestCase::new("2/3 * 1.5", rational, decimal).assert_mul(
        Number::from(1),
        "Rational",
        exact(),
    );
}

#[test]
fn test_decimal_divided_by_rational() {
    let decimal = decimal!(5, 0); // 5
    let rational = rational!(2, 1); // 2

    // 5 / 2 = 2.5 = 5/2
    ArithmeticTestCase::new("5 / 2", decimal, rational).assert_div(
        rational!(5, 2),
        "Rational",
        exact(),
    );
}

#[test]
fn test_cross_representation_properties() {
    // Test algebraic laws hold across representations
    CombinatorialTest::new("mixed_repr")
        .operand("int", Number::from(7))
        .operand("rational", rational!(3, 4))
        .operand("decimal", decimal!(125, 2))
        .assert_commutative_add()
        .assert_commutative_mul()
        .assert_distributive()
        .assert_additive_identity()
        .assert_multiplicative_identity();
}

#[test]
fn test_representation_stability_under_operations() {
    // Once graduated to a higher representation, operations should maintain it
    // (unless demotion is explicitly possible)

    let rational = rational!(1, 3);
    let decimal = Number::from(2);

    // 1/3 + 2 should stay rational
    let result = rational + decimal;
    assert_eq!(result.representation(), "Rational");
    assert!(result.is_exact());
}
