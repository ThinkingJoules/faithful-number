# Acceptance Criteria

## Primary Test

The implementation is complete when this command succeeds:

```bash
cargo test --all-features serialization_roundtrip
```

This test verifies the key invariant: numbers survive serialization round-trips.

---

## Phase 1: Format Display

```bash
cargo test --features format format_display
```

### Functional Criteria

- [ ] `DisplayOptions::default()` produces same output as current `Display`
- [ ] US format: `1234567.89` → `"1,234,567.89"`
- [ ] European format: `1234567.89` → `"1.234.567,89"`
- [ ] SI format: `1234567.89` → `"1 234 567.89"`
- [ ] Indian format: `12345678.9` → `"1,23,45,678.9"`
- [ ] Scientific: `1234567` → `"1.234567e6"`
- [ ] Engineering: `1234567` → `"1.234567e6"` (powers of 1000)
- [ ] Special values format correctly: NaN, Infinity, -Infinity, -0

---

## Phase 2: Format Parsing

```bash
cargo test --features format format_parse
```

### Functional Criteria

- [ ] Round-trip: `parse_formatted(format(n, opts), opts) == n` for all regional formats
- [ ] Scientific notation parses: `"1.23e6"` → `1230000`
- [ ] Regional separators handled: `"1.234,56"` with European opts → `1234.56`
- [ ] Error on malformed input (multiple decimal points, etc.)

### Error Handling

- [ ] `ParseError::EmptyInput` for `""`
- [ ] `ParseError::InvalidCharacter` for `"12abc34"`
- [ ] `ParseError::MultipleSeparators` for `"1..5"` or `"1,,234"`
- [ ] `ParseError::MismatchedFormat` when input doesn't match regional format
- [ ] `ParseError::Overflow` for numbers exceeding BigDecimal capacity

---

## Phase 3: serde_str

```bash
cargo test --features serde_str serde_str
```

### Functional Criteria

- [ ] Exact number serializes as single-element array: `["0.5"]`
- [ ] Approximate number includes type: `["1.414...", "transcendental"]`
- [ ] Deserialize reconstructs Number with correct ApproximationType
- [ ] Round-trip through serde_json preserves value and approximation
- [ ] Works with nested structures (Vec<Number>, HashMap with Number values)

### Test Code

```rust
#[test]
fn serde_str_roundtrip() {
    let exact = Number::from(0.5);
    let json = serde_json::to_string(&exact).unwrap();
    assert_eq!(json, r#"["0.5"]"#);
    let back: Number = serde_json::from_str(&json).unwrap();
    assert_eq!(exact, back);
    assert!(back.is_exact());

    let approx = Number::from(2.0).sqrt();
    let json = serde_json::to_string(&approx).unwrap();
    // lowercase: "transcendental"
    assert!(json.contains(r#""transcendental""#));
    let back: Number = serde_json::from_str(&json).unwrap();
    assert!(!back.is_exact());
}
```

---

## Phase 4: serde_bin

```bash
cargo test --features serde_bin serde_bin
```

### Functional Criteria

- [ ] Binary encoding uses onenum for value
- [ ] Approximation type encoded as **suffix** byte (preserves sort order)
- [ ] Round-trip through bincode preserves value and approximation
- [ ] Encoded bytes are lexicographically sortable by numeric value
- [ ] Special values (NaN, Infinity) encode/decode correctly

### Test Code

```rust
#[test]
fn serde_bin_roundtrip() {
    let nums = vec![
        Number::from(0),
        Number::from(-1),
        Number::from(1.5),
        Number::from(2.0).sqrt(),
        Number::POSITIVE_INFINITY,
        Number::NAN,
    ];

    for n in nums {
        let bytes = bincode::serialize(&n).unwrap();
        let back: Number = bincode::deserialize(&bytes).unwrap();
        // NaN != NaN, so check specially
        if n.is_nan() {
            assert!(back.is_nan());
        } else {
            assert_eq!(n, back);
        }
    }
}

#[test]
fn serde_bin_sortable() {
    // Test that onenum bytes (excluding approx suffix) sort correctly
    let mut nums: Vec<Number> = vec![
        Number::from(100),
        Number::from(-5),
        Number::from(0),
        Number::from(3.14),
    ];

    let mut encoded: Vec<Vec<u8>> = nums.iter()
        .map(|n| {
            let bytes = bincode::serialize(n).unwrap();
            // Strip the approx suffix byte for sorting
            bytes[..bytes.len()-1].to_vec()
        })
        .collect();

    nums.sort_by(|a, b| a.partial_cmp(b).unwrap());
    encoded.sort();

    // Verify order matches
    for (i, bytes) in encoded.iter().enumerate() {
        let full_bytes = bincode::serialize(&nums[i]).unwrap();
        assert_eq!(bytes, &full_bytes[..full_bytes.len()-1]);
    }
}
```

---

## Phase 5: Documentation

```bash
cargo doc --all-features --no-deps
```

### Criteria

- [ ] All public items have doc comments
- [ ] Feature flags documented in crate-level docs
- [ ] Examples compile and run
- [ ] README updated with serialization section

---

## Structural Criteria

- [ ] `serde_str` and `serde_bin` are mutually exclusive via cfg:
  - `#[cfg(all(feature = "serde_str", not(feature = "serde_bin")))]`
  - `#[cfg(all(feature = "serde_bin", not(feature = "serde_str")))]`
  - If both enabled, neither impl compiles (Number won't have Serialize)
- [ ] No new clippy warnings with `cargo clippy --all-features`
- [ ] `cargo test` (no features) still passes - no regressions

## Quality Criteria

- [ ] All existing tests pass
- [ ] New tests cover happy path and error cases
- [ ] Feature-gated code properly isolated
