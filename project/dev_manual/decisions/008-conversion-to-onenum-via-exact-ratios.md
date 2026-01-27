# Decision: Binary Serialization Converts All Representations to Exact Ratios

## Context

The `serde_bin` feature uses onenum for binary encoding. onenum accepts rational numbers (ratios of integers) and special values (NaN, ±∞, -0).

Our `Number` type has three internal representations:
- `Rational` - already a ratio (i64/i64)
- `Decimal` - fixed-point (i128 mantissa, u8 scale)
- `BigDecimal` - arbitrary precision decimal

The question: How do we convert these to onenum format?

## Conversion Strategy

All of our representations can convert to **exact terminating ratios**:

- `Rational(num, denom)` → Already a `Ratio<i64>` → `Ratio<BigInt>`
- `Decimal(mantissa, scale)` → `mantissa / 10^scale` (exact ratio)
- `BigDecimal` → `mantissa / 10^scale` (exact ratio with `BigInt`)
- Special values (NaN, ±∞, -0) → onenum handles these directly via `SpecialValue`

User insight: "surely these can convert to a CF that terminates, as they must terminate"

This is correct - all our representations are **exact decimal numbers**, which means they have terminating decimal expansions, which means they can be expressed as exact ratios with denominators that are powers of 10 (and factors of 2 and 5).

## Implementation

```rust
fn number_to_onum(num: &Number) -> Onum<DefaultEqTolerance> {
    match &num.value {
        NumericValue::NaN => Onum::from_special(SpecialValue::NaN),
        NumericValue::PositiveInfinity => Onum::from_special(SpecialValue::PositiveInfinity),
        NumericValue::NegativeInfinity => Onum::from_special(SpecialValue::NegativeInfinity),
        NumericValue::NegativeZero => Onum::from_special(SpecialValue::NegZero),

        NumericValue::Rational(r, _) => {
            // Convert i64 ratio to BigInt ratio
            let n = BigInt::from(*r.numer());
            let d = BigInt::from(*r.denom());
            Onum::from_number(Ratio::new(n, d))
        }

        NumericValue::Decimal(d) => {
            // Convert to ratio: mantissa / 10^scale
            let mantissa = BigInt::from(d.mantissa());
            let scale = d.scale();
            let denominator = BigInt::from(10).pow(scale as u32);
            Onum::from_number(Ratio::new(mantissa, denominator))
        }

        NumericValue::BigDecimal(bd) => {
            // BigDecimal already has (bigint, scale) internally
            // Convert to ratio via string or direct construction
            let (mantissa, scale) = bd.as_bigint_and_exponent();
            let denom = if scale >= 0 {
                BigInt::one()
            } else {
                BigInt::from(10).pow((-scale) as u32)
            };
            Onum::from_number(Ratio::new(mantissa, denom))
        }
    }
}
```

## Deserialization Strategy

On deserialization, we convert onenum back to `Number` by using **string conversion and normal construction routines**:

```rust
fn onum_to_number(onum: &Onum<DefaultEqTolerance>) -> Number {
    if onum.is_special_value() {
        // Handle NaN, ±∞, -0
        // ...
    } else {
        // Convert to string, then parse
        // This lets Number's normal construction logic choose the best representation
        let s = onum.to_string();
        s.parse().unwrap()
    }
}
```

User clarification on representation recovery: "In theory we shouldn't need this? I think we have a routine that make sures it is in the proper repr. We would start with 0 and apply mathematical ops to decode the cf terms wouldn't we? If so the normal routines will handle this."

This is brilliant - by using `FromStr`, we let the normal representation selection logic work. The string "0.5" becomes a `Rational`, "0.333..." triggers approximation logic, etc.

## Decision

We use **exact ratio conversion** for encoding and **string round-trip** for decoding.

This approach:
- Preserves exactness (no precision loss)
- Lets onenum handle the efficient continued fraction encoding
- Lets our normal construction logic choose the best representation on decode
- Works for all three internal representations

## Consequences

### Enables
- **Lossless encoding**: All values round-trip exactly
- **Automatic repr selection**: Deserialization uses normal Number construction
- **Simple implementation**: No special cases for each repr type

### Prevents
- **Representation preservation**: A `Decimal` might deserialize as a `Rational` if the value fits
- **Metadata loss**: We don't serialize which repr was used originally

### Future Implications
- The approximation type is preserved via the suffix byte
- The original representation type is NOT preserved (only the value)
- This is acceptable because representation is an internal optimization, not part of the number's identity
- If users need representation preservation, they should use `serde_str` which serializes to string that preserves representation via `Display`/`FromStr`

## Related

- Plan: v1-serialization (Phase 4)
- Files: `src/serde_impl.rs` (number_to_onum, onum_to_number functions)
- Decision: 006-binary-serialization-approx-suffix.md
- Dependency: onenum crate's continued fraction encoding
