//! Adversarial tests for string parsing and display.

use faithful_number::Number;
use std::str::FromStr;

#[test]
fn parse_0_1_is_exact() {
    // 0.1 should parse to exact Decimal, not lossy f64
    let tenth = Number::from_str("0.1").unwrap();

    // Should be exact
    assert!(tenth.is_exact());

    // 0.1 + 0.2 should equal 0.3 exactly
    let two_tenths = Number::from_str("0.2").unwrap();
    let three_tenths = Number::from_str("0.3").unwrap();

    assert_eq!(tenth + two_tenths, three_tenths);
}

#[test]
fn roundtrip_integers() {
    let values = vec![0i64, 1, -1, 42, -42, i64::MAX, i64::MIN];

    for v in values {
        let n = Number::from(v);
        let s = n.to_string();
        let parsed = Number::from_str(&s).unwrap();
        assert_eq!(n, parsed, "Roundtrip failed for {}", v);
    }
}

#[test]
fn roundtrip_decimals() {
    let strings = vec!["0.1", "0.5", "123.456", "-99.99", "0.125"];

    for s in strings {
        let n = Number::from_str(s).unwrap();
        let back = n.to_string();
        let parsed = Number::from_str(&back).unwrap();
        assert_eq!(n, parsed, "Roundtrip failed for {}", s);
    }
}

#[test]
fn parse_scientific_notation() {
    // Large number in scientific notation
    let large = Number::from_str("1e20").unwrap();
    assert!(large.to_f64() > 1e19);

    // Small number
    let small = Number::from_str("1e-20").unwrap();
    assert!(small.to_f64() < 1e-19);
}

#[test]
fn parse_negative_zero() {
    let neg_zero = Number::from_str("-0").unwrap();

    // Should be negative zero
    assert!(neg_zero.is_neg_zero());
}

#[test]
fn parse_nan() {
    let nan = Number::from_str("NaN").unwrap();
    assert!(nan.is_nan());
}

#[test]
fn parse_infinity() {
    let pos_inf = Number::from_str("Infinity").unwrap();
    let neg_inf = Number::from_str("-Infinity").unwrap();

    assert!(pos_inf.is_infinite());
    assert!(neg_inf.is_infinite());
    assert!(pos_inf.to_f64() > 0.0);
    assert!(neg_inf.to_f64() < 0.0);
}

#[test]
fn parse_very_large_integer() {
    // Number exceeding i64
    let huge = Number::from_str("9999999999999999999999").unwrap();

    assert!(!huge.is_nan());
    assert!(huge.to_f64() > 1e21);
}

#[test]
fn parse_invalid_returns_error() {
    assert!(Number::from_str("not a number").is_err());
    assert!(Number::from_str("12.34.56").is_err());
    assert!(Number::from_str("abc123").is_err());
}

#[test]
#[cfg(not(feature = "js_string_parse"))]
fn empty_string_is_error_default() {
    assert!(Number::from_str("").is_err());
}

#[test]
#[cfg(feature = "js_string_parse")]
fn empty_string_is_zero_js() {
    assert_eq!(Number::from_str("").unwrap(), Number::ZERO);
}

#[test]
fn display_special_values() {
    assert_eq!(Number::nan().to_string(), "NaN");
    assert_eq!(Number::infinity().to_string(), "Infinity");
    assert_eq!(Number::neg_infinity().to_string(), "-Infinity");
    // -0 displays as "0" per convention
    assert_eq!(Number::neg_zero().to_string(), "0");
}
