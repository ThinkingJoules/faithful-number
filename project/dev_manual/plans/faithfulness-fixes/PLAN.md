# Plan: Faithfulness Fixes

## Goal

Eliminate all cases where faithful-number silently loses precision or behaves inconsistently, ensuring the library lives up to its name.

## Vision Alignment

From VISION.md:
> "**Explicit Approximation**: When exactness is lost (transcendental functions, overflow), we mark it explicitly via `ApproximationType`. Users can query `is_exact()` to know if precision was sacrificed."

This work directly addresses violations of this principle - cases where precision is lost without setting the approximation flag.

## Context

Code review identified multiple faithfulness violations:

**Silent Unfaithfulness (Critical)**:
1. Modulo on non-terminating rationals converts to Decimal, loses exactness, sets NO flag
2. String parsing never recovers Rationals - `"0.5"` stays Decimal
3. `floor`/`ceil`/`round` convert through f64 - silent precision loss for large values
4. `to_decimal()` for Rationals goes through f64 - loses precision silently

**Inconsistent Behavior**:
5. Constants `ZERO`/`ONE` are Decimal, but `Number::from(0)` is Rational
6. Hash may differ for equal values across representations (Rational vs Decimal)

**Design Gaps**:
7. BigDecimal division has no precision control
8. CF denominator limit inconsistency (10^9 vs i64::MAX in different paths)
9. No introspection method for users to understand Number state

## Approach

Fix each violation, prioritizing silent unfaithfulness first. Add an introspection API so users can understand any Number's state. Document the CF limit decision.

## Phases

### Phase 0: Create Acceptance Test File

**Outcome**: Test file exists and fails (test-driven development).

Steps:
- [ ] Create `tests/faithfulness_acceptance.rs` with all acceptance tests from acceptance.md
- [ ] Run `cargo test --test faithfulness_acceptance` to confirm tests fail
- [ ] Commit as baseline

Files:
- `tests/faithfulness_acceptance.rs` (new)

---

### Phase 1: Exact Rational Modulo

**Outcome**: `Rational % Rational` returns exact `Rational`, no precision loss.

Steps:
- [ ] Replace lines 968-976 in arithmetic.rs with exact rational modulo
- [ ] Formula: `(a/b) % (c/d) = ((a*d) % (b*c)) / (b*d)`
- [ ] Handle overflow by promoting to BigDecimal (with RationalApproximation flag)
- [ ] Add tests for exact modulo cases

Files likely touched:
- `src/ops/arithmetic.rs` (Rem impl for NumericValue, lines 964-1100)

---

### Phase 2: String Parsing Recovers Rationals

**Outcome**: Parsing `"0.5"` produces `Rational(1, 2)`, not `Decimal`.

Steps:
- [ ] At `src/conversions.rs` line 199, change `NumericValue::Decimal(d)` to `NumericValue::from_decimal(d)`
- [ ] `from_decimal()` already calls `try_decimal_to_rational()` internally (core.rs:67-74)
- [ ] Verify `"0.333..."` stays Decimal (non-terminating can't recover to exact rational)
- [ ] Add tests for terminating decimals becoming Rationals

Files likely touched:
- `src/conversions.rs` (FromStr impl, line 199)

---

### Phase 3: Exact floor/ceil/round/trunc

**Outcome**: Rounding operations work on native representations without f64 conversion.

Steps:
- [ ] Implement `floor` for Rational: `numer.div_floor(denom)`
- [ ] Implement `ceil` for Rational: `numer.div_ceil(denom)`
- [ ] Implement `round` for Rational: proper half-away-from-zero
- [ ] Implement `trunc` for Rational (already correct, verify)
- [ ] For Decimal/BigDecimal, use native methods (already correct)
- [ ] Add tests with large values that would overflow f64 mantissa

Files likely touched:
- `src/math.rs` (floor, ceil, round implementations)

---

### Phase 4: Exact to_decimal() for Rationals

**Outcome**: `to_decimal()` on Rational uses exact Decimal division, not f64 intermediate.

**Design Decision**: Non-terminating rationals (1/3) return `None`. Rationale: `to_decimal()` returns `Option<Decimal>`, implying exact conversion. A truncated 1/3 is not exact.

Steps:
- [ ] At `src/math.rs` lines 1166-1170, replace f64 conversion with direct Decimal arithmetic
- [ ] Check if rational is terminating first (use cached `is_term` flag from Rational tuple)
- [ ] If terminating: `Some(Decimal::from(numer) / Decimal::from(denom))`
- [ ] If non-terminating: `None`
- [ ] Add tests for both cases

Files likely touched:
- `src/math.rs` (to_decimal method, lines 1164-1176)

---

### Phase 5: Consistent Constants

**Outcome**: `Number::ZERO` and `Number::ONE` are `Rational`, matching `Number::from(0)`.

Steps:
- [ ] Change `ZERO` constant to `Rational(0/1, true)`
- [ ] Change `ONE` constant to `Rational(1/1, true)`
- [ ] Update `NumericValue::ZERO` and `NumericValue::ONE` similarly
- [ ] Verify no test breakage from representation change

Files likely touched:
- `src/core.rs` (constant definitions)

---

### Phase 6: Consistent Hashing

**Outcome**: Equal values hash equally regardless of representation.

**Target**: `Number::Hash` impl at `src/traits.rs` lines 207-259. (Note: `OrderedNumber` has separate Hash impl in `ordered.rs` which may need similar treatment.)

**Design Decision**: Use Option B - normalize to canonical rational form before hashing. This is O(1) for Rational, O(n) for Decimal (where n = digits), which is acceptable.

Steps:
- [ ] Analyze `Number::Hash` impl (traits.rs:207-259) for cross-representation inconsistencies
- [ ] Bug confirmed: Rational(1, 2) hashes (numer, denom), Decimal(0.5) hashes normalized decimal string
- [ ] Fix: For Decimal, attempt rational recovery (`try_decimal_to_rational`), hash that if successful
- [ ] If recovery fails, hash the normalized Decimal string (existing behavior)
- [ ] Add property test: `a == b` implies `hash(a) == hash(b)`
- [ ] Verify `OrderedNumber::Hash` doesn't have same issue (likely uses `Number::Hash` internally)

Files likely touched:
- `src/traits.rs` (Hash impl for Number, lines 207-259)

---

### Phase 7: BigDecimal Division Precision

**Outcome**: BigDecimal division specifies precision, flags when truncation occurs.

Steps:
- [ ] Research bigdecimal crate's precision control
- [ ] Set explicit precision for division (match Decimal's 28 digits?)
- [ ] Detect when result is truncated (repeating decimal)
- [ ] Set RationalApproximation flag when truncation occurs
- [ ] Add tests

Files likely touched:
- `src/ops/arithmetic.rs` (BigDecimal division)

---

### Phase 8: CF Limit Consistency + Decision Doc

**Outcome**: CF denominator limit is consistent and documented.

Steps:
- [ ] Unify limit: both `try_decimal_to_rational` (line 521) and `try_decimal_to_rational_bigdecimal` (line 654) use same limit
- [ ] Choose limit: keep 10^9 for performance (document why)
- [ ] Create decision doc explaining the trade-off
- [ ] Add code comment referencing the decision

Files likely touched:
- `src/core.rs` (rational recovery functions)
- `project/dev_manual/decisions/011-cf-denominator-limit.md` (new)

---

### Phase 9: Number Introspection API

**Outcome**: Users can inspect a Number's full state via a single method.

Steps:
- [ ] Design `NumberInfo` struct with: representation, approximation_type, is_exact, precision_bits (for transcendentals)
- [ ] Implement `Number::info(&self) -> NumberInfo`
- [ ] Include Display impl for NumberInfo for easy debugging
- [ ] Add doc examples

Files likely touched:
- `src/core.rs` (new NumberInfo struct and method)
- `src/lib.rs` (re-export)

---

## Implementation Context

**Codebase maturity**: Evolving - structure is settling, core types stable

**Type strategy**:
- NumberInfo should be a proper struct, not a tuple
- Approximation tracking is already well-typed

**Error handling**:
- Continue using Option for fallible conversions
- Approximation flags for precision loss (not errors)

**Notes for implementer**:
- The modulo fix (Phase 1) is mathematically straightforward but needs overflow handling
- Hash consistency (Phase 6) is subtle - ensure property tests cover edge cases
- The CF limit decision (Phase 8) should reference this plan's analysis

## Risks

- **Hash change breaks existing code**: Low risk since unpublished. Add migration note anyway.
- **Performance regression from exact operations**: Monitor with existing benchmarks. Exact modulo may be slower.
- **BigDecimal precision detection difficult**: May need to compare against arbitrary-precision result.

## Out of Scope

- Changing the Rational64 type to support larger denominators (would require different underlying type)
- Adding new approximation types beyond Transcendental/RationalApproximation
- Symbolic simplification (e.g., sqrt(4) before computing)

## Open Questions

- [x] ~~For `to_decimal()` on non-terminating rationals (1/3): return `None`, or return truncated value with flag?~~ **Resolved**: Return `None` (see Phase 4)
- [x] ~~For hash consistency: is O(n) normalization acceptable, or do we need O(1) approach?~~ **Resolved**: O(n) is acceptable (see Phase 6)
