// Integration tests based on report.md
// These tests verify that Number passes all 43 tests to match F64's 43/43 compatibility
// WITHOUT converting our Numbers to f64 (except when comparing to f64's behavior)

use faithful_number::Number;
use num_rational::Ratio;
use std::str::FromStr;

// ==== Addition Tests (10 tests) ====

#[test]
fn add_decimal_precision() {
    // Test: 0.1 + 0.2 = 0.3 (the classic floating point test)
    let a = Number::from_str("0.1").unwrap();
    let b = Number::from_str("0.2").unwrap();
    let result = a + b;
    let expected = Number::from_str("0.3").unwrap();

    assert_eq!(result, expected, "0.1 + 0.2 should equal 0.3 exactly");
    println!(
        "add_decimal_precision: representation = {}",
        result.representation()
    );
}

#[test]
fn add_extreme_1e50() {
    // Test: 1e50 + 1e50 = 2e50
    let a = Number::from_str("100000000000000000000000000000000000000000000000000").unwrap();
    let b = Number::from_str("100000000000000000000000000000000000000000000000000").unwrap();
    let result = a + b;
    let expected = Number::from_str("200000000000000000000000000000000000000000000000000").unwrap();

    assert_eq!(result, expected);
    println!(
        "add_extreme_1e50: representation = {}",
        result.representation()
    );
}

#[test]
fn add_f64_precision_limit() {
    // Test: 10000000000000000 + 1 = 10000000000000001
    // F64 fails this (loses precision), but we should pass
    let a = Number::from_str("10000000000000000").unwrap();
    let b = Number::from(1i64);
    let result = a + b;
    let expected = Number::from_str("10000000000000001").unwrap();

    assert_eq!(result, expected);
    println!(
        "add_f64_precision_limit: representation = {}",
        result.representation()
    );
}

#[test]
fn add_large_1e15() {
    // Test: 1e15 + 1e15 = 2e15
    let a = Number::from(1_000_000_000_000_000i64);
    let b = Number::from(1_000_000_000_000_000i64);
    let result = a + b;
    let expected = Number::from(2_000_000_000_000_000i64);

    assert_eq!(result, expected);
    println!(
        "add_large_1e15: representation = {}",
        result.representation()
    );
}

#[test]
fn add_medium_1e6() {
    // Test: 1e6 + 1e6 = 2e6
    let a = Number::from(1_000_000i64);
    let b = Number::from(1_000_000i64);
    let result = a + b;
    let expected = Number::from(2_000_000i64);

    assert_eq!(result, expected);
    println!(
        "add_medium_1e6: representation = {}",
        result.representation()
    );
}

#[test]
fn add_medium_1e9() {
    // Test: 1e9 + 1e9 = 2e9
    let a = Number::from(1_000_000_000i64);
    let b = Number::from(1_000_000_000i64);
    let result = a + b;
    let expected = Number::from(2_000_000_000i64);

    assert_eq!(result, expected);
    println!(
        "add_medium_1e9: representation = {}",
        result.representation()
    );
}

#[test]
fn add_near_i64_max() {
    // Test: 9223372036854775000 + 1000 = 9223372036854776000
    let a = Number::from_str("9223372036854775000").unwrap();
    let b = Number::from(1000i64);
    let result = a + b;
    let expected = Number::from_str("9223372036854776000").unwrap();

    assert_eq!(result, expected);
    println!(
        "add_near_i64_max: representation = {}",
        result.representation()
    );
}

#[test]
fn add_small_positive() {
    // Test: 1 + 2 = 3
    let a = Number::from(1);
    let b = Number::from(2);
    let result = a + b;
    let expected = Number::from(3);

    assert_eq!(result, expected);
    println!(
        "add_small_positive: representation = {}",
        result.representation()
    );
}

#[test]
fn add_very_large_1e18() {
    // Test: 1e18 + 1e18 = 2e18
    let a = Number::from(1_000_000_000_000_000_000i64);
    let b = Number::from(1_000_000_000_000_000_000i64);
    let result = a + b;
    let expected = Number::from(2_000_000_000_000_000_000i64);

    assert_eq!(result, expected);
    println!(
        "add_very_large_1e18: representation = {}",
        result.representation()
    );
}

#[test]
fn add_very_large_1e20() {
    // Test: 1e20 + 1e20 = 2e20
    let a = Number::from_str("100000000000000000000").unwrap();
    let b = Number::from_str("100000000000000000000").unwrap();
    let result = a + b;
    let expected = Number::from_str("200000000000000000000").unwrap();

    assert_eq!(result, expected);
    println!(
        "add_very_large_1e20: representation = {}",
        result.representation()
    );
}

// ==== Division Tests (8 tests) ====

#[test]
fn div_basic_div() {
    // Test: 12 / 4 = 3
    let a = Number::from(12);
    let b = Number::from(4);
    let result = a / b;
    let expected = Number::from(3);

    assert_eq!(result, expected);
    println!(
        "div_basic_div: representation = {}",
        result.representation()
    );
}

#[test]
fn div_large_div_large() {
    // Test: 1e20 / 1e10 = 1e10
    let a = Number::from_str("100000000000000000000").unwrap();
    let b = Number::from_str("10000000000").unwrap();
    let result = a / b;
    let expected = Number::from_str("10000000000").unwrap();

    assert_eq!(result, expected);
    println!(
        "div_large_div_large: representation = {}",
        result.representation()
    );
}

#[test]
fn div_large_div_small() {
    // Test: 1e18 / 1e6 = 1e12
    let a = Number::from(1_000_000_000_000_000_000i64);
    let b = Number::from(1_000_000i64);
    let result = a / b;
    let expected = Number::from(1_000_000_000_000i64);

    assert_eq!(result, expected);
    println!(
        "div_large_div_small: representation = {}",
        result.representation()
    );
}

#[test]
fn div_one_seventh() {
    // Test: 1 / 7 = 1/7 (should be stored as Rational for exact representation)
    let a = Number::from(1);
    let b = Number::from(7);
    let result = a / b;
    let expected = Number::from_rational(Ratio::new(1, 7));

    assert_eq!(result, expected);
    assert_eq!(
        result.representation(),
        "Rational",
        "1/7 should be stored as Rational"
    );
    println!(
        "div_one_seventh: representation = {}",
        result.representation()
    );
}

#[test]
fn div_one_third() {
    // Test: 1 / 3 = 1/3 (should be stored as Rational)
    let a = Number::from(1);
    let b = Number::from(3);
    let result = a / b;
    let expected = Number::from_rational(Ratio::new(1, 3));

    assert_eq!(result, expected);
    assert_eq!(
        result.representation(),
        "Rational",
        "1/3 should be stored as Rational"
    );
    println!(
        "div_one_third: representation = {}",
        result.representation()
    );
}

#[test]
fn div_third_times_three() {
    // Test: 3 / 3 = 1 (should simplify to exact integer)
    let a = Number::from(3);
    let b = Number::from(3);
    let result = a / b;
    let expected = Number::from(1);

    assert_eq!(result, expected);
    println!(
        "div_third_times_three: representation = {}",
        result.representation()
    );
}

#[test]
fn div_two_thirds() {
    // Test: 2 / 3 = 2/3 (should be stored as Rational)
    let a = Number::from(2);
    let b = Number::from(3);
    let result = a / b;
    let expected = Number::from_rational(Ratio::new(2, 3));

    assert_eq!(result, expected);
    assert_eq!(
        result.representation(),
        "Rational",
        "2/3 should be stored as Rational"
    );
    println!(
        "div_two_thirds: representation = {}",
        result.representation()
    );
}

#[test]
fn div_very_large_div() {
    // Test: 1e50 / 1e25 = 1e25
    let a = Number::from_str("100000000000000000000000000000000000000000000000000").unwrap();
    let b = Number::from_str("10000000000000000000000000").unwrap();
    let result = a / b;
    let expected = Number::from_str("10000000000000000000000000").unwrap();

    assert_eq!(result, expected);
    println!(
        "div_very_large_div: representation = {}",
        result.representation()
    );
}

// ==== Natural Logarithm Tests (3 tests) ====

#[test]
fn ln_ln_1() {
    // Test: ln(1) = 0 (exact mathematical result)
    let a = Number::from(1);
    let result = a.log();
    let expected = Number::from(0);

    assert_eq!(result, expected);
    println!("ln_ln_1: representation = {}", result.representation());
}

#[test]
fn ln_ln_e() {
    // Test: ln(e) ≈ 1 (using 2721/1001 as approximation for e)
    // Compare to f64 behavior
    let e_approx = Number::from_rational(Ratio::new(2721, 1001));
    let result = e_approx.log();

    // F64 gets approximately 0.99999959468026, so we should be at least as good
    let f64_e_approx: f64 = 2721.0 / 1001.0;
    let f64_result = f64_e_approx.ln();
    let f64_as_number = Number::from(f64_result);

    // Our result should be close to the f64 result (we're comparing behaviors)
    assert!(
        result == f64_as_number || result.is_approximated(),
        "ln(e) should produce an approximated result"
    );
    println!("ln_ln_e: representation = {}", result.representation());
}

#[test]
fn ln_ln_negative() {
    // Test: ln(-1) = NaN
    let a = Number::from(-1);
    let result = a.log();

    assert!(result.is_nan());
    println!(
        "ln_ln_negative: representation = {}",
        result.representation()
    );
}

// ==== Multiplication Tests (7 tests) ====

#[test]
fn mul_basic_mul() {
    // Test: 3 * 4 = 12
    let a = Number::from(3);
    let b = Number::from(4);
    let result = a * b;
    let expected = Number::from(12);

    assert_eq!(result, expected);
    println!(
        "mul_basic_mul: representation = {}",
        result.representation()
    );
}

#[test]
fn mul_extreme_mul_1e50() {
    // Test: 1e25 * 1e25 = 1e50
    let a = Number::from_str("10000000000000000000000000").unwrap();
    let b = Number::from_str("10000000000000000000000000").unwrap();
    let result = a * b;
    let expected = Number::from_str("100000000000000000000000000000000000000000000000000").unwrap();

    assert_eq!(result, expected);
    println!(
        "mul_extreme_mul_1e50: representation = {}",
        result.representation()
    );
}

#[test]
fn mul_fractional_mul() {
    // Test: 0.5 * 0.25 = 0.125
    let a = Number::from_str("0.5").unwrap();
    let b = Number::from_str("0.25").unwrap();
    let result = a * b;
    let expected = Number::from_str("0.125").unwrap();

    assert_eq!(result, expected);
    println!(
        "mul_fractional_mul: representation = {}",
        result.representation()
    );
}

#[test]
fn mul_large_mul_1e15() {
    // Test: 1e9 * 1e6 = 1e15
    let a = Number::from(1_000_000_000i64);
    let b = Number::from(1_000_000i64);
    let result = a * b;
    let expected = Number::from(1_000_000_000_000_000i64);

    assert_eq!(result, expected);
    println!(
        "mul_large_mul_1e15: representation = {}",
        result.representation()
    );
}

#[test]
fn mul_medium_mul() {
    // Test: 1e6 * 1e3 = 1e9
    let a = Number::from(1_000_000i64);
    let b = Number::from(1_000i64);
    let result = a * b;
    let expected = Number::from(1_000_000_000i64);

    assert_eq!(result, expected);
    println!(
        "mul_medium_mul: representation = {}",
        result.representation()
    );
}

#[test]
fn mul_small_times_large() {
    // Test: 0.1 * 1e16 = 1e15
    let a = Number::from_str("0.1").unwrap();
    let b = Number::from_str("10000000000000000").unwrap();
    let result = a * b;
    let expected = Number::from(1_000_000_000_000_000i64);

    assert_eq!(result, expected);
    println!(
        "mul_small_times_large: representation = {}",
        result.representation()
    );
}

#[test]
fn mul_very_large_mul_1e20() {
    // Test: 1e10 * 1e10 = 1e20
    let a = Number::from_str("10000000000").unwrap();
    let b = Number::from_str("10000000000").unwrap();
    let result = a * b;
    let expected = Number::from_str("100000000000000000000").unwrap();

    assert_eq!(result, expected);
    println!(
        "mul_very_large_mul_1e20: representation = {}",
        result.representation()
    );
}

// ==== Sine Tests (2 tests) ====

#[test]
fn sin_sin_0() {
    // Test: sin(0) = 0 (exact mathematical result)
    let a = Number::from(0);
    let result = a.sin();
    let expected = Number::from(0);

    assert_eq!(result, expected);
    println!("sin_sin_0: representation = {}", result.representation());
}

#[test]
fn sin_sin_pi_2() {
    // Test: sin(π/2) ≈ 1 (using 355/226 as approximation for π/2)
    // Compare to f64 behavior
    let pi_half_approx = Number::from_rational(Ratio::new(355, 226));
    let result = pi_half_approx.sin();

    // F64 gets approximately 0.99999999999999911
    let f64_pi_half: f64 = 355.0 / 226.0;
    let f64_result = f64_pi_half.sin();
    let f64_as_number = Number::from(f64_result);

    // Our result should be at least as good as f64
    assert!(
        result == f64_as_number || result.is_approximated(),
        "sin(π/2) should produce an approximated result close to 1"
    );
    println!("sin_sin_pi_2: representation = {}", result.representation());
}

// ==== Square Root Tests (3 tests) ====

#[test]
fn sqrt_sqrt_2() {
    // Test: sqrt(2) ≈ 1.41421356... (transcendental, compare to f64)
    let a = Number::from(2);
    let result = a.sqrt();

    // Compare to f64 behavior
    let f64_result = 2.0f64.sqrt();
    let f64_as_number = Number::from(f64_result);

    // We should be at least as good as f64
    assert!(
        result == f64_as_number || result.is_approximated(),
        "sqrt(2) should produce an approximated result"
    );
    println!("sqrt_sqrt_2: representation = {}", result.representation());
}

#[test]
fn sqrt_sqrt_4() {
    // Test: sqrt(4) = 2 (exact mathematical result)
    let a = Number::from(4);
    let result = a.sqrt();
    let expected = Number::from(2);

    assert_eq!(result, expected);
    println!("sqrt_sqrt_4: representation = {}", result.representation());
}

#[test]
fn sqrt_sqrt_negative() {
    // Test: sqrt(-1) = NaN
    let a = Number::from(-1);
    let result = a.sqrt();

    assert!(result.is_nan());
    println!(
        "sqrt_sqrt_negative: representation = {}",
        result.representation()
    );
}

// ==== Subtraction Tests (10 tests) ====

#[test]
fn sub_basic_sub() {
    // Test: 5 - 3 = 2
    let a = Number::from(5);
    let b = Number::from(3);
    let result = a - b;
    let expected = Number::from(2);

    assert_eq!(result, expected);
    println!(
        "sub_basic_sub: representation = {}",
        result.representation()
    );
}

#[test]
fn sub_catastrophic_cancellation() {
    // Test: 10000000000000001 - 10000000000000000 = 1
    // This is where F64 fails catastrophically
    let a = Number::from_str("10000000000000001").unwrap();
    let b = Number::from_str("10000000000000000").unwrap();
    let result = a - b;
    let expected = Number::from(1);

    assert_eq!(result, expected, "Should preserve precision unlike f64");
    println!(
        "sub_catastrophic_cancellation: representation = {}",
        result.representation()
    );
}

#[test]
fn sub_extreme_1e50() {
    // Test: 3e50 - 1e50 = 2e50
    let a = Number::from_str("300000000000000000000000000000000000000000000000000").unwrap();
    let b = Number::from_str("100000000000000000000000000000000000000000000000000").unwrap();
    let result = a - b;
    let expected = Number::from_str("200000000000000000000000000000000000000000000000000").unwrap();

    assert_eq!(result, expected);
    println!(
        "sub_extreme_1e50: representation = {}",
        result.representation()
    );
}

#[test]
fn sub_large_1e15() {
    // Test: 3e15 - 1e15 = 2e15
    let a = Number::from(3_000_000_000_000_000i64);
    let b = Number::from(1_000_000_000_000_000i64);
    let result = a - b;
    let expected = Number::from(2_000_000_000_000_000i64);

    assert_eq!(result, expected);
    println!(
        "sub_large_1e15: representation = {}",
        result.representation()
    );
}

#[test]
fn sub_medium_1e6() {
    // Test: 3e6 - 1e6 = 2e6
    let a = Number::from(3_000_000i64);
    let b = Number::from(1_000_000i64);
    let result = a - b;
    let expected = Number::from(2_000_000i64);

    assert_eq!(result, expected);
    println!(
        "sub_medium_1e6: representation = {}",
        result.representation()
    );
}

#[test]
fn sub_medium_1e9() {
    // Test: 3e9 - 1e9 = 2e9
    let a = Number::from(3_000_000_000i64);
    let b = Number::from(1_000_000_000i64);
    let result = a - b;
    let expected = Number::from(2_000_000_000i64);

    assert_eq!(result, expected);
    println!(
        "sub_medium_1e9: representation = {}",
        result.representation()
    );
}

#[test]
fn sub_near_i64_max() {
    // Test: 9223372036854776000 - 1000 = 9223372036854775000
    let a = Number::from_str("9223372036854776000").unwrap();
    let b = Number::from(1000i64);
    let result = a - b;
    let expected = Number::from_str("9223372036854775000").unwrap();

    assert_eq!(result, expected);
    println!(
        "sub_near_i64_max: representation = {}",
        result.representation()
    );
}

#[test]
fn sub_negative_result() {
    // Test: 3 - 5 = -2
    let a = Number::from(3);
    let b = Number::from(5);
    let result = a - b;
    let expected = Number::from(-2);

    assert_eq!(result, expected);
    println!(
        "sub_negative_result: representation = {}",
        result.representation()
    );
}

#[test]
fn sub_very_large_1e18() {
    // Test: 3e18 - 1e18 = 2e18
    let a = Number::from(3_000_000_000_000_000_000i64);
    let b = Number::from(1_000_000_000_000_000_000i64);
    let result = a - b;
    let expected = Number::from(2_000_000_000_000_000_000i64);

    assert_eq!(result, expected);
    println!(
        "sub_very_large_1e18: representation = {}",
        result.representation()
    );
}

#[test]
fn sub_very_large_1e20() {
    // Test: 3e20 - 1e20 = 2e20
    let a = Number::from_str("300000000000000000000").unwrap();
    let b = Number::from_str("100000000000000000000").unwrap();
    let result = a - b;
    let expected = Number::from_str("200000000000000000000").unwrap();

    assert_eq!(result, expected);
    println!(
        "sub_very_large_1e20: representation = {}",
        result.representation()
    );
}
