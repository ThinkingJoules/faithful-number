# Decision: ParseError Enum with Descriptive Variants

## Context

The `format` feature adds `parse_formatted()` which parses human-formatted number strings (with regional formats, scientific notation, etc.). This is more complex than basic `FromStr` parsing and needs better error reporting.

The basic `FromStr` returns `Result<Number, ()>` which provides no information about what went wrong.

## Error Types Needed

From the plan discussion, we identified five error categories:

1. **Empty input**: `""` is not a valid number
2. **Invalid character**: Non-numeric characters like `"12abc34"`
3. **Multiple separators**: Malformed like `"1..5"` or `"1,,234"`
4. **Format mismatch**: Input doesn't match expected regional format
5. **Overflow**: Number too large to represent

## Decision

We defined a dedicated `ParseError` enum:

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    /// Input string is empty
    EmptyInput,

    /// Invalid character encountered
    InvalidCharacter {
        pos: usize,
        ch: char,
    },

    /// Multiple decimal points or thousands separators
    MultipleSeparators,

    /// Input doesn't match expected regional format
    MismatchedFormat,

    /// Number exceeds BigDecimal capacity
    Overflow,
}
```

User confirmation: "4. should probably do proper error handling?"

## Implementation Notes

The error provides context where possible:
- `InvalidCharacter` includes position and the problematic character
- Other variants are self-explanatory from name alone

The error implements standard traits:
- `Debug` for development
- `Clone` for error propagation
- `PartialEq, Eq` for testing

Could implement `std::error::Error` and `Display` in the future for better integration with error handling libraries (anyhow, thiserror, etc.).

## Decision

Use **descriptive enum variants** rather than string-based errors or error codes.

Benefits:
- Type-safe error handling
- Pattern matching in user code
- Good error messages for debugging
- Extensible (can add more variants later)

## Consequences

### Enables
- **Better error messages**: Users know exactly what went wrong
- **Type-safe handling**: Can match on specific error types
- **Debugging**: `InvalidCharacter` shows where parsing failed
- **Testing**: Can assert on specific error conditions

### Prevents
- **Generic error handling**: Can't just `?` propagate without context
- **Size overhead**: Enum larger than `()` or error code

### Future Implications
- If we add more parsing modes (hex floats, unicode digits), we may need more variants
- Could add error recovery hints (e.g., `MismatchedFormat` could suggest trying a different `RegionalFormat`)
- Could add `From<ParseError>` implementations for common error types

## Related

- Plan: v1-serialization (Phase 2)
- Files: `src/format.rs` (ParseError definition and usage)
- Related: 009-regional-format-presets.md (parsing with explicit formats)
