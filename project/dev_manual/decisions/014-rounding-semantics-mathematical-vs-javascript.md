# Decision: Rounding Semantics - Mathematical vs JavaScript

## Context

The `Number::round()` method needed to define how to handle ties (values exactly at 0.5). Different languages use different conventions:
- **JavaScript**: `Math.round()` uses "round half toward positive infinity" (-1.5 → -1, 1.5 → 2)
- **Mathematics**: Common convention is "round half away from zero" (-1.5 → -2, 1.5 → 2)
- **Rust f64**: `round()` uses "round half away from zero"
- **IEEE 754**: Recommends "round half to even" (banker's rounding)

The library's original implementation matched JavaScript's behavior, but this conflicted with the VISION principle of "ideological purity by default."

## Options Considered

1. **Keep JavaScript semantics (half toward positive infinity)**
   - Pros: Matches existing implementation, useful for JS compatibility
   - Cons: Violates VISION.md principle of mathematical defaults

2. **Use mathematical convention (half away from zero)**
   - Pros: Aligns with VISION.md, matches Rust's f64::round(), most intuitive
   - Cons: Breaking change from v0.1 behavior

3. **Use IEEE 754 recommendation (half to even)**
   - Pros: Standard, reduces bias in repeated operations
   - Cons: Less intuitive, more complex to implement for Rational

## Decision

We chose **Option 2: Mathematical convention (half away from zero)**.

The default `round()` method rounds ties away from zero. JavaScript compatibility should be achieved through the `js_compat` feature flag if needed (future work).

## Reasoning

### VISION Alignment

From VISION.md:
> "**Ideological Purity by Default**: The default behavior follows mathematical and IEEE 754 semantics, not any specific language runtime."

The library should not favor JavaScript's quirks in the default behavior. JavaScript-specific semantics belong behind feature flags.

### Intuitive Behavior

"Half away from zero" matches most people's intuition:
- Positive 1.5 rounds to 2 (up)
- Negative -1.5 rounds to -2 (down, away from zero)

This is symmetric and easy to predict.

### Consistency with f64

Rust's `f64::round()` uses half away from zero, so matching this behavior reduces surprise for Rust developers.

### Implementation During Faithfulness Fixes

During Phase 3, the original implementation was:
```rust
let rounded = if f >= 0.0 {
    (f + 0.5).floor()
} else {
    (f + 0.5).ceil()  // JavaScript: -3.5 becomes -3
};
```

The acceptance test specified:
```rust
// -3/2 = -1.5 should round to -2 (away from zero)
let n = Number::from(-3) / Number::from(2);
assert_eq!(n.round(), Number::from(-2));
```

This caused a conflict with the existing JS semantics test, which expected -3.5 → -3.

The test was updated to reflect mathematical semantics as the default.

## Consequences

### Positive
- Aligns with VISION.md principles
- Intuitive and symmetric behavior
- Matches Rust's f64 behavior
- Clear path for JS compatibility via feature flag

### Negative
- Breaking change from v0.1 (but library is unpublished)
- JavaScript compatibility requires explicit feature flag (future work)

## Implementation

For Rational values, the implementation uses:
```rust
fn round(&self) -> Number {
    match &self.value {
        NumericValue::Rational(r, is_term) => {
            let numer = r.numer();
            let denom = r.denom();

            // Half away from zero
            let doubled = numer * 2;
            let rounded = if doubled >= 0 {
                (doubled + denom) / (denom * 2)
            } else {
                (doubled - denom) / (denom * 2)
            };

            Number::from(rounded)
        }
        // ... Decimal and BigDecimal use native methods
    }
}
```

## Implementation

The `js_rounding` feature flag (part of `js_compat` umbrella) modifies `round()` to use JavaScript's "half toward positive infinity" semantics:

```rust
// With js_rounding feature:
// round(1.5) = 2, round(-1.5) = -1

// Without js_rounding feature (default):
// round(1.5) = 2, round(-1.5) = -2
```

The feature is included in the `js_compat` umbrella alongside `js_nan_equality`, `js_bitwise`, and `js_string_parse`.

## Related

- Plan: faithfulness-fixes (Phase 3)
- Vision: "Ideological Purity by Default"
- Files: `src/math.rs` (round implementation)
- Tests: `tests/faithfulness_acceptance.rs` (round_exact_half_away_from_zero)
