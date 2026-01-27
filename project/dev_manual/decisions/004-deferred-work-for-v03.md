# TODO: Decision: Work Deferred to v0.3+

> **Tech Debt Note**: This documents intentionally deferred work. Items here should be addressed in v0.3+: BigDecimal transcendentals, serde support, performance optimizations, additional language modes. Review after v0.2 user feedback.

## Context

The v0.2 release plan had ambitious scope: feature flags, breaking changes, comprehensive tests, and fixing all panicking paths. As implementation progressed, we identified work that was important but not critical for v0.2.

From the plan's "Out of Scope" section:
- Performance optimizations
- Additional language compatibility modes (Python, etc.)
- Symbolic simplification
- Serialization (serde support)
- BigDecimal transcendental functions (pow, sqrt, trig)

The question: What is the minimum BigDecimal support needed for v0.2 to be usable?

From resolved questions:
> **Q: What's the minimum BigDecimal math support needed for 0.2?**
> **A: Implement `to_i32/i64`, `round_dp`, `trunc`, and `Hash`. Defer `pow`, `sqrt`, trig to 0.3.**

## Decisions Made

### Included in v0.2 (Essential)

1. **BigDecimal conversions** - No panics in public API
   - `to_i32()`, `to_i64()`, `to_u32()`, `to_u64()`
   - `to_i32_js_coerce()`, `to_i64_js_coerce()` (with js_bitwise feature)
   - These prevent `unimplemented!()` panics

2. **BigDecimal basic math** - Critical operations only
   - `round_dp()` - Needed for decimal place rounding
   - `trunc()` - Needed for integer conversion
   - `signum()` - Needed for comparison operations
   - `Hash` - Needed for collections (via OrderedNumber)

3. **Feature flag architecture** - Must be correct from start
   - js_nan_equality, js_bitwise, js_string_parse
   - Cannot easily change this later without more breaking changes

4. **Adversarial tests** - Understand implementation limits
   - Overflow boundaries, continued fractions, hash consistency
   - Better to know the limits now than discover in production

### Deferred to v0.3+ (Important but not blocking)

1. **BigDecimal transcendentals**
   - `pow()`, `sqrt()`, `sin()`, `cos()`, `tan()`, etc.
   - Reason: BigDecimal currently falls back to f64 for these (imprecise)
   - Proper implementation requires MPFR (via rug crate, which is optional)
   - Deferral impact: High-precision transcendentals degrade to f64 precision

2. **Performance optimizations**
   - Rational operations allocate frequently
   - Continued fractions algorithm could be tuned
   - Representation demotion heuristics could be smarter
   - Reason: Correctness before performance
   - Deferral impact: 4-10x slower than f64 (acceptable for v0.2)

3. **Serialization (serde)**
   - Would enable JSON, bincode, etc. support
   - Reason: Complex decision (serialize representation or value?)
   - Deferral impact: Users must implement custom serialization

4. **Additional language modes**
   - Python compatibility (truthy/falsy, floor division)
   - Ruby compatibility (integer division, type coercion)
   - Reason: JS mode establishes the pattern, others can follow
   - Deferral impact: Only JS compatibility in v0.2

5. **Symbolic simplification**
   - `(x + 1) - 1` could simplify back to `x`
   - `x / x` could simplify to `1`
   - Reason: Requires AST/expression tree, major architecture change
   - Deferral impact: Approximation flags accumulate unnecessarily

6. **Property-based testing**
   - Mentioned in original assessment, not implemented
   - Reason: Manual adversarial tests sufficient for v0.2
   - Deferral impact: May miss edge cases that proptest would find

## Trade-offs

### Why BigDecimal transcendentals were deferred

The implementation complexity vs. value trade-off:
- **Complexity**: Requires arbitrary-precision transcendentals (MPFR via rug)
- **Existing path**: Already falls back to f64 (works, just imprecise)
- **User impact**: Users needing high precision can enable `high_precision` feature
- **Timeline**: Would add significant testing burden to v0.2

Decision: Accept f64 precision for BigDecimal transcendentals in v0.2, implement properly in v0.3 when high_precision feature is more mature.

### Why serde was deferred

The design question is unsolved:
- Should we serialize the internal representation (Rational/Decimal/BigDecimal)?
- Or serialize the value (potentially losing representation information)?
- What about approximation flags?
- How to handle NaN, Infinity, -0 in various formats?

Decision: Don't rush this. Let users provide feedback on what they need.

## Consequences

### Enables (by deferring work)

- **v0.2 ships sooner**: Feature flags and tests are substantial work already
- **Learn from usage**: User feedback will guide v0.3 priorities
- **Avoid premature optimization**: Correctness established first, speed later
- **Clearer scope**: v0.2 = "ideological purity", v0.3 = "performance and features"

### Prevents/Complicates

- **Incomplete BigDecimal operations**: Some operations degrade to f64 precision
- **No built-in serialization**: Users must implement custom solutions
- **Performance sensitive uses**: May not be suitable for hot paths
- **Symbolic math users**: Cannot use library for CAS-style workloads

### Future Implications

- v0.3 roadmap is partially defined by these deferrals
- Pattern established: Correctness in early releases, optimization in later releases
- Breaking changes are acceptable pre-1.0, so we're not locked in
- User feedback between v0.2 and v0.3 will validate/invalidate these decisions

## Related

- Plan: v02-release (Out of Scope section)
- Files: `src/math.rs` (transcendental stubs), `README.md` (performance notes)
- For BigDecimal transcendentals: Enable `high_precision` feature (uses rug/MPFR)
