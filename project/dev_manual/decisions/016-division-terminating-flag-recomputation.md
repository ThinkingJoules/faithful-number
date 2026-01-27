# Decision: Division Terminating Flag Must Be Recomputed

## Context

Rational values in faithful-number carry a cached `is_terminating` flag indicating whether the fraction can be exactly represented as a terminating decimal. This flag is used to optimize conversions and operations.

During implementation of Phase 4 (exact to_decimal), a critical bug was discovered: the division operation was incorrectly computing the terminating flag for the result.

## The Bug

Original implementation:
```rust
(NumericValue::Rational(a, a_term), NumericValue::Rational(b, b_term)) => {
    if let Some(result) = a.checked_div(&b) {
        let is_term = a_term && b_term;  // ❌ WRONG
        (NumericValue::Rational(result, is_term), false)
    }
    // ...
}
```

This logic assumed: "If both inputs are terminating, the result is terminating."

### Why This Is Wrong

Dividing two terminating rationals can produce a non-terminating result:
- `1 / 3 = 1/3` (non-terminating, even though `1` and `3` are both terminating integers)
- `1/2 / 3 = 1/6` (still non-terminating, `6 = 2 × 3`)

The bug manifested when:
```rust
let r = Number::from(1) / Number::from(3);
// r.value = Rational(1/3, true)  ❌ Should be (1/3, false)
r.to_decimal()  // Returned Some(...) ❌ Should return None
```

## Correct Approach

The terminating property depends on the **result's denominator**, not the inputs' flags.

A rational `n/d` is terminating if and only if `d = 2^a × 5^b` for some non-negative integers a, b. This must be checked on the result.

## Decision

**Always recompute the terminating flag based on the result's denominator, never derive it from input flags.**

## Implementation

Corrected division:
```rust
(NumericValue::Rational(a, _), NumericValue::Rational(b, _)) => {
    if let Some(result) = a.checked_div(&b) {
        use crate::core::is_terminating_decimal;
        let is_term = is_terminating_decimal(*result.numer(), *result.denom());
        (NumericValue::Rational(result, is_term), false)
    }
    // ...
}
```

The `is_terminating_decimal` function checks if the denominator's prime factorization contains only 2s and 5s:
```rust
pub fn is_terminating_decimal(numer: i64, denom: i64) -> bool {
    let mut d = denom.abs();

    // Remove all factors of 2
    while d % 2 == 0 {
        d /= 2;
    }

    // Remove all factors of 5
    while d % 5 == 0 {
        d /= 5;
    }

    // If only 2s and 5s, d should now be 1
    d == 1
}
```

## Why Not Cache Smarter?

Could we avoid recomputation by tracking the terminating flag through operations more carefully?

### Multiplication
`(a/b) × (c/d) = (a×c) / (b×d)`

If both inputs are terminating:
- b = 2^a1 × 5^b1
- d = 2^a2 × 5^b2
- b×d = 2^(a1+a2) × 5^(b1+b2)

Result is terminating. **Could cache: `a_term && b_term`**.

### Addition/Subtraction
`(a/b) + (c/d) = (a×d + b×c) / (b×d)`

Same denominator logic as multiplication. **Could cache: `a_term && b_term`**.

### Division
`(a/b) / (c/d) = (a/b) × (d/c) = (a×d) / (b×c)`

New denominator is `b×c`. Even if b and c are individually terminating:
- b = 2^a1 × 5^b1
- c = 2^a2 × 5^b2
- b×c = 2^(a1+a2) × 5^(b1+b2)

Result **should** be terminating... but wait:
- `1 / 3 = 1/3`, and `3 ≠ 2^a × 5^b`

The issue is when one input is an **integer** (denominator = 1), which is trivially terminating. Dividing by a non-terminating integer (like 3) produces a non-terminating result.

**Cannot reliably cache for division without checking the result.**

## Decision Rationale

Given that:
1. Division is the problematic case
2. The check is cheap (O(log d) factorization, d ≤ i64::MAX)
3. Correctness is critical for faithfulness
4. Trying to be clever with caching adds complexity and bug risk

**The safest approach is to always recompute for division.**

## Consequences

### Positive
- Correctness guaranteed
- Bug prevented (to_decimal now works correctly)
- Simple, maintainable code

### Negative
- Small performance cost for division (factorization check)
- Redundant for cases where caching would work (multiplication, addition)

### Future Optimization

If profiling shows the terminating check is a bottleneck, we could:
1. Keep recomputation for division
2. Use cached flags for addition/subtraction/multiplication
3. Add a debug assertion that cached flags match recomputed values

## Related

- Plan: faithfulness-fixes (Phase 4, bug discovered)
- Vision principle: "Exactness Over Speed"
- Files: `src/ops/arithmetic.rs` (Div impl), `src/core.rs` (is_terminating_decimal)
- Tests: `tests/faithfulness_acceptance.rs` (to_decimal_non_terminating_returns_none)
