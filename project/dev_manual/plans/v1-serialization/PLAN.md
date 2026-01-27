# Plan: v1 Serialization

## Goal

Add complete serialization support to faithful-number: string-based serde, binary serde via onenum, and rich human-readable formatting with round-trip parsing.

## Vision Alignment

From VISION.md:
> "Representation Transparency: Users can inspect which internal representation is used"

Serialization is foundational to transparency - you can't inspect or persist numbers without it. Additionally, preserving `ApproximationType` through serialization keeps us "faithful" to our name.

## Context

Current state (v0.2.0):
- Basic `Display` and `FromStr` exist (universal machine format)
- No serde support
- No rich formatting (regional, scientific)
- Deferred in decision 004 pending design clarity

We now have clarity:
- **onenum** (sibling repo) provides exact binary-sortable encoding
- **compute-num-rs** (sibling repo) has display type definitions to copy
- String serialization uses array format: `["value", "approx?"]`

## Approach

Three feature flags, each independent:
1. `serde_str` - string-based serde (JSON, TOML, etc.)
2. `serde_bin` - binary serde via onenum (bincode, postcard, etc.)
3. `format` - rich display/parse for human formats

Phases build foundation first (format types), then add serde implementations.

## Phases

### Phase 1: Format Types and Display

**Outcome**: Rich display formatting works with `Number::format(&self, opts: &DisplayOptions) -> String`

Steps:
- [ ] Copy types from compute-num-rs/src/display.rs: `ExpNotation`, `Notation`, `RegionalFormat`, `DisplayOptions`
- [ ] Add to new file `src/format.rs`, gate behind `format` feature
- [ ] Implement `format_number()` logic for each `Notation` variant (Decimal, Scientific, Engineering)
- [ ] Implement regional formatting (thousands separators, decimal separator)
- [ ] Add `Number::format(&self, opts: &DisplayOptions) -> String` method
- [ ] Tests for US, European, SI, Indian formats
- [ ] Tests for scientific and engineering notation

Files likely touched:
- `src/format.rs` (new)
- `src/lib.rs` (module declaration, feature gate)
- `Cargo.toml` (feature flag)
- `tests/format_display.rs` (new)

---

### Phase 2: Format Parsing

**Outcome**: Round-trip works: `Number::parse_formatted()` can parse anything `format()` produces

**Error handling:**
```rust
pub enum ParseError {
    EmptyInput,
    InvalidCharacter { pos: usize, ch: char },
    MultipleSeparators,      // e.g., "1..5" or "1,,234"
    MismatchedFormat,        // input doesn't match expected regional format
    Overflow,                // number too large for any representation
}
```

Steps:
- [ ] Define `ParseError` enum with descriptive variants
- [ ] Define `ParseOptions` struct (mirrors `RegionalFormat` + notation hints)
- [ ] Implement parsing logic that handles regional separators
- [ ] Implement scientific/engineering notation parsing
- [ ] Add `Number::parse_formatted(s: &str, opts: &ParseOptions) -> Result<Number, ParseError>`
- [ ] Tests for round-trip: format then parse equals original
- [ ] Tests for each error variant
- [ ] Tests for edge cases (ambiguous formats require explicit options)

Files likely touched:
- `src/format.rs`
- `tests/format_parse.rs` (new)

---

### Phase 3: serde_str Feature

**Outcome**: `serde_str` feature enables serde Serialize/Deserialize using string array format

Serialization format:
```json
["1.4142135...", "transcendental"]       // ApproximationType::Transcendental
["0.333...", "rational_approximation"]   // ApproximationType::RationalApproximation
["0.5"]                                   // exact (no second element)
```

ApproximationType strings are **lowercase**: `"transcendental"`, `"rational_approximation"`

Steps:
- [ ] Add `serde` dependency (optional, enabled by `serde_str`)
- [ ] Create `src/serde_impl.rs` with `#[cfg(all(feature = "serde_str", not(feature = "serde_bin")))]`
- [ ] Implement `Serialize` for `Number` - emit as array
- [ ] Implement `Deserialize` for `Number` - parse array, handle optional approx
- [ ] Implement `Serialize`/`Deserialize` for `ApproximationType` (lowercase strings)
- [ ] Tests with serde_json
- [ ] Tests for round-trip through JSON

Files likely touched:
- `src/serde_impl.rs` (new)
- `src/lib.rs` (module declaration, feature gate)
- `Cargo.toml` (serde dependency, feature flag)
- `tests/serde_str.rs` (new)

---

### Phase 4: serde_bin Feature

**Outcome**: `serde_bin` feature enables serde Serialize/Deserialize using onenum binary encoding

Serialization format:
```
[onenum_bytes...][approx_byte]
```
Approx byte is a **suffix** to preserve onenum's lexicographical sortability.
Where `approx_byte`: 0 = exact, 1 = transcendental, 2 = rational_approximation

**Conversion strategy:**
All our representations can convert to exact ratios (they all terminate):
- `Rational` → `Ratio<i64>` → onenum
- `Decimal` → `Ratio<i128>` (mantissa / 10^scale) → onenum
- `BigDecimal` → `Ratio<BigInt>` → onenum
- Special values (NaN, ±Infinity, -0) → onenum handles these directly

Steps:
- [ ] Add `onenum` dependency (optional, enabled by `serde_bin`)
- [ ] Implement conversion: `Number` → ratio → `onenum::Onum`
- [ ] Implement conversion: `onenum::Onum` → math operations → `Number` (auto-repr via normal routines)
- [ ] Add serde impl in `src/serde_impl.rs` with `#[cfg(all(feature = "serde_bin", not(feature = "serde_str")))]`
- [ ] Implement `Serialize` - onenum bytes + approx suffix byte
- [ ] Implement `Deserialize` - split suffix, decode onenum, reconstruct Number with approx
- [ ] Tests with bincode
- [ ] Tests for round-trip through binary
- [ ] Tests for lexicographical sortability

Files likely touched:
- `src/serde_impl.rs`
- `Cargo.toml` (onenum dependency, feature flag)
- `tests/serde_bin.rs` (new)

---

### Phase 5: Documentation and Cleanup

**Outcome**: Features are documented, examples work, clippy/aw clean

Steps:
- [ ] Add feature documentation to README.md
- [ ] Add examples for each feature
- [ ] Ensure `cargo clippy --all-features` passes
- [ ] Ensure `aw check` passes (or document acceptable violations)
- [ ] Update CHANGELOG.md

Files likely touched:
- `README.md`
- `CHANGELOG.md`
- `examples/` (new examples)

---

## Implementation Context

**Codebase maturity**: Evolving
- Core types are stable (Number, NumericValue, ApproximationType)
- Adding new modules, not changing existing structure

**Type strategy**:
- Copy DisplayOptions types as-is from compute-num-rs
- ParseOptions can be simpler (just RegionalFormat + flags)
- Serde impls are straightforward trait implementations

**Error handling**:
- `ParseError` enum for format parsing failures
- Serde errors use serde's error types

**Notes for implementer**:
- `serde_str` and `serde_bin` mutual exclusivity enforced via cfg:
  - `#[cfg(all(feature = "serde_str", not(feature = "serde_bin")))]`
  - `#[cfg(all(feature = "serde_bin", not(feature = "serde_str")))]`
  - If both enabled, neither impl compiles (no Serialize trait)
- onenum decoding should use math operations so auto-repr selection works
- Format round-trip is the key invariant: `parse_formatted(format(n, opts), opts) == n`

**Feature interactions**:
- Serialization features are **independent** of `js_*` features
- `parse_formatted()` does NOT inherit `js_string_parse` behavior (empty string → 0)
- `parse_formatted()` uses only explicit `ParseOptions`, no implicit behaviors
- The `js_*` features affect Number creation/comparison, not serialization

## Risks

- **onenum API changes**: Low risk, sibling repo under same control
- **Regional format ambiguity**: Mitigated by requiring explicit ParseOptions

## Out of Scope

- Source code formats (hex floats, octal, binary literals)
- Symbolic simplification
- Additional language compatibility modes (Python, Ruby)
- Performance optimization

## Open Questions

- [ ] Should `format` feature be default? (Adds no deps, just code)
- [ ] Exact onenum version to depend on (after bigint fix)
