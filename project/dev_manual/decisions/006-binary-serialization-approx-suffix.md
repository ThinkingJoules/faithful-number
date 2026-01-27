# Decision: Binary Serialization Uses Approx Byte as Suffix

## Context

The `serde_bin` feature serializes `Number` using onenum's binary encoding. onenum provides a key property: **byte-by-byte lexicographical sorting matches mathematical numeric ordering**. This means you can sort the raw bytes and get correctly ordered numbers.

However, `Number` includes an `ApproximationType` field that onenum doesn't handle. We need to encode this metadata along with the value.

## Options Considered

1. **Prefix: [approx_byte][onenum_bytes]**
   - Pros: Natural to read "metadata first, then data"
   - Cons: **BREAKS SORTABILITY** - Numbers with approx=0 would sort before approx=1 regardless of value (e.g., exact 100 < approx 10)

2. **Suffix: [onenum_bytes][approx_byte]**
   - Pros: Preserves onenum's sortability guarantee - value bytes sort correctly, approx is just trailing metadata
   - Cons: Slightly less natural to read

3. **Separate fields via serde struct**
   ```rust
   #[serde(serialize_with = "...")]
   struct SerializedNumber {
       value: Vec<u8>,
       approx: Option<u8>,
   }
   ```
   - Pros: Clean separation
   - Cons: Adds serde struct overhead, loses raw byte sortability entirely (bincode adds struct framing)

## Decision

We chose **Option 2: Suffix [onenum_bytes][approx_byte]** because:
- Preserves onenum's sortability property
- The approximation metadata is logically "about" the value, not "before" it
- Enables database/index use cases where byte-ordered indexes work correctly
- Simple to implement: append one byte after onenum encoding

User quote: "suffix is good compromise. This keeps the bytes sortable"

The implementation:
```rust
// Encoding
let mut bytes = onum.as_bytes().to_vec();
let approx_byte = match &num.apprx {
    None => 0,
    Some(ApproximationType::Transcendental) => 1,
    Some(ApproximationType::RationalApproximation) => 2,
};
bytes.push(approx_byte);

// Decoding
let approx_byte = bytes.pop().unwrap();
let onum = Onum::from_bytes(&bytes)?;
```

## Consequences

### Enables
- **Sortable binary serialization**: Raw bytes can be sorted and produce mathematically correct ordering
- **Database indexes**: Can use the serialized bytes directly in B-tree or sorted indexes
- **Simple implementation**: No complex struct serialization

### Prevents
- Reading the format requires knowing the convention (approx is at the end)
- Slightly harder to debug/inspect (metadata is not self-describing)

### Future Implications
- If we add more metadata fields, they must also be suffixes to preserve sortability
- The order of suffix bytes must be carefully chosen to avoid breaking sort order
- Format is not self-describing - requires documentation

## Related

- Plan: v1-serialization (Phase 4)
- Files: `src/serde_impl.rs` (bin_impl module)
- Dependency: onenum crate
- Related decision: 005-mutual-exclusivity-of-serde-features.md
