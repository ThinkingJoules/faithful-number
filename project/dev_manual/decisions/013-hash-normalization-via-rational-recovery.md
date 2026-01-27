# Decision: Hash Normalization via Rational Recovery

## Context

Rust requires that equal values hash equally: `a == b` implies `hash(a) == hash(b)`. In faithful-number, the same mathematical value can have different internal representations:
- `Rational(1, 2)` and `Decimal(0.5)` are equal but structurally different
- Original implementation: `Rational` hashed as `(discriminant, numer, denom)`, `Decimal` hashed as normalized string
- This violated the hash consistency requirement

## Options Considered

1. **Normalize all values to f64 before hashing**
   - Pros: O(1) conversion, simple
   - Cons: Loses precision, defeats purpose of faithful-number

2. **Normalize to canonical rational form before hashing**
   - Pros: Exact, preserves faithfulness
   - Cons: O(n) where n = number of decimal digits (for Decimal → Rational conversion)

3. **Hash based on mathematical value using BigInt**
   - Pros: Exact
   - Cons: Complex, expensive for large values

## Decision

We chose **Option 2: Normalize to canonical rational form**.

For Decimal values, attempt rational recovery using the existing `try_decimal_to_rational()` function. If successful, hash as Rational. If recovery fails (non-terminating or exceeds denominator limit), hash the normalized decimal string as before.

## Reasoning

### Performance Trade-off

The O(n) cost is acceptable because:
- n is bounded by Decimal's 28-digit precision (small constant)
- Hashing is already O(n) for string normalization
- Rational recovery is only attempted once per hash operation
- HashMap/HashSet usage is not typically performance-critical in this library's domain

### Correctness

This is the only approach that maintains both:
1. Hash consistency: `a == b` → `hash(a) == hash(b)`
2. Faithfulness: no precision loss through f64

### User Input

During plan discussion, the user was asked:
> "For hash consistency: is O(n) normalization acceptable, or do we need O(1) approach?"

User response:
> "O(n) is acceptable"

## Consequences

### Positive
- Hash consistency requirement satisfied
- Works correctly with HashMap/HashSet
- No precision loss
- Uses existing rational recovery infrastructure

### Negative
- Slightly slower than direct hashing (O(n) vs O(1) for Rational)
- Decimal values that can't be recovered hash differently than equivalent rationals that exceed the denominator limit (but these cases are rare and technically different representations)

## Implementation

```rust
impl Hash for NumericValue {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            NumericValue::Rational(r, _) => {
                3u8.hash(state);  // Discriminant
                r.numer().hash(state);
                r.denom().hash(state);
            }
            NumericValue::Decimal(d) => {
                // Try to recover as rational for consistent hashing
                if let Some(r) = try_decimal_to_rational(*d) {
                    3u8.hash(state);  // Same discriminant as Rational
                    r.numer().hash(state);
                    r.denom().hash(state);
                } else {
                    // Can't recover, hash as decimal
                    3u8.hash(state);
                    d.normalize().hash(state);
                }
            }
            // ... other variants
        }
    }
}
```

## Related

- Plan: faithfulness-fixes (Phase 6)
- Decision: 011-cf-denominator-limit (rational recovery uses CF with 10^9 limit)
- Files: `src/traits.rs` (Hash impl)
- Tests: `tests/faithfulness_acceptance.rs` (equal_values_hash_equal, hashmap_works_across_representations)
