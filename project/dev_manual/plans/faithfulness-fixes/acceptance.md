# Acceptance Criteria

## Primary Test

The implementation is complete when this command succeeds:

```bash
cargo test --test faithfulness_acceptance
```

Test file: `tests/faithfulness_acceptance.rs` (created in Phase 0)

This test validates all faithfulness guarantees:

```rust
//! Faithfulness acceptance tests
//!
//! These tests verify that faithful-number lives up to its name:
//! no silent precision loss, consistent behavior across representations.

use faithful_number::{Number, ApproximationType};
use std::str::FromStr;
use std::collections::HashMap;

// ============ Phase 1: Exact Rational Modulo ============

#[test]
fn modulo_rational_is_exact() {
    // (1/3) % (1/7) should be exact, not approximated
    let a = Number::from(1) / Number::from(3);
    let b = Number::from(1) / Number::from(7);
    let result = a % b;

    assert!(result.is_exact(), "Rational modulo must be exact");
    assert_eq!(result.representation(), "Rational");
}

#[test]
fn modulo_result_mathematically_correct() {
    // (5/6) % (1/4) = (5/6) - floor((5/6)/(1/4)) * (1/4)
    //              = (5/6) - floor(10/3) * (1/4)
    //              = (5/6) - 3 * (1/4)
    //              = (5/6) - (3/4)
    //              = (10/12) - (9/12) = 1/12
    let a = Number::from(5) / Number::from(6);
    let b = Number::from(1) / Number::from(4);
    let result = a % b;
    let expected = Number::from(1) / Number::from(12);

    assert_eq!(result, expected);
}

// ============ Phase 2: String Parsing Recovers Rationals ============

#[test]
fn parse_terminating_decimal_becomes_rational() {
    let n: Number = "0.5".parse().unwrap();
    assert_eq!(n.representation(), "Rational");
    assert_eq!(n, Number::from(1) / Number::from(2));
}

#[test]
fn parse_0_125_becomes_rational() {
    let n: Number = "0.125".parse().unwrap();
    assert_eq!(n.representation(), "Rational");
    assert_eq!(n, Number::from(1) / Number::from(8));
}

#[test]
fn parse_integer_becomes_rational() {
    let n: Number = "42".parse().unwrap();
    assert_eq!(n.representation(), "Rational");
}

// ============ Phase 3: Exact floor/ceil/round ============

#[test]
fn floor_large_rational_no_f64_overflow() {
    // Value larger than f64 mantissa can represent exactly
    // 2^60 + 1/2 should floor to 2^60
    let big = Number::from(1i64 << 60);
    let half = Number::from(1) / Number::from(2);
    let n = big.clone() + half;

    assert_eq!(n.floor(), big);
}

#[test]
fn ceil_large_rational_no_f64_overflow() {
    let big = Number::from(1i64 << 60);
    let half = Number::from(1) / Number::from(2);
    let n = big.clone() + half;

    assert_eq!(n.ceil(), big + Number::from(1));
}

#[test]
fn round_exact_half_away_from_zero() {
    // 3/2 = 1.5 should round to 2 (away from zero)
    let n = Number::from(3) / Number::from(2);
    assert_eq!(n.round(), Number::from(2));

    // -3/2 = -1.5 should round to -2 (away from zero)
    let n = Number::from(-3) / Number::from(2);
    assert_eq!(n.round(), Number::from(-2));
}

// ============ Phase 4: Exact to_decimal() ============

#[test]
fn to_decimal_terminating_rational_exact() {
    let r = Number::from(1) / Number::from(4); // 0.25
    let d = r.to_decimal().unwrap();
    assert_eq!(d.to_string(), "0.25");
}

#[test]
fn to_decimal_non_terminating_returns_none() {
    let r = Number::from(1) / Number::from(3); // 0.333...
    // Non-terminating rationals cannot be exactly represented as Decimal
    assert!(r.to_decimal().is_none());
}

// ============ Phase 5: Consistent Constants ============

#[test]
fn zero_constant_is_rational() {
    assert_eq!(Number::ZERO.representation(), "Rational");
}

#[test]
fn one_constant_is_rational() {
    assert_eq!(Number::ONE.representation(), "Rational");
}

#[test]
fn zero_constant_equals_from_zero() {
    assert_eq!(Number::ZERO, Number::from(0));
    // Same representation too
    assert_eq!(Number::ZERO.representation(), Number::from(0).representation());
}

// ============ Phase 6: Consistent Hashing ============

#[test]
fn equal_values_hash_equal() {
    use std::hash::{Hash, Hasher};
    use std::collections::hash_map::DefaultHasher;

    fn hash_of<T: Hash>(t: &T) -> u64 {
        let mut s = DefaultHasher::new();
        t.hash(&mut s);
        s.finish()
    }

    // Rational(1, 2) and Decimal(0.5) are equal
    let r = Number::from(1) / Number::from(2);
    let d: Number = "0.5".parse().unwrap();

    // After Phase 2, d should also be Rational, but let's test the principle
    assert_eq!(r, d);
    assert_eq!(hash_of(&r), hash_of(&d), "Equal values must hash equal");
}

#[test]
fn hashmap_works_across_representations() {
    let mut map = HashMap::new();

    let key = Number::from(1) / Number::from(2);
    map.insert(key.clone(), "half");

    // Should find it regardless of how we construct 0.5
    let lookup = Number::from(1) / Number::from(2);
    assert_eq!(map.get(&lookup), Some(&"half"));
}

// ============ Phase 7: BigDecimal Division Precision ============

#[test]
fn bigdecimal_division_flags_truncation() {
    // Create BigDecimals that would have repeating decimal division
    // 1e30 / 3 = 0.333...e30 (repeating)
    let big = Number::try_from_i128_with_scale(1_000_000_000_000_000_000_000_000_000_000i128, 0).unwrap();
    let three = Number::from(3);
    let result = big / three;

    // Should be flagged as approximation since result is non-terminating
    assert!(result.is_rational_approximation() || result.representation() == "Rational",
            "Non-terminating BigDecimal division should be flagged or exact Rational");
}

// ============ Phase 9: Introspection API ============

#[test]
fn number_info_shows_representation() {
    let n = Number::from(1) / Number::from(3);
    let info = n.info();

    assert_eq!(info.representation, "Rational");
    assert!(info.is_exact);
}

#[test]
fn number_info_shows_approximation() {
    let n = Number::from(2).sqrt();
    let info = n.info();

    assert!(!info.is_exact);
    assert_eq!(info.approximation_type, Some(ApproximationType::Transcendental));
}

#[test]
fn number_info_display_is_readable() {
    let n = Number::from(2).sqrt();
    let info = n.info();
    let display = format!("{}", info);

    // Should contain useful information
    assert!(display.contains("Transcendental") || display.contains("approximate"));
}
```

---

## Functional Criteria

- [ ] `(a/b) % (c/d)` returns exact Rational when possible
- [ ] Parsing terminating decimal strings produces Rational representation
- [ ] `floor`/`ceil`/`round` work without f64 conversion for Rational/Decimal
- [ ] `to_decimal()` returns `None` for non-terminating rationals
- [ ] `Number::ZERO` and `Number::ONE` have Rational representation
- [ ] Equal Numbers always hash equal, regardless of internal representation
- [ ] BigDecimal division with repeating result sets RationalApproximation flag
- [ ] `Number::info()` returns complete state information

## Structural Criteria

- [ ] Decision document exists at `project/dev_manual/decisions/cf-denominator-limit.md`
- [ ] NumberInfo struct is public and well-documented
- [ ] No f64 conversions in rounding operations for Rational type

## Quality Criteria

- [ ] `cargo test` passes (no regressions)
- [ ] `cargo clippy` has no new warnings
- [ ] Existing benchmarks don't regress more than 20%
