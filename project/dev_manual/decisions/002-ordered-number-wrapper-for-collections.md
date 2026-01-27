# Decision: OrderedNumber Wrapper for Collections

## Context

IEEE 754 requires `NaN != NaN`, which means `NaN == NaN` returns `false`. This violates the reflexivity property required by Rust's `Eq` trait (every value must equal itself).

Without `Eq`, `Number` cannot be used as a key in `HashMap` or `HashSet`. This is a practical problem even though it's mathematically correct.

The original plan asked: "Should we provide an `OrderedNumber` wrapper that impls Eq for HashMap use?"

## Options Considered

1. **Make Number impl Eq with NaN == NaN**
   - Pros: Simplest API, no wrapper needed
   - Cons: Violates IEEE 754, forces wrong behavior on all users
   - Used in v0.1, caused the "INTENTIONAL BUG" comment in code

2. **Remove Eq entirely, users figure it out**
   - Pros: Mathematically pure, no compromises
   - Cons: Cannot use Number in collections at all, poor UX

3. **OrderedNumber wrapper with total ordering**
   - Pros: Separate pure Number from collection-compatible wrapper
   - Cons: Users must explicitly wrap/unwrap, two types to understand

4. **Feature flag: Eq only with js_nan_equality**
   - Pros: Opt-in, users who need it can enable it
   - Cons: Doesn't help users who want pure defaults but also need collections

## Decision

We chose **both Option 3 and Option 4**: Provide `OrderedNumber` wrapper AND gate `Eq` trait behind `js_nan_equality` feature.

From the plan:
> **Q: Should we provide an `OrderedNumber` wrapper that impls Eq for HashMap use?**
> **A: YES. Provide `OrderedNumber` BEFORE removing `Eq` from `Number`. This is the migration path.**

This gives users two choices:
1. Use `OrderedNumber` wrapper (pure defaults, explicit collection semantics)
2. Enable `js_nan_equality` feature (JS semantics everywhere)

## Implementation Details

`OrderedNumber` implements total ordering:
- `NaN == NaN` (for Eq requirement)
- `NaN < -Infinity < finite < +Infinity` (for Ord requirement)
- Hash consistency: Equal values have equal hashes (including all zeros)

```rust
impl PartialEq for OrderedNumber {
    fn eq(&self, other: &Self) -> bool {
        match (self.0.value(), other.0.value()) {
            (NumericValue::NaN, NumericValue::NaN) => true, // For collections
            _ => self.0 == other.0,
        }
    }
}

impl Eq for OrderedNumber {}
```

## Consequences

### Enables

- **Pure defaults with practical collections**: Users don't have to choose between correctness and usability
- **Explicit intent**: Wrapping in `OrderedNumber` signals "I know NaN == NaN here"
- **Type safety**: `Number` and `OrderedNumber` are distinct types, can't accidentally mix semantics
- **Future-proof**: Can add other wrappers (e.g., `StrictNumber` with even stricter semantics)

### Prevents/Complicates

- **API surface increased**: Two types instead of one
- **Wrapping/unwrapping overhead**: Must convert at collection boundaries
- **Documentation complexity**: Must explain when to use which type
- **Deref convenience comes with footguns**: `OrderedNumber` derefs to `Number`, but equality semantics differ

### Future Implications

- Pattern established: Wrappers for different semantics, pure core type
- Could add `StrictNumber` (errors on any approximation), `JSNumber` (JS semantics), etc.
- Hash implementation must remain consistent with both `Number::PartialEq` and `OrderedNumber::Eq`

## Related

- Plan: v02-release (Phase 3)
- Files: `src/ordered.rs` (new), `src/lib.rs` (re-export)
- Decision: 001-pure-defaults-with-js-compat-features.md
- Tests: `tests/ordered_number.rs`
