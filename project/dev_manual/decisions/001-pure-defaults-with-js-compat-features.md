# Decision: Pure Defaults with JS Compatibility Features

## Context

In v0.1, the library had JavaScript semantics baked in as defaults:
- `NaN == NaN` returned `true` (violates IEEE 754)
- Bitwise operators had implicit ToInt32 coercion
- Empty string parsed to `0`

This created a philosophical problem: the library was tied to JavaScript's specific quirks rather than being a general-purpose exact arithmetic library. As the user stated: "I think JS is one of many compatibilities that we could maybe allow configuration for."

The question was: Should we maintain JS defaults for backward compatibility in v0.2, or break compatibility to establish ideological purity?

## Options Considered

1. **Keep JS defaults, add strict mode**
   - Pros: No breaking changes, easy migration from v0.1
   - Cons: Library remains JS-centric, "wrong" defaults for general use

2. **Pure defaults with JS opt-in features**
   - Pros: Ideologically correct, positions library as general-purpose
   - Cons: Breaking change for v0.1 users, more complex feature matrix

3. **Defer to v1.0**
   - Pros: More time to plan, could gather user feedback
   - Cons: Delays getting the architecture right, harder to change later

## Decision

We chose **pure defaults with JS opt-in features** because the user explicitly stated:

> "feature per behavior with a wrapper feature that enables all. I think we default to not having any js semantics. default should be ideologically pure as possible (outside of any single impl paradigm as much as possible). I want all of this in 0.2 push. it affects lots of things and we need to head in the right direction sooner than later."

This resulted in the feature flag architecture:

```toml
[features]
default = []

# Individual JS behaviors
js_nan_equality = []     # NaN == NaN returns true, enables Eq trait
js_bitwise = []          # Implicit ToInt32 coercion for &, |, ^, <<, >>
js_string_parse = []     # Empty string → 0, whitespace trimming

# Umbrella feature
js_compat = ["js_nan_equality", "js_bitwise", "js_string_parse"]
```

## Consequences

### Enables

- **Ideological purity**: Default behavior follows IEEE 754 and mathematical semantics
- **Language agnostic**: Foundation for future Python, Ruby, etc. compatibility modes
- **Granular control**: Users can opt-in to specific behaviors, not all-or-nothing
- **Clear documentation**: Breaking changes force us to document the "why" of each behavior

### Prevents/Complicates

- **v0.1 migration requires feature flag**: Users must add `features = ["js_compat"]`
- **Conditional compilation complexity**: Many `#[cfg(feature = "...")]` attributes throughout codebase
- **Test matrix explosion**: Must test default mode, js_compat mode, and all-features mode
- **Documentation burden**: Must explain each feature flag and when to use it

### Future Implications

- Sets precedent for other language compatibility modes (Python, Ruby, etc.)
- Establishes that "pure" is the default, languages are opt-in
- Feature flags are additive only (no feature should disable correctness)
- Breaking changes are acceptable for pre-1.0 releases when moving toward correctness

## Related

- Plan: v02-release
- Files: `Cargo.toml`, `src/traits.rs`, `src/ops/bitwise.rs`, `src/conversions.rs`
- User quote: "default should be ideologically pure as possible (outside of any single impl paradigm as much as possible)"
