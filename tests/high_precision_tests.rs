#![cfg(feature = "high_precision")]

use faithful_number::Number;
use std::str::FromStr;

#[test]
fn test_precision_control() {
    // Test default precision (256 bits = ~71 decimal digits)
    let default_prec = Number::get_default_precision();
    assert_eq!(default_prec, 256);

    // Test setting precision
    Number::set_default_precision(200);
    assert_eq!(Number::get_default_precision(), 200);

    // Restore default
    Number::set_default_precision(256);
}

#[test]
fn test_high_precision_sqrt() {
    Number::set_default_precision(200);

    let n = Number::from(2);
    let sqrt2 = n.sqrt();

    // High precision sqrt(2) should be accurate to many digits
    // sqrt(2) = 1.41421356237309504880168872420969807856967187537694...
    let result_str = sqrt2.to_f64();
    let expected = 2.0_f64.sqrt();

    // Should be very close (within f64 precision)
    assert!((result_str - expected).abs() < 1e-15);

    // Verify it's stored as BigDecimal (high precision representation)
    assert_eq!(sqrt2.representation(), "BigDecimal");
}

#[test]
fn test_high_precision_log() {
    Number::set_default_precision(150);

    let n = Number::from(2);
    let ln2 = n.log();

    // ln(2) = 0.693147180559945309417232121458176568075500134360255...
    let result = ln2.to_f64();
    let expected = 2.0_f64.ln();

    assert!((result - expected).abs() < 1e-15);
    assert_eq!(ln2.representation(), "BigDecimal");
}

#[test]
fn test_high_precision_exp() {
    Number::set_default_precision(150);

    let n = Number::from(1);
    let e = n.exp();

    // e^1 = 2.718281828459045235360287471352662497757247093699...
    let result = e.to_f64();
    let expected = 1.0_f64.exp();

    assert!((result - expected).abs() < 1e-15);
    assert_eq!(e.representation(), "BigDecimal");
}

#[test]
fn test_high_precision_sin() {
    Number::set_default_precision(150);

    // Test sin(π/6) = 0.5 exactly
    let pi_over_6 = Number::from_decimal(
        rust_decimal::Decimal::from_str("0.5235987755982988730771072305465838140328615665625")
            .unwrap(),
    );
    let result = pi_over_6.sin();

    let result_f64 = result.to_f64();
    assert!((result_f64 - 0.5).abs() < 1e-10);
    assert_eq!(result.representation(), "BigDecimal");
}

#[test]
fn test_high_precision_cos() {
    Number::set_default_precision(150);

    // Test cos(π/3) = 0.5 exactly
    let pi_over_3 = Number::from_decimal(
        rust_decimal::Decimal::from_str("1.0471975511965977461542144610931676280657231331250")
            .unwrap(),
    );
    let result = pi_over_3.cos();

    let result_f64 = result.to_f64();
    assert!((result_f64 - 0.5).abs() < 1e-10);
    assert_eq!(result.representation(), "BigDecimal");
}

#[test]
fn test_high_precision_tan() {
    Number::set_default_precision(150);

    // Test tan(π/4) = 1 exactly
    let pi_over_4 = Number::from_decimal(
        rust_decimal::Decimal::from_str("0.7853981633974483096156608458198757210492923498437")
            .unwrap(),
    );
    let result = pi_over_4.tan();

    let result_f64 = result.to_f64();
    assert!((result_f64 - 1.0).abs() < 1e-10);
    assert_eq!(result.representation(), "BigDecimal");
}

#[test]
fn test_high_precision_asin() {
    Number::set_default_precision(150);

    let half = Number::from_decimal(rust_decimal::Decimal::from_str("0.5").unwrap());
    let result = half.asin();

    // asin(0.5) = π/6 ≈ 0.5235987755982988...
    let result_f64 = result.to_f64();
    let expected = 0.5_f64.asin();

    assert!((result_f64 - expected).abs() < 1e-15);
    assert_eq!(result.representation(), "BigDecimal");
}

#[test]
fn test_high_precision_acos() {
    Number::set_default_precision(150);

    let half = Number::from_decimal(rust_decimal::Decimal::from_str("0.5").unwrap());
    let result = half.acos();

    // acos(0.5) = π/3 ≈ 1.0471975511965979...
    let result_f64 = result.to_f64();
    let expected = 0.5_f64.acos();

    assert!((result_f64 - expected).abs() < 1e-15);
    assert_eq!(result.representation(), "BigDecimal");
}

#[test]
fn test_high_precision_atan() {
    Number::set_default_precision(150);

    let one = Number::from(1);
    let result = one.atan();

    // atan(1) = π/4 ≈ 0.7853981633974483...
    let result_f64 = result.to_f64();
    let expected = 1.0_f64.atan();

    assert!((result_f64 - expected).abs() < 1e-15);
    assert_eq!(result.representation(), "BigDecimal");
}

#[test]
fn test_high_precision_atan2() {
    Number::set_default_precision(150);

    let one = Number::from(1);
    let two = Number::from(2);
    let result = one.atan2(two);

    // atan2(1, 2) ≈ 0.4636476090008061...
    let result_f64 = result.to_f64();
    let expected = 1.0_f64.atan2(2.0_f64);

    assert!((result_f64 - expected).abs() < 1e-15);
    assert_eq!(result.representation(), "BigDecimal");
}

#[test]
fn test_high_precision_pow() {
    Number::set_default_precision(150);

    let two = Number::from(2);
    let half = Number::from_decimal(rust_decimal::Decimal::from_str("0.5").unwrap());
    let result = two.pow(half);

    // 2^0.5 = sqrt(2) ≈ 1.4142135623730951...
    let result_f64 = result.to_f64();
    let expected = 2.0_f64.powf(0.5);

    assert!((result_f64 - expected).abs() < 1e-15);
    assert_eq!(result.representation(), "BigDecimal");
}

#[test]
fn test_high_precision_log10() {
    Number::set_default_precision(150);

    let ten = Number::from(10);
    let log10_10 = ten.log10();

    // log10(10) = 1 exactly
    let result = log10_10.to_f64();
    assert!((result - 1.0).abs() < 1e-15);
    assert_eq!(log10_10.representation(), "BigDecimal");
}

#[test]
fn test_high_precision_log2() {
    Number::set_default_precision(150);

    let eight = Number::from(8);
    let log2_8 = eight.log2();

    // log2(8) = 3 exactly
    let result = log2_8.to_f64();
    assert!((result - 3.0).abs() < 1e-15);
    assert_eq!(log2_8.representation(), "BigDecimal");
}

#[test]
fn test_different_precision_levels() {
    // Test with low precision
    Number::set_default_precision(50);
    let sqrt2_low = Number::from(2).sqrt();
    assert_eq!(sqrt2_low.representation(), "BigDecimal");

    // Test with high precision
    Number::set_default_precision(300);
    let sqrt2_high = Number::from(2).sqrt();
    assert_eq!(sqrt2_high.representation(), "BigDecimal");

    // Both should be close to the correct value
    let low_result = sqrt2_low.to_f64();
    let high_result = sqrt2_high.to_f64();
    let expected = 2.0_f64.sqrt();

    assert!((low_result - expected).abs() < 1e-10);
    assert!((high_result - expected).abs() < 1e-10);
}

#[test]
fn test_chained_operations() {
    Number::set_default_precision(150);

    // Test chained transcendental operations
    // exp(log(x)) should equal x
    let x = Number::from(5);
    let result = x.log().exp();

    let result_f64 = result.to_f64();
    assert!((result_f64 - 5.0).abs() < 1e-10);
}

#[test]
fn test_special_values_high_precision() {
    Number::set_default_precision(150);

    // Test that special values still work correctly
    assert!(Number::from(0).log().is_negative_infinity());
    assert!(Number::from(-1).sqrt().is_nan());
    assert!(Number::POSITIVE_INFINITY.exp().is_positive_infinity());
}
