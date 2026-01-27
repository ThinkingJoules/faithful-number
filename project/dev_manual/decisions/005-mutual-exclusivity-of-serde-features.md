# Decision: Mutual Exclusivity of serde_str and serde_bin via cfg(not()) Pattern

## Context

The v1-serialization plan added two serialization features:
- `serde_str`: String-based serialization (JSON, TOML)
- `serde_bin`: Binary serialization via onenum (bincode, postcard)

Both features implement the same serde `Serialize` and `Deserialize` traits on `Number`. If both features are enabled simultaneously, Rust will error with "conflicting implementations of trait."

## Options Considered

1. **compile_error! macro**:
   ```rust
   #[cfg(all(feature = "serde_str", feature = "serde_bin"))]
   compile_error!("Features 'serde_str' and 'serde_bin' are mutually exclusive");
   ```
   - Pros: Clear, explicit error message explaining the conflict
   - Cons: Extra boilerplate code

2. **Precedence (one wins silently)**:
   ```rust
   #[cfg(feature = "serde_str")]
   impl Serialize for Number { /* string */ }

   #[cfg(all(feature = "serde_bin", not(feature = "serde_str")))]
   impl Serialize for Number { /* binary */ }
   ```
   - Pros: Always compiles
   - Cons: Silent behavior - users don't know serde_bin is being ignored

3. **Both require exclusivity (cfg(not()) pattern)**:
   ```rust
   #[cfg(all(feature = "serde_str", not(feature = "serde_bin")))]
   impl Serialize for Number { /* string */ }

   #[cfg(all(feature = "serde_bin", not(feature = "serde_str")))]
   impl Serialize for Number { /* binary */ }
   ```
   - Pros: Symmetric, no "magic", simplest code
   - Cons: Error message less clear ("trait not implemented" vs explicit conflict)

## Decision

We chose **Option 3: cfg(not()) pattern** because:
- It's symmetric - both features explicitly require the other to be absent
- No magic precedence rules
- Simpler code with no extra error checking boilerplate
- If users enable both features, they get a standard Rust error that `Number` doesn't implement `Serialize`, which prompts them to check their feature flags

User quote: "can't we just use the cfg not and cfg tags together to create mutual exclusivity? shouldn't need compile error?"

## Consequences

### Enables
- Clean, symmetric feature implementation
- No boilerplate compile_error! checks
- Standard Rust compilation errors guide users

### Prevents
- Cannot catch the misconfiguration with a custom error message
- Users must interpret "trait not implemented" to realize they enabled conflicting features

### Future Implications
- This pattern can be reused for other mutually exclusive features
- Documentation in Cargo.toml and README must clearly state the mutual exclusivity
- If we add a third serialization format (e.g., `serde_msgpack`), we'll need to update all cfg conditions

## Related

- Plan: v1-serialization
- Files: `src/serde_impl.rs` (lines 14, 106)
- Also applies to: Any future mutually exclusive feature pairs
