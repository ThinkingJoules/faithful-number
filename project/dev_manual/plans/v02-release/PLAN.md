# Plan: v0.2 Release - Ideological Purity & Test Hardening

## Goal

Ship v0.2 with feature-gated JS compatibility, comprehensive adversarial tests, and proper documentation structure.

## Vision Alignment

From VISION.md: "Ideological purity by default. The default behavior follows mathematical and IEEE 754 semantics, not any specific language runtime. Language-specific behaviors are opt-in via feature flags."

This release establishes the foundation for that principle.

## Context

Current state (v0.1):
- JS semantics baked in as defaults (NaN==NaN for Eq, implicit bitwise coercion)
- 40/43 exact results in benchmark suite
- Test coverage gaps: dedicated bitwise tests, overflow boundaries, FromStr edge cases
- No VISION.md or documentation structure
- `unimplemented!()` panics in BigDecimal paths (Hash, to_i32_js_coerce, etc.)

## Approach

1. Establish documentation structure and vision
2. Audit current equality/hash usage before breaking changes
3. Refactor JS-specific behaviors behind feature flags (with OrderedNumber wrapper)
4. Fix panicking paths (Hash, BigDecimal conversions)
5. Add adversarial tests
6. Polish for release

## Resolved Questions

**Q: Should we provide an `OrderedNumber` wrapper that impls Eq for HashMap use?**
A: YES. Provide `OrderedNumber` BEFORE removing `Eq` from `Number`. This is the migration path.

**Q: What's the minimum BigDecimal math support needed for 0.2?**
A: Implement `to_i32/i64`, `round_dp`, `trunc`, and `Hash`. Defer `pow`, `sqrt`, trig to 0.3.

**Q: Should `js_compat` be the default for 0.1 → 0.2 migration ease?**
A: NO. Pure defaults. v0.1 users add `features = ["js_compat"]` to preserve behavior. Breaking change is acceptable for 0.2.

## Feature Flag Architecture

```toml
[features]
default = []

# Individual JS behaviors
js_nan_equality = []     # NaN == NaN returns true, enables Eq trait
js_bitwise = []          # Implicit ToInt32 coercion for &, |, ^, <<, >>
js_string_parse = []     # Empty string → 0, whitespace trimming

# Umbrella feature
js_compat = ["js_nan_equality", "js_bitwise", "js_string_parse"]

# High precision (existing)
high_precision = ["rug"]
```

All features are **additive**. Default (no features) is pure IEEE/mathematical semantics.

## Phases

### Phase 1: Documentation Foundation

**Outcome**: Project has proper doc structure with VISION.md as north star.

Steps:
- [x] Create `project/user_manual/VISION.md`
- [x] Create `project/dev_manual/` structure
- [ ] Add README.md with feature documentation and migration guide
- [ ] Document breaking changes from v0.1

Files touched:
- `project/user_manual/VISION.md` (new)
- `README.md` (new or update)

---

### Phase 2: Breaking Change Audit

**Outcome**: Understand full scope of Eq removal before implementing.

Steps:
- [ ] Grep for all `==` and `assert_eq!` usage with Number
- [ ] Find all HashMap/HashSet usage with Number keys
- [ ] Identify all tests that rely on `Eq` trait
- [ ] Document migration path in README

Commands:
```bash
grep -rn "assert_eq!" tests/ src/ | grep -i number
grep -rn "HashMap.*Number\|HashSet.*Number" src/ tests/
grep -rn "impl.*Eq.*Number" src/
```

Files touched:
- `README.md` (migration section)

---

### Phase 3: Feature Flag Architecture & OrderedNumber

**Outcome**: JS behaviors are opt-in, defaults are "pure", OrderedNumber enables collection use.

Steps:
- [ ] Add features to Cargo.toml (see Feature Flag Architecture above)
- [ ] Create `OrderedNumber` wrapper type that always impls Eq (NaN == NaN)
- [ ] Implement Hash for `OrderedNumber`
- [ ] Fix Hash `unimplemented!()` for Rational and BigDecimal variants
- [ ] Refactor `PartialEq` for Number: NaN != NaN by default
- [ ] Gate `Eq` impl behind `js_nan_equality` feature
- [ ] Refactor bitwise ops: remove trait impls by default, add under `js_bitwise`
- [ ] Add explicit `Number::bitand()`, `Number::bitor()` etc. methods (always available)
- [ ] Refactor `FromStr`: error on empty string by default, gate JS behavior
- [ ] Update all internal tests to use `js_compat` feature where needed

Files touched:
- `Cargo.toml`
- `src/lib.rs` (OrderedNumber, feature re-exports)
- `src/traits.rs` (PartialEq, Eq, Hash changes)
- `src/ops/bitwise.rs` (conditional trait impls + explicit methods)
- `src/conversions.rs` (FromStr changes)
- `src/js_semantics.rs` (feature-gated)

---

### Phase 4: Fix Panicking Paths

**Outcome**: No `unimplemented!()` or `panic!()` in public API paths.

**Note**: Hash implementations are done in Phase 3. This phase handles remaining panics.

Steps:
- [ ] Audit for `unimplemented!()` - list all occurrences
- [ ] Implement `to_i32_js_coerce` for BigDecimal (truncate + wrap)
- [ ] Implement `to_i64_js_coerce` for BigDecimal (truncate + wrap)
- [ ] Implement `round_dp` for BigDecimal
- [ ] Implement `trunc` for BigDecimal
- [ ] Replace remaining `unimplemented!()` with documented limitations or proper errors

Known locations after Phase 3:
- `src/js_semantics.rs:107` - BigDecimal to_i32_js_coerce
- `src/js_semantics.rs:138` - BigDecimal to_i64_js_coerce
- (Hash for Rational/BigDecimal - fixed in Phase 3)

Files touched:
- `src/js_semantics.rs`
- `src/math.rs`
- `src/core.rs`

---

### Phase 5: Adversarial Test Suite

**Outcome**: Edge cases that reveal implementation limits are tested and documented.

Steps:
- [ ] Add `tests/bitwise.rs` - shift by 32, negative shifts, Infinity/NaN coercion
- [ ] Add `tests/overflow_boundaries.rs` - i64 boundary tests for Rational
- [ ] Add `tests/continued_fractions.rs` - CF algorithm adversarial inputs
- [ ] Add `tests/string_parsing.rs` - FromStr/Display roundtrip, edge cases
- [ ] Add `tests/associativity.rs` - operations that break associativity under overflow
- [ ] Add `tests/negative_zero.rs` - -0 preservation chains
- [ ] Add `tests/hash_consistency.rs` - equal values have equal hashes
- [ ] Add `tests/ordered_number.rs` - OrderedNumber in HashMap/HashSet

Files touched:
- `tests/bitwise.rs` (new)
- `tests/overflow_boundaries.rs` (new)
- `tests/continued_fractions.rs` (new)
- `tests/string_parsing.rs` (new)
- `tests/associativity.rs` (new)
- `tests/negative_zero.rs` (new)
- `tests/hash_consistency.rs` (new)
- `tests/ordered_number.rs` (new)

---

### Phase 6: API Polish & Release

**Outcome**: Clean public API, version bump, changelog.

Steps:
- [ ] Review public API surface - hide internal types
- [ ] Add rustdoc for all public items
- [ ] Create CHANGELOG.md with breaking changes section
- [ ] Update Cargo.toml version to 0.2.0
- [ ] Update Cargo.toml metadata (description, keywords, categories)
- [ ] Final `cargo test` (default features)
- [ ] Final `cargo test --all-features`
- [ ] Final `cargo clippy --all-features`

Files touched:
- `src/lib.rs`
- `Cargo.toml`
- `CHANGELOG.md` (new)

---

## Implementation Context

**Codebase maturity**: Evolving

The core arithmetic is solid (40/43 exact). We're now hardening the API boundaries and making architectural decisions that will persist.

**Type strategy**:
- Keep current internal types (NumericValue, ApproximationType)
- Add `OrderedNumber` as public wrapper for collection use
- Be careful about public API - these are harder to change

**Error handling**:
- Prefer returning `Result` or special values over panicking
- `unimplemented!()` is never acceptable in library code

**Conditional compilation pattern**:
```rust
// For trait impls that should only exist with feature
#[cfg(feature = "js_bitwise")]
impl BitAnd for Number { ... }

// For behavior differences within a function
fn parse_str(s: &str) -> Result<Number, Error> {
    if s.is_empty() {
        #[cfg(feature = "js_string_parse")]
        return Ok(Number::ZERO);
        #[cfg(not(feature = "js_string_parse"))]
        return Err(Error::EmptyString);
    }
    ...
}
```

**CI Testing**: Must test both configurations
```bash
cargo test                    # Pure mode
cargo test --features js_compat  # JS mode
```

## Risks

- **Breaking change for Eq users**: Mitigated by OrderedNumber wrapper + documentation
- **Test suite breakage**: Phase 2 audit will quantify; update tests to use features
- **BigDecimal implementation complexity**: Scoped to essentials only for 0.2

## Out of Scope

- Performance optimizations (save for 0.3)
- Additional language compatibility modes (Python, etc.)
- Symbolic simplification
- Serialization (serde support)
- BigDecimal transcendental functions (pow, sqrt, trig)
