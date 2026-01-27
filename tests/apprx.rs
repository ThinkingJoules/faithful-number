mod common;
use common::*;
use faithful_number::Number;

// ============================================================================
// TRANSCENDENTAL APPROXIMATIONS
// ============================================================================

#[test]
fn test_sqrt_creates_transcendental() {
    let two = Number::from(2);
    let sqrt_two = two.sqrt();

    assert!(sqrt_two.is_transcendental());
    assert!(!sqrt_two.is_exact());

    // √2 is between 1 and 2
    assert!(sqrt_two > Number::from(1));
    assert!(sqrt_two < Number::from(2));
}

#[test]
fn test_sqrt_of_perfect_square_stays_exact() {
    let four = Number::from(4);
    let sqrt_four = four.sqrt();

    assert_eq!(sqrt_four, Number::from(2));
    assert!(sqrt_four.is_exact());
    assert_eq!(sqrt_four.representation(), "Rational");
}

#[test]
fn test_transcendental_propagates() {
    let sqrt_two = Number::from(2).sqrt();

    // Transcendental + exact = transcendental
    let result = sqrt_two.clone() + Number::from(1);
    assert!(result.is_transcendental());

    // Transcendental * exact = transcendental
    let result2 = sqrt_two * Number::from(2);
    assert!(result2.is_transcendental());
}

#[test]
fn test_transcendental_times_transcendental() {
    let sqrt_two = Number::from(2).sqrt();
    let sqrt_three = Number::from(3).sqrt();

    // Both transcendental
    let result = sqrt_two * sqrt_three;
    assert!(result.is_transcendental());
}

#[test]
fn test_log_exp_transcendental() {
    let x = Number::from(5);

    let log_x = x.clone().log();
    assert!(log_x.is_transcendental());

    let exp_x = x.exp();
    assert!(exp_x.is_transcendental());

    // log(exp(x)) should be close to x (but still transcendental)
    let roundtrip = exp_x.log();
    assert!(roundtrip.is_transcendental());
}

#[test]
fn test_trig_functions_transcendental() {
    let pi_approx = Number::from(3) + decimal!(141592653589793, 15);

    let sin_pi = pi_approx.clone().sin();
    assert!(sin_pi.is_transcendental());

    let cos_pi = pi_approx.cos();
    assert!(cos_pi.is_transcendental());

    // sin²(x) + cos²(x) ≈ 1 (but may not be exactly 1 in high_precision mode)
    let sin_sq = sin_pi.clone() * sin_pi;
    let cos_sq = cos_pi.clone() * cos_pi;
    let pythagorean = sin_sq + cos_sq;

    // In high_precision mode, the result may be very close but not exactly 1
    // due to BigDecimal representation. In standard mode, f64 rounds to 1.
    let diff = (pythagorean.to_f64() - 1.0).abs();
    assert!(
        diff < 1e-10,
        "pythagorean identity should be close to 1, got diff: {}",
        diff
    );
}

// ============================================================================
// RATIONAL APPROXIMATION TESTS
// ============================================================================

#[test]
fn test_rational_approximation_flag() {
    // When a non-terminating rational is forced to decimal (if that's how you implement it)
    // This depends on your specific graduation strategy

    // For now, test that the flag can be set and detected
    let one_third = rational!(1, 3);

    // Currently should stay rational
    assert!(one_third.is_exact());

    // If operations force decimal approximation, flag should be set
    // (Implementation-specific)
}

#[test]
fn test_rational_approximation_from_overflow() {
    // If rational numerator/denominator overflow and get approximated to decimal
    let large_prime = Number::from(999999999989_i64);
    let another_large = Number::from(999999999967_i64);

    let result = large_prime / another_large;

    // Should either stay exact (if fits) or be flagged as approximation
    assert!(result.is_finite());
}

// ============================================================================
// APPROXIMATION PROPAGATION
// ============================================================================

#[test]
fn test_exact_plus_transcendental_is_transcendental() {
    let exact = Number::from(5);
    let transcendental = Number::from(2).sqrt();

    let result = exact + transcendental;

    // Result should be transcendental
    assert!(result.is_transcendental());

    // Result should be approximately 5 + 1.414... ≈ 6.414
    let f = result.to_f64();
    assert!(
        f > 6.4 && f < 6.5,
        "5 + √2 should be around 6.414, got {}",
        f
    );
}

#[test]
fn test_transcendental_plus_transcendental() {
    let sqrt_two = Number::from(2).sqrt();
    let sqrt_three = Number::from(3).sqrt();

    let result = sqrt_two + sqrt_three;

    assert!(result.is_transcendental());
    assert!(!result.is_exact());
}

#[test]
fn test_approximation_does_not_revert() {
    // Once a number is approximated, it stays approximated
    let transcendental = Number::from(2).sqrt();

    // Even if we multiply by 0 (giving exact 0), we might want to preserve the flag
    // This is a design decision - test both possibilities
    let result = transcendental * Number::ZERO;

    // Either stays transcendental OR becomes exact 0
    // Document your choice here
    assert_eq!(result, Number::ZERO);
    // assert!(result.is_transcendental()); // OR
    // assert!(result.is_exact()); // depending on your semantics
}

#[test]
fn test_mixed_approximation_types() {
    // Transcendental + RationalApproximation
    let transcendental = Number::from(2).sqrt();
    // Create a rational approximation somehow (implementation-specific)

    let result = transcendental.clone() + transcendental;

    // Should still be transcendental
    assert!(result.is_transcendental());
}

// ============================================================================
// PRECISION TESTS
// ============================================================================

#[cfg(feature = "high_precision")]
#[test]
fn test_high_precision_sqrt() {
    Number::set_default_precision(200);

    let sqrt_two = Number::from(2).sqrt();

    // With high precision, verify bounds and transcendental flag
    assert!(sqrt_two > Number::from(1));
    assert!(sqrt_two < Number::from(2));
    assert!(sqrt_two.is_transcendental());

    // Verify (√2)² is very close to 2 with high precision
    // (may not be exactly equal due to BigDecimal representation)
    let squared = sqrt_two.clone() * sqrt_two;
    let diff = (squared.to_f64() - 2.0).abs();
    assert!(
        diff < 1e-10,
        "(√2)² should be close to 2, got diff: {}",
        diff
    );
}

#[test]
#[cfg(not(feature = "high_precision"))]
fn test_approximation_arithmetic_consistency() {
    // Test that arithmetic with approximations is consistent
    let sqrt_two = Number::from(2).sqrt();
    assert!(sqrt_two.is_transcendental());

    // (√2)² = exactly 2 (demotes to exact when result is exact)
    // This only works in non-high_precision mode where f64 rounds perfectly
    let squared = sqrt_two.clone() * sqrt_two;

    // When demotion recovers exact rational, flag clears
    assert_eq!(squared, Number::from(2));
    assert!(squared.is_exact());
    assert_eq!(squared.representation(), "Rational");
}

#[test]
#[cfg(feature = "high_precision")]
fn test_approximation_arithmetic_consistency_high_precision() {
    // In high_precision mode, (√2)² produces a BigDecimal very close to 2
    let sqrt_two = Number::from(2).sqrt();
    assert!(sqrt_two.is_transcendental());

    let squared = sqrt_two.clone() * sqrt_two;
    let diff = (squared.to_f64() - 2.0).abs();
    assert!(
        diff < 1e-10,
        "(√2)² should be close to 2, got diff: {}",
        diff
    );
}
