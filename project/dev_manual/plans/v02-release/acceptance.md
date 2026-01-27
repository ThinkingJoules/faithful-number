# Acceptance Criteria

## Primary Test

The implementation is complete when these commands succeed:

```bash
cargo test                       # Pure mode (default)
cargo test --features js_compat  # JS compatibility mode
cargo test --all-features        # All features combined
```

All three must pass.

---

## Phase 1: Documentation

- [ ] `project/user_manual/VISION.md` exists
- [ ] `README.md` has feature flag documentation
- [ ] `README.md` has migration guide from v0.1

---

## Phase 2: Breaking Change Audit

Audit complete when documented:
- [ ] Number of `assert_eq!` usages that need updating
- [ ] Number of HashMap/HashSet usages with Number keys
- [ ] Migration path documented in README

---

## Phase 3: Feature Flags & OrderedNumber

### Default Mode (Pure)

```bash
cargo test --test feature_defaults
```

Test file: `tests/feature_defaults.rs`

```rust
#[test]
fn nan_not_equal_to_nan() {
    let nan1 = Number::nan();
    let nan2 = Number::nan();
    assert!(nan1 != nan2, "IEEE 754: NaN != NaN");
}

#[test]
fn number_does_not_impl_eq() {
    // This is a compile-time check - Number should not impl Eq
    // fn requires_eq<T: Eq>(_: T) {}
    // requires_eq(Number::from(1)); // Should not compile
}

#[test]
fn empty_string_parse_fails() {
    use std::str::FromStr;
    assert!(Number::from_str("").is_err());
}

#[test]
fn bitwise_traits_not_available() {
    // Without js_bitwise, BitAnd etc. should not be implemented
    // let a = Number::from(5);
    // let b = Number::from(3);
    // let _ = a & b;  // Should not compile
}
```

### OrderedNumber for Collections

```bash
cargo test --test ordered_number
```

```rust
use std::collections::HashMap;
use faithful_number::OrderedNumber;

#[test]
fn ordered_number_in_hashmap() {
    let mut map: HashMap<OrderedNumber, i32> = HashMap::new();
    map.insert(OrderedNumber::from(Number::from(1)), 100);
    map.insert(OrderedNumber::from(Number::nan()), 200);

    assert_eq!(map.get(&OrderedNumber::from(Number::from(1))), Some(&100));
    assert_eq!(map.get(&OrderedNumber::from(Number::nan())), Some(&200));
}

#[test]
fn ordered_number_nan_equals_nan() {
    let nan1 = OrderedNumber::from(Number::nan());
    let nan2 = OrderedNumber::from(Number::nan());
    assert_eq!(nan1, nan2); // For collection use
}
```

### JS Compat Mode

```bash
cargo test --test feature_js_compat --features js_compat
```

```rust
#[test]
#[cfg(feature = "js_nan_equality")]
fn nan_equals_nan_with_feature() {
    let nan1 = Number::nan();
    let nan2 = Number::nan();
    assert_eq!(nan1, nan2);
}

#[test]
#[cfg(feature = "js_bitwise")]
fn bitwise_with_implicit_coercion() {
    let a = Number::from(5.7);
    let b = Number::from(3);
    let result = a & b;
    assert_eq!(result, Number::from(1)); // 5 & 3 = 1
}

#[test]
#[cfg(feature = "js_string_parse")]
fn empty_string_parses_to_zero() {
    use std::str::FromStr;
    assert_eq!(Number::from_str("").unwrap(), Number::ZERO);
}
```

---

## Phase 4: No Panics

```bash
! grep -r "unimplemented!" src/
! grep -r "todo!" src/
```

Both greps must find nothing (exit code 1).

Specific implementations required:
- [ ] `BigDecimal::to_i32()` - truncate and wrap
- [ ] `BigDecimal::to_i64()` - truncate and wrap
- [ ] `Hash for Rational` - implemented
- [ ] `Hash for BigDecimal` - implemented
- [ ] `round_dp for BigDecimal` - implemented
- [ ] `trunc for BigDecimal` - implemented

---

## Phase 5: Adversarial Tests

All new test files must exist and pass:

```bash
cargo test --test bitwise
cargo test --test overflow_boundaries
cargo test --test continued_fractions
cargo test --test string_parsing
cargo test --test associativity
cargo test --test negative_zero
cargo test --test hash_consistency
cargo test --test ordered_number
```

### Key Adversarial Cases

**Overflow Boundaries** (`tests/overflow_boundaries.rs`):
- [ ] `Rational(i64::MAX - 1, 1) + Rational(2, 1)` promotes to BigDecimal
- [ ] `Rational(1, i64::MAX) + Rational(1, i64::MAX)` handles denominator overflow
- [ ] Division creating huge denominators doesn't panic

**Continued Fractions** (`tests/continued_fractions.rs`):
- [ ] `0.999999999999999999999999999` does NOT become `1`
- [ ] `0.142857142857142857142857142857` finds `1/7`
- [ ] Values at CF algorithm boundary don't infinite loop

**Hash Consistency** (`tests/hash_consistency.rs`):
- [ ] `Rational(1, 2)` and equivalent `Decimal(0.5)` have same hash
- [ ] Equal `Number` values have equal hashes regardless of representation

**String Parsing** (`tests/string_parsing.rs`):
- [ ] `Number::from_str("0.1")` is exact (not f64 lossy)
- [ ] Roundtrip: `from_str(n.to_string()) == n` for various values
- [ ] `"-0"` parses to NegativeZero

---

## Phase 6: Release Polish

- [ ] `cargo doc --all-features` builds without warnings
- [ ] `cargo clippy --all-features` has no warnings
- [ ] `CHANGELOG.md` exists with breaking changes section
- [ ] `Cargo.toml` version is `0.2.0`
- [ ] All 43 benchmark tests still pass

---

## Summary Checklist

```bash
# All must pass
cargo test
cargo test --features js_compat
cargo test --all-features
cargo clippy --all-features
cargo doc --all-features

# Must find nothing
grep -r "unimplemented!" src/ && exit 1
grep -r "todo!" src/ && exit 1

# Version check
grep 'version = "0.2.0"' Cargo.toml
```
