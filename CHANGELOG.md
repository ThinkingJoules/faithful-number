# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2026-01-26

### Breaking Changes

- **`NaN != NaN` by default** - Now follows IEEE 754 semantics
  - Previously: `NaN == NaN` returned `true`
  - Now: `NaN == NaN` returns `false`
  - Migration: Enable `js_nan_equality` feature or use `OrderedNumber` wrapper

- **`Number` no longer implements `Eq` by default**
  - Required for IEEE 754 compliance (NaN breaks reflexivity)
  - Migration: Use `OrderedNumber` for HashMap/HashSet, or enable `js_nan_equality`

- **Bitwise operators removed by default**
  - `&`, `|`, `^`, `<<`, `>>` traits no longer implemented without feature flag
  - Explicit methods always available: `bitand_i32()`, `bitor_i32()`, etc.
  - Migration: Enable `js_bitwise` feature or use explicit methods

- **Empty string parsing returns error**
  - Previously: `Number::from_str("")` returned `Ok(0)`
  - Now: Returns `Err(())`
  - Migration: Enable `js_string_parse` feature

### Added

- **`OrderedNumber` wrapper** - Enables `Eq`, `Ord`, `Hash` for collection use
  - NaN == NaN within OrderedNumber for practical collection semantics
  - Works with HashMap, HashSet, BTreeSet, etc.

- **Feature flags for JS compatibility**
  - `js_nan_equality` - Makes NaN == NaN, enables `Eq` trait
  - `js_bitwise` - Enables bitwise operator traits
  - `js_string_parse` - JS string parsing (empty string → 0, whitespace trimming)
  - `js_compat` - Umbrella feature enabling all of the above

- **Serialization support**
  - `serde_str` - String-based serialization for JSON, TOML, etc.
    - Serializes as `["value"]` or `["value", "transcendental"]`
    - Preserves approximation metadata on roundtrip
  - `serde_bin` - Binary serialization via onenum for bincode, etc.
    - Compact binary format with sortable encoding
    - Approximation type encoded as suffix byte (preserves sort order)
  - Note: `serde_str` and `serde_bin` are mutually exclusive

- **Rich formatting (`format` feature)**
  - `DisplayOptions` - Configurable number display
  - `ParseOptions` - Configurable number parsing
  - `RegionalFormat` - US, European, SI, and Indian number formats
  - `Notation` - Standard, Scientific, and Engineering notation
  - `Number::format()` and `Number::parse_formatted()` methods
  - Roundtrip guarantee: `parse_formatted(n.format(opts), opts) == n`

- **New methods on `Number`**
  - `is_neg_zero()` - Check for negative zero
  - `is_zero()` - Check for any zero value
  - `is_neg_infinity()` - Alias for is_negative_infinity()
  - `bitand_i32()`, `bitor_i32()`, `bitxor_i32()`, `bitnot_i32()` - Explicit bitwise ops
  - `shl_i32()`, `shr_i32()` - Explicit shift ops

- **Comprehensive test suite**
  - Adversarial tests for overflow boundaries
  - Bitwise operation edge cases
  - String parsing roundtrips
  - Hash consistency verification
  - Negative zero semantics

### Fixed

- `Hash` now properly implemented for `Rational` and `BigDecimal` variants
- `signum` implemented for `Rational` and `BigDecimal`
- `round_dp` and `trunc` implemented for `BigDecimal`
- All `to_i32`/`to_i64`/`to_u32`/`to_u64` conversions implemented for `BigDecimal`
- No more `unimplemented!()` panics in public API

### Changed

- Default behavior is now "pure" IEEE 754 / mathematical semantics
- JS-specific behaviors are opt-in via feature flags
- Project documentation structure added (`project/user_manual/`, `project/dev_manual/`)

## [0.1.0] - Initial Release

- Multi-representation arithmetic (Rational → Decimal → BigDecimal)
- Automatic representation promotion and demotion
- Approximation tracking for transcendental operations
- JavaScript-compatible semantics (NaN == NaN, implicit bitwise coercion)
- IEEE 754 special values (NaN, ±Infinity, -0)
- Optional high-precision transcendentals via `rug` crate
