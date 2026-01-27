# Decision: JSON Format Uses Array ["value"] or ["value", "transcendental"]

## Context

The `serde_str` feature provides JSON serialization for `Number`. We needed to decide how to represent a number with its optional approximation metadata in JSON.

The core value is a string (not a JSON number) to preserve exactness - JSON's native number type is f64 which loses precision for values like 1/3.

## Options Considered

1. **Short keys: {"v": "...", "a": "..."}**
   - Pros: Minimal wire overhead
   - Cons: Keys aren't descriptive, unclear meaning

2. **Descriptive keys: {"value": "...", "approximate": "..."}**
   - Pros: Self-documenting
   - Cons: Verbose

3. **Array with optional second element: ["value"] or ["value", "approx"]**
   - Pros: Clean, position is the schema, no key overhead
   - Cons: Not self-describing (must know convention)

4. **String only (lose approximation info)**
   - Pros: Simple, interoperable
   - Cons: Loses fidelity - not faithful to the library's name/purpose

## Decision

We chose **Option 3: Array format** because:
- Position-based schema is cleaner than arbitrary short keys
- More compact than descriptive keys
- Preserves approximation metadata (staying "faithful")
- Common pattern in compact JSON serialization

User quote: "or do we use an array with optional index[1]? short keys aren't descriptive, might as well do no keys."

Examples:
```json
["0.5"]                                   // exact value
["1.4142135623730951", "transcendental"] // approximate
["0.333...", "rational_approximation"]   // approximate
```

## ApproximationType Strings

ApproximationType enum serializes as **lowercase** strings:
- `ApproximationType::Transcendental` → `"transcendental"`
- `ApproximationType::RationalApproximation` → `"rational_approximation"`

User confirmation: "3. lowercase"

This matches Rust convention for serde enum serialization (lowercase snake_case).

## Consequences

### Enables
- **Full fidelity**: Approximation metadata preserved on round-trip
- **Compact representation**: No key overhead, optional second element only when needed
- **Type safety**: Deserializer validates array length and approximation strings

### Prevents
- **Self-describing format**: Consumers must know the schema
- **Schema evolution**: Adding more metadata fields requires careful consideration
- **JSON interop**: External systems need to understand the array convention

### Future Implications
- If we add more metadata (precision hints, original representation type), we'd need to either:
  - Add more array elements (fragile)
  - Switch to object format (breaking change)
  - Create a v2 format with versioning

## Related

- Plan: v1-serialization (Phase 3)
- Files: `src/serde_impl.rs` (str_impl module)
- Decision: 005-mutual-exclusivity-of-serde-features.md
- User quote: "I think we can include the flag. That way we are truly faithful."
