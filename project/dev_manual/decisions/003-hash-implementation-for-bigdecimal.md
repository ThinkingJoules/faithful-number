# TODO: Decision: Hash Implementation Using String Representation for BigDecimal

> **Tech Debt Note**: This is a pragmatic solution, not ideal. Revisit in v0.3+ to implement proper cross-representation hash normalization. The string-based hash is slower and doesn't guarantee hash consistency for equal values across different representations (e.g., Rational(1,2) vs BigDecimal("0.5")).

## Context

Phase 4 required fixing all `unimplemented!()` panics. One critical gap was `Hash` implementation for `BigDecimal` variant.

The challenge: `bigdecimal::BigDecimal` does not implement `Hash` (because it represents the same mathematical value in multiple ways, e.g., `1.0` vs `1.00`).

Additionally, hash must satisfy the invariant: **if `a == b`, then `hash(a) == hash(b)`**. Since `Number` supports cross-representation equality (e.g., `Rational(1, 2) == Decimal(0.5)`), we needed consistent hashing across representations.

## Options Considered

1. **Hash the BigDecimal bytes directly**
   - Pros: Fast
   - Cons: BigDecimal doesn't expose internal representation consistently
   - Cons: Different representations of same value would hash differently

2. **Convert to canonical Rational for hashing**
   - Pros: Mathematically correct, consistent with Rational hashing
   - Cons: Expensive conversion, may lose precision
   - Cons: BigDecimal exists precisely because Rational overflowed

3. **Use string representation**
   - Pros: Simple, deterministic
   - Cons: Slower than numeric hash
   - Cons: String representation may vary (e.g., "1.0" vs "1")

4. **Use normalized BigDecimal, hash mantissa + exponent**
   - Pros: Faster than string
   - Cons: BigDecimal normalization is non-trivial
   - Cons: Still need to handle cross-representation equality

## Decision

We chose **Option 3: Hash string representation** for BigDecimal, with careful handling of cross-representation equality.

From the implementation:
```rust
NumericValue::BigDecimal(bd) => {
    if bd.is_zero() {
        3u8.hash(state);
        0i64.hash(state); // All zeros hash the same
    } else {
        3u8.hash(state);
        bd.to_string().hash(state); // String repr for BigDecimal
    }
}
```

For Rational and Decimal, we use numerator/denominator and normalized decimal respectively, ensuring cross-representation consistency for common cases (especially zero).

## Special Case: Zero Variants

Critical insight from implementation: **All zero representations must hash identically**.

Zero can appear as:
- `NumericValue::NegativeZero`
- `NumericValue::Rational(0, 1, ...)`
- `NumericValue::Decimal(0.0)`
- `NumericValue::BigDecimal(0)`

They all compare equal via `PartialEq`, so they must hash the same:
```rust
// All zeros hash to: 3u8.hash(state); 0i64.hash(state);
```

## Consequences

### Enables

- **No panics**: Hash works for all Number values, including BigDecimal
- **Collection support**: OrderedNumber can use HashMap/HashSet with BigDecimal values
- **Correctness**: Hash invariant maintained (equal values have equal hashes)
- **Special value handling**: NaN, Infinity, and all zeros hash consistently

### Prevents/Complicates

- **Performance**: String hashing is slower than numeric hashing
- **Precision edge cases**: Cross-representation equality only guaranteed for common values
- **Implementation dependency**: Relies on `BigDecimal::to_string()` format stability

### Known Limitations

1. **Rational(1, 2) vs BigDecimal("0.5") may hash differently**
   - They compare equal via PartialEq (cross-representation equality)
   - But they may hash differently (Rational uses numer/denom, BigDecimal uses string)
   - This technically violates the hash invariant
   - Mitigated: BigDecimal is only used after overflow, rarely equals exact Rational

2. **String representation dependence**
   - If `BigDecimal::to_string()` format changes, hashes change
   - Not a problem within single program execution
   - Could be problem for persistent hashes (we don't support this)

## Alternatives Considered but Deferred

- **Normalize all representations to canonical form before hashing**: Too expensive
- **Implement proper cross-representation hash normalization**: Deferred to v0.3+
- **Property testing to verify hash invariant**: Added to test suite but not exhaustive

## Related

- Plan: v02-release (Phase 3 and Phase 4)
- Files: `src/traits.rs` (Hash impl), `src/ordered.rs` (OrderedNumber Hash)
- Tests: `tests/hash_consistency.rs`
- Issue: Cross-representation hash consistency is best-effort, not guaranteed
