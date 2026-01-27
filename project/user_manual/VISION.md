# Vision: faithful-number

## North Star

**faithful-number** is a Rust library that provides exact arithmetic when possible, with explicit tracking when approximation is unavoidable.

The core insight: most numeric libraries force you to choose between speed (f64) and precision (BigDecimal). We provide a third option—**adaptive precision** that automatically selects the minimal representation needed to preserve exactness.

## Core Principles

### 1. Exactness Over Speed
When you write `1/3 + 1/3 + 1/3`, the result should be exactly `1`, not `0.9999999999999999`. We prefer correctness over raw performance, while staying practical (4-10x slower than f64, not 1000x).

### 2. Explicit Approximation
When exactness is lost (transcendental functions, overflow), we mark it explicitly via `ApproximationType`. Users can query `is_exact()` to know if precision was sacrificed.

### 3. Ideological Purity by Default
The default behavior follows mathematical and IEEE 754 semantics, not any specific language runtime. Language-specific behaviors (JavaScript, Python, etc.) are opt-in via feature flags.

### 4. Representation Transparency
Users can inspect which internal representation is used (`Rational`, `Decimal`, `BigDecimal`) and understand promotion/demotion behavior.

## What We Build

A `Number` type that:
- Stores values as `Rational64` when possible (exact fractions)
- Promotes to `Decimal` (28 digits) when rationals overflow
- Promotes to `BigDecimal` when decimals overflow
- Demotes back to simpler representations when operations allow
- Tracks IEEE 754 special values: `NaN`, `±Infinity`, `-0`
- Marks approximations from transcendental operations or overflow

## Feature Philosophy

**Default (no features)**: Pure mathematical/IEEE semantics
- `NaN != NaN` (IEEE 754 compliant, no `Eq` trait)
- Bitwise operations require explicit integer conversion
- String parsing follows Rust conventions
- `-0` preserved (IEEE compliant)

**`js_compat`**: Full JavaScript compatibility (enables all below)
- `js_nan_equality`: `NaN == NaN` in certain contexts
- `js_bitwise`: Implicit ToInt32 coercion for bitwise ops
- `js_string_parse`: Empty string → 0, whitespace trimming

**`high_precision`**: MPFR-backed transcendentals via `rug`

## Success Metrics

From our benchmark (43 test cases across 10 number types):
- **40/43 exact results** (93% exactness)
- **4-10x slower than f64** (acceptable trade-off)
- **Outperforms all alternatives** in combined exactness/capability

## Non-Goals

- Not a drop-in f64 replacement (different semantics)
- Not focused on raw performance (use f64 for that)
- Not a symbolic algebra system (we compute, not simplify)
