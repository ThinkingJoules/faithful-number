# Decision 011: CF Denominator Limit

## Context

The `try_decimal_to_rational` function uses continued fractions (CF) to recover exact rational representations from decimals. The algorithm requires a maximum denominator bound to prevent unbounded computation and ensure the result fits in `Rational64` (which uses `i64` for numerator and denominator).

## Problem

The codebase had inconsistent denominator limits:
- `try_decimal_to_rational` used 10^9 (1,000,000,000)
- `try_decimal_to_rational_bigdecimal` used `i64::MAX` (~9.2×10^18)

This inconsistency could cause issues:
1. Rationals recovered from BigDecimal might have denominators that overflow on the first arithmetic operation
2. Behavior differs based on the source representation

## Decision

**Use 10^9 as the uniform denominator limit for all CF-based rational recovery.**

## Rationale

### Arithmetic Safety

When multiplying two rationals, the product denominator is (d1 × d2). With:
- Limit = 10^9: product denominator ≤ 10^18, safely within i64 range (~9.2×10^18)
- Limit = i64::MAX: product denominator could exceed i128 on first operation

The 10^9 limit provides headroom for:
- One multiplication: up to 10^18
- Division (which inverts): still up to 10^18
- Addition/subtraction (cross-multiply): 10^9 × 10^9 = 10^18

### Continued Fractions Theory

For decimal numbers with N significant digits, the CF algorithm typically converges in O(log N) iterations. The denominator grows roughly exponentially with iterations. A limit of 10^9:
- Is large enough to exactly represent most common fractions
- Stops before denominators become unwieldy
- Matches the 28-digit precision of Decimal (10^28 mantissa / 10^28 scale ≈ simple fractions)

### Examples

Common fractions and their denominators:
- 1/3 → denom 3 (well under limit)
- 1/7 → denom 7 (well under limit)
- 0.142857... (repeating) → 1/7, denom 7
- 0.1 → 1/10, denom 10
- 1/999999999 → denom 999,999,999 (at limit)

Fractions that would exceed 10^9:
- 1/1000000001 → cannot be recovered (stays as Decimal)
- 123456789123/1234567891234 → cannot be recovered

This is acceptable because:
1. Such fractions are rare in practice
2. The Decimal representation is still exact to 28 digits
3. Operations on very large denominators would likely overflow anyway

## Consequences

### Positive
- Consistent behavior regardless of input representation
- Safe arithmetic without overflow checking on every operation
- Predictable performance (CF iterations bounded)

### Negative
- Some exact rationals with denominators > 10^9 won't be recovered
- These stay as Decimal (still exact, just different representation)

## Implementation

Both `try_decimal_to_rational` and `try_decimal_to_rational_bigdecimal` use the same `CF_MAX_DENOM` constant defined at module level.

See: `src/core.rs`, constant `CF_MAX_DENOM`
