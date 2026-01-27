# Decision: to_decimal() Returns None for Non-Terminating Rationals

## Context

The `to_decimal()` method converts a `Number` to the `rust_decimal::Decimal` type, which has 28 digits of precision. When the internal representation is a non-terminating rational like 1/3 (0.333...), we must decide how to handle the conversion.

## Options Considered

1. **Truncate to 28 digits and return Some(Decimal)**
   - Pros: Always succeeds, provides a value
   - Cons: Silent precision loss, truncated 1/3 is not exact, violates faithfulness

2. **Return None for non-terminating rationals**
   - Pros: Honest about inability to exactly represent, maintains faithfulness
   - Cons: Conversion may fail even for valid numbers

3. **Return truncated value with approximation flag**
   - Pros: Provides a value and signals approximation
   - Cons: `to_decimal()` returns `Option<Decimal>`, not `Option<(Decimal, bool)>`, so can't return flag

## Decision

We chose **Option 2: Return None for non-terminating rationals**.

## Reasoning

The method signature `to_decimal() -> Option<Decimal>` implies an exact conversion. The `Option` signals that conversion may not be possible, not that conversion is approximate. A truncated 1/3 is fundamentally not an exact decimal representation.

This aligns with the library's core principle: **no silent precision loss**. If a user receives `Some(Decimal)`, they can trust it's exact. If they receive `None`, they know exact conversion isn't possible and can choose an alternative approach.

### User Input

During plan discussion, the user was asked:
> "For `to_decimal()` on non-terminating rationals (1/3): return `None`, or return truncated value with flag?"

User response:
> "We are trying to be faithful to math, so I would say exact is correct answer"

## Consequences

### Positive
- Maintains faithfulness: `Some(value)` always means exact
- Clear contract: method either succeeds exactly or fails
- Users must explicitly choose how to handle non-terminating rationals

### Negative
- `to_decimal()` can fail for valid numbers like 1/3
- Users need to handle `None` case, can't assume success
- May require documentation to explain when/why it fails

## Implementation

The implementation checks the cached `is_terminating` flag on Rational values:

```rust
NumericValue::Rational(r, is_terminating) => {
    if !is_terminating {
        return None; // Non-terminating, cannot be exactly represented
    }
    // Terminating: use exact Decimal arithmetic
    Some(Decimal::from(*r.numer()) / Decimal::from(*r.denom()))
}
```

## Related

- Plan: faithfulness-fixes (Phase 4)
- Vision principle: "Explicit Approximation"
- Files: `src/math.rs` (to_decimal method)
