# Decision: BigDecimal Division Truncation Detection

## Context

When dividing BigDecimal values, the `bigdecimal` crate may truncate repeating decimals without signaling precision loss. For example:
- `1e30 / 3 = 0.333...e30` (repeating)
- BigDecimal silently truncates at some precision
- Original implementation returned `false` for the approximation flag

This violated the faithfulness principle: precision was lost without setting `ApproximationType::RationalApproximation`.

## Problem

The `bigdecimal` crate doesn't provide a way to detect truncation. Division operations don't return metadata about whether the result is exact or approximate.

## Options Considered

1. **Keep result as Rational when possible**
   - For `BigDecimal / small_integer`, try to represent as exact Rational
   - Pros: Preserves exactness when possible
   - Cons: Complex logic, limited applicability (only works for small divisors)

2. **Detect truncation via multiply-back verification**
   - After division, multiply result by divisor and compare to dividend
   - If `(a / b) * b != a`, then truncation occurred
   - Pros: General solution, works for all division cases
   - Cons: Extra computation, requires careful BigDecimal comparison

3. **Always flag BigDecimal division as approximate**
   - Assume all BigDecimal division loses precision
   - Pros: Simple, safe
   - Cons: Overly conservative, flags exact divisions (like `BigDecimal(6) / BigDecimal(3)`)

## Decision

We chose **Option 2: Detect truncation via multiply-back verification**.

After performing BigDecimal division, multiply the result by the divisor and compare to the original dividend. If they differ, set `ApproximationType::RationalApproximation`.

## Reasoning

### Generality

This approach works for all BigDecimal division cases:
- `BigDecimal / BigDecimal`
- `BigDecimal / Decimal`
- `Decimal / BigDecimal`
- `BigDecimal / Rational`
- `Rational / BigDecimal`

### Correctness

The verification is mathematically sound:
- If `a / b = c` exactly, then `c * b = a`
- If `a / b` was truncated to `c`, then `c * b ≠ a`
- BigDecimal comparison handles precision correctly

### Performance

The extra multiplication and comparison add overhead, but:
- BigDecimal division is already expensive (arbitrary precision)
- The verification cost is proportional to the division cost (same operations)
- This is only for BigDecimal operations (rare compared to Rational/Decimal)

### Faithfulness

This is the only option that maintains faithfulness without being overly conservative. Users receive accurate approximation flags.

## Implementation

A helper function performs the verification:

```rust
fn is_bigdecimal_division_truncated(
    dividend: &BigDecimal,
    divisor: &BigDecimal,
    quotient: &BigDecimal,
) -> bool {
    // Verify: quotient * divisor == dividend?
    let verification = quotient * divisor;
    verification != *dividend
}
```

Applied to all BigDecimal division cases:

```rust
(NumericValue::BigDecimal(a), NumericValue::Rational(b, _)) => {
    let b_bd = BigDecimal::from(*b.numer()) / BigDecimal::from(*b.denom());
    let result = a / &b_bd;
    let truncated = is_bigdecimal_division_truncated(a, &b_bd, &result);
    (NumericValue::BigDecimal(result), truncated)
}
```

## Consequences

### Positive
- Accurate truncation detection
- Works for all division cases
- No false positives (exact divisions not flagged)
- Maintains faithfulness guarantee

### Negative
- Additional computation for every BigDecimal division
- Slight performance overhead (typically acceptable given BigDecimal operations are already slow)

## Edge Cases

### Rounding Errors

BigDecimal operations use finite precision. The multiply-back might not exactly equal the dividend due to rounding in the multiplication itself, not the division.

This is acceptable because:
1. If rounding occurs in verification, it also occurred in the division
2. Any difference indicates precision loss somewhere in the operation chain
3. Flagging is conservative (better to flag when unsure than miss a truncation)

## Related

- Plan: faithfulness-fixes (Phase 7)
- Vision principle: "Explicit Approximation"
- Files: `src/ops/arithmetic.rs` (BigDecimal division implementations)
- Tests: `tests/faithfulness_acceptance.rs` (bigdecimal_division_flags_truncation)
