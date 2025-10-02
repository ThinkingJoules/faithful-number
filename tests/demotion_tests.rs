use faithful_number::Number;
use num_rational::Ratio;

/// Test terminating vs non-terminating rational promotion
///
/// Current behavior: Both will promote to Decimal with RationalApproximation flag
/// Desired behavior: Terminating should use Decimal (exact), non-terminating should use BigDecimal with flag
#[test]
fn terminating_vs_non_terminating() {
    // Terminating: 1/2 should use Decimal (exact)
    let half = Number::from_rational(Ratio::new(1, 2));
    let large = Number::from(i64::MAX);
    let result = large + half;

    println!(
        "Terminating result representation: {}",
        result.representation()
    );
    println!("Terminating result is_exact: {}", result.is_exact());
    println!(
        "Terminating result is_rational_approximation: {}",
        result.is_rational_approximation()
    );

    // CURRENT: Will be Decimal with RationalApproximation flag
    // DESIRED: Should be Decimal with NO flag (exact representation)
    assert_eq!(result.representation(), "Decimal");
    assert!(result.is_exact()); // Should be exact
}

/// Test small non-terminating rational operations
///
/// Current behavior: Should stay as Rational64 (no overflow)
/// Desired behavior: Same - stays as Rational64
#[test]
fn non_terminating_small() {
    // Small non-terminating: stays as Rational64
    let third = Number::from_rational(Ratio::new(1, 3));
    let seventh = Number::from_rational(Ratio::new(1, 7));
    let result = third + seventh;

    println!(
        "Small non-terminating representation: {}",
        result.representation()
    );
    println!("Small non-terminating is_exact: {}", result.is_exact());

    // CURRENT and DESIRED: Should be Rational (no overflow)
    assert_eq!(result.representation(), "Rational");
    assert!(result.is_exact()); // Exact representation
}

/// Test large non-terminating rational preservation of flag
///
/// Current behavior: Promotes to Decimal with RationalApproximation flag
/// Desired behavior: Should promote to BigDecimal with RationalApproximation flag
#[test]
fn non_terminating_large_preserves_flag() {
    // Large non-terminating: BigDecimal with flag
    let third = Number::from_rational(Ratio::new(1, 3));
    let large = Number::from(i64::MAX);
    let result = large + third;

    println!(
        "Large non-terminating representation: {}",
        result.representation()
    );
    println!(
        "Large non-terminating is_rational_approximation: {}",
        result.is_rational_approximation()
    );

    // CURRENT: Will be Decimal with RationalApproximation
    // DESIRED: Should be BigDecimal with RationalApproximation
    assert_eq!(result.representation(), "BigDecimal");
    assert!(result.is_rational_approximation());
}

/// Test that magnitude prevents wasteful conversion attempts
///
/// Current behavior: No demotion logic exists yet, so this test just verifies current state
/// Desired behavior: Should NOT attempt rational recovery on large values
#[test]
fn magnitude_prevents_wasteful_conversion() {
    let max = Number::from(i64::MAX);
    let third = Number::from_rational(Ratio::new(1, 3));
    let large = max + third.clone();

    println!("Large value representation: {}", large.representation());
    println!(
        "Large value is_rational_approximation: {}",
        large.is_rational_approximation()
    );

    // CURRENT: Should be Decimal with RationalApproximation (or might be Decimal)
    // DESIRED: Should be BigDecimal with RationalApproximation, and should NOT attempt conversion (magnitude too large)

    // The main point: magnitude should be checked before attempting expensive rational recovery
    // This test documents current behavior - implementation will add magnitude check
}

/// Test rational recovery after magnitude reduction
///
/// This is the KEY test case for the new implementation
///
/// Current behavior: No automatic demotion logic, so result stays as Decimal/BigDecimal
/// Desired behavior: Should recover Rational64 after magnitude reduction
#[test]
fn rational_recovery_after_magnitude_reduction() {
    let max = Number::from(i64::MAX);
    let third = Number::from_rational(Ratio::new(1, 3));

    // Step 1: Create large non-terminating value
    let large = max.clone() + third.clone();
    println!("Step 1 - Large representation: {}", large.representation());
    println!(
        "Step 1 - Large is_rational_approximation: {}",
        large.is_rational_approximation()
    );

    // Step 2: Reduce magnitude through subtraction
    let result = large - max;
    println!(
        "Step 2 - Result representation: {}",
        result.representation()
    );
    println!("Step 2 - Result is_exact: {}", result.is_exact());

    // CURRENT: Will likely be Decimal (no automatic demotion to Rational)
    // DESIRED: Should successfully recover Rational64(1, 3)
    assert_eq!(result.representation(), "Rational");
    assert!(result.is_exact());
    assert_eq!(result, third);
}

/// Test transcendental operations clear rational flag
///
/// Current behavior: sqrt() sets Transcendental flag, overriding RationalApproximation
/// Desired behavior: Same
#[test]
fn transcendental_clears_rational_flag() {
    let third = Number::from_rational(Ratio::new(1, 3));
    let large = Number::from(i64::MAX) + third;

    println!(
        "Before sqrt - is_rational_approximation: {}",
        large.is_rational_approximation()
    );

    let result = large.sqrt();

    println!("After sqrt - representation: {}", result.representation());
    println!(
        "After sqrt - is_transcendental: {}",
        result.is_transcendental()
    );
    println!(
        "After sqrt - is_rational_approximation: {}",
        result.is_rational_approximation()
    );

    // CURRENT and DESIRED: Should be Transcendental now, not RationalApproximation
    assert!(result.is_transcendental());
    assert!(!result.is_rational_approximation());
}

/// Test flag propagates through arithmetic operations
///
/// Current behavior: Flag propagates through operations
/// Desired behavior: Same
#[test]
fn flag_propagates_through_arithmetic() {
    let max = Number::from(i64::MAX);
    let third = Number::from_rational(Ratio::new(1, 3));
    let large = max + third;

    println!(
        "Initial is_rational_approximation: {}",
        large.is_rational_approximation()
    );

    // Addition should preserve the flag
    let result = large.clone() + Number::from(1000);
    println!(
        "After +1000 - is_rational_approximation: {}",
        result.is_rational_approximation()
    );
    assert!(result.is_rational_approximation());

    // Subtraction should preserve the flag
    let result = large.clone() - Number::from(1000);
    println!(
        "After -1000 - is_rational_approximation: {}",
        result.is_rational_approximation()
    );
    assert!(result.is_rational_approximation());

    // Multiplication should preserve the flag
    let result = large.clone() * Number::from(2);
    println!(
        "After *2 - is_rational_approximation: {}",
        result.is_rational_approximation()
    );
    assert!(result.is_rational_approximation());
}

/// Test exact rational equality after operations
///
/// This test verifies that exact rational arithmetic is preserved when possible
#[test]
fn exact_rational_arithmetic() {
    let third = Number::from_rational(Ratio::new(1, 3));
    let two_thirds = Number::from_rational(Ratio::new(2, 3));

    // Should stay as exact rationals
    let result = third + two_thirds;
    println!("1/3 + 2/3 representation: {}", result.representation());
    println!("1/3 + 2/3 is_exact: {}", result.is_exact());

    // Should be exact Rational or Decimal representation of 1
    assert!(result.is_exact());
}

/// Test decimal to rational recovery (current behavior)
///
/// This documents current Decimal→Rational demotion behavior
#[test]
fn decimal_to_rational_recovery() {
    use rust_decimal::Decimal;

    // Create a Decimal that represents 1/3
    let dec = Decimal::new(1, 0) / Decimal::new(3, 0);
    let num = Number::from_decimal(dec);

    println!("Decimal 1/3 representation: {}", num.representation());
    println!("Decimal 1/3 is_exact: {}", num.is_exact());

    // CURRENT: from_decimal() already tries to demote to Rational
    // So this should be Rational if the demotion succeeded
    assert_eq!(num.representation(), "Rational");
}
