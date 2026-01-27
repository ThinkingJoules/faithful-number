# faithful-number

Exact arithmetic when possible, with explicit tracking when approximation is unavoidable.

## Overview

`faithful-number` provides a `Number` type that automatically selects the minimal internal representation needed to preserve exactness:

- **Rational** (exact fractions) when possible
- **Decimal** (28 digits) when rationals overflow
- **BigDecimal** (arbitrary precision) when decimals overflow

When exactness is lost (transcendental functions, overflow), it's marked explicitly via `ApproximationType`.

```rust
use faithful_number::Number;

// The classic floating-point problem: 0.1 + 0.2 != 0.3
// With faithful-number: exact
let a = Number::from_str("0.1").unwrap();
let b = Number::from_str("0.2").unwrap();
let c = Number::from_str("0.3").unwrap();
assert_eq!(a + b, c); // Exact!

// Repeating decimals stay exact as rationals
let third = Number::from(1) / Number::from(3);
let result = third.clone() + third.clone() + third; // Exactly 1
assert_eq!(result, Number::from(1));
```

## Features

### Default (Pure IEEE/Mathematical Semantics)

By default, `faithful-number` follows IEEE 754 and mathematical semantics:

- `NaN != NaN` (IEEE 754 compliant)
- Bitwise operations require explicit integer conversion
- Empty string parsing returns an error
- `-0` is preserved (IEEE compliant)

### Feature Flags

| Feature | Description |
|---------|-------------|
| `js_nan_equality` | `NaN == NaN` returns true, enables `Eq` trait |
| `js_bitwise` | Implicit ToInt32 coercion for `&`, `\|`, `^`, `<<`, `>>` |
| `js_string_parse` | Empty string parses to `0`, whitespace trimming |
| `js_compat` | Enables all JS compatibility features above |
| `high_precision` | MPFR-backed transcendentals via `rug` crate |

```toml
[dependencies]
faithful-number = "0.2"

# For JavaScript-like behavior:
faithful-number = { version = "0.2", features = ["js_compat"] }

# For high-precision transcendentals:
faithful-number = { version = "0.2", features = ["high_precision"] }
```

## Using Numbers in Collections

The default `Number` type does not implement `Eq` (because `NaN != NaN` breaks reflexivity). For HashMap/HashSet use, wrap in `OrderedNumber`:

```rust
use faithful_number::{Number, OrderedNumber};
use std::collections::HashMap;

let mut map: HashMap<OrderedNumber, i32> = HashMap::new();
map.insert(OrderedNumber::from(Number::from(1)), 100);
map.insert(OrderedNumber::from(Number::nan()), 200); // NaN as key works

assert_eq!(map.get(&OrderedNumber::from(Number::from(1))), Some(&100));
```

Alternatively, enable the `js_nan_equality` feature to make `Number` implement `Eq` directly.

## Approximation Tracking

When operations lose exactness, `Number` tracks it:

```rust
let n = Number::from(2).sqrt(); // Transcendental - not exact
assert!(n.is_approximate());
assert!(n.is_transcendental());

let exact = Number::from(4).sqrt(); // Perfect square - exact!
assert!(exact.is_exact());
assert_eq!(exact, Number::from(2));
```

## Migration from v0.1

### Breaking Changes in v0.2

1. **`NaN != NaN` by default** (IEEE 754 compliant)
   - v0.1: `NaN == NaN` returned `true`
   - v0.2: `NaN == NaN` returns `false`
   - Migration: Enable `js_nan_equality` feature OR use `OrderedNumber` for collections

2. **`Number` no longer implements `Eq` by default**
   - v0.1: `Number: Eq` (for HashMap use)
   - v0.2: `Number: PartialEq` only
   - Migration: Use `OrderedNumber` wrapper OR enable `js_nan_equality`

3. **Bitwise operators removed by default**
   - v0.1: `Number & Number` worked with implicit coercion
   - v0.2: Use explicit methods `n.bitand_i32(other)` OR enable `js_bitwise`
   - Migration: Enable `js_bitwise` feature

4. **Empty string parsing returns error**
   - v0.1: `Number::from_str("")` returned `Ok(0)`
   - v0.2: Returns `Err`
   - Migration: Enable `js_string_parse` feature

### Quick Migration

To preserve v0.1 behavior exactly:

```toml
[dependencies]
faithful-number = { version = "0.2", features = ["js_compat"] }
```

## Performance

From our benchmark (43 test cases across 10 number types):

| Metric | Result |
|--------|--------|
| Exact results | 40/43 (93%) |
| Speed vs f64 | 4-10x slower |

The trade-off: near-perfect precision at the cost of raw speed.

## Internal Notes (v0.2 Development)

### Breaking Change Audit

| Item | Count | Impact |
|------|-------|--------|
| `assert_eq!` with Number | ~90 | Will continue working (uses PartialEq) |
| Tests asserting `NaN == NaN` | 3 | Must update or feature-gate |
| HashMap/HashSet with Number | 0 | No migration needed |
| `impl Eq for Number` | 1 | Will be feature-gated |

Tests requiring update:
- `tests/arithmetic.rs:285`
- `src/lib.rs:56`
- `src/js_semantics.rs:469`

## License

[Your license here]
