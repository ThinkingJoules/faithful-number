# Decision: Regional Number Formats from compute-num-rs

## Context

The `format` feature adds rich display and parsing capabilities. Different regions format numbers differently:
- US: `1,234,567.89` (comma thousands, dot decimal)
- Europe: `1.234.567,89` (dot thousands, comma decimal)
- SI: `1 234 567.89` (space thousands, dot decimal)
- India: `12,34,567.89` (irregular grouping)

These formats are **ambiguous** when parsing - "1.234" could be 1234 (European) or 1.234 (US).

## Source

The display option types were adapted from `../compute-num-rs/src/display.rs`, which had well-designed type definitions but no implementation.

From compute-num-rs:
```rust
pub struct RegionalFormat {
    pub decimal_separator: char,
    pub thousands_separator: Option<char>,
    pub grouping_size: Option<u8>,
    pub secondary_grouping_size: Option<u8>,
}
```

The types were copied directly, including the presets: `us()`, `european()`, `si()`, `indian()`, and `plain()`.

User decision: "3. We have the other impl, we might as well just use it."

## Implementation

Four standard regional format presets:

1. **US/UK**: `RegionalFormat::us()`
   - Decimal: `.`
   - Thousands: `,`
   - Grouping: 3
   - Example: `1,234,567.89`

2. **European**: `RegionalFormat::european()`
   - Decimal: `,`
   - Thousands: `.`
   - Grouping: 3
   - Example: `1.234.567,89`

3. **SI (International System)**: `RegionalFormat::si()`
   - Decimal: `.`
   - Thousands: ` ` (space)
   - Grouping: 3
   - Example: `1 234 567.89`

4. **Indian**: `RegionalFormat::indian()`
   - Decimal: `.`
   - Thousands: `,`
   - Primary grouping: 3 (rightmost)
   - Secondary grouping: 2 (rest)
   - Example: `12,34,567.89` (not `1,234,567.89`)

5. **Plain**: `RegionalFormat::plain()`
   - Decimal: `.`
   - No thousands separator
   - Example: `1234567.89`

## Parsing Requires Explicit Format

Because formats are ambiguous, parsing **requires explicit `ParseOptions`**:

```rust
// ERROR: Can't parse without knowing format
let n = Number::parse_formatted("1.234", &ParseOptions::default())?;

// CORRECT: Specify the format
let us_opts = ParseOptions { regional_format: RegionalFormat::us() };
let n = Number::parse_formatted("1.234", &us_opts)?; // → 1234

let euro_opts = ParseOptions { regional_format: RegionalFormat::european() };
let n = Number::parse_formatted("1.234", &euro_opts)?; // → 1234.0
```

User insight: "we probably need parse options, as euro format might be ambiguous?"

## Decision

We adopted the **compute-num-rs format types** with full implementation of display and parsing.

Key choices:
- No "smart detection" of format - requires explicit config
- Round-trip invariant: `parse_formatted(format(n, opts), opts) == n`
- Separate from basic `Display`/`FromStr` (universal machine format)

## Consequences

### Enables
- **International support**: Numbers display correctly for different locales
- **Unambiguous parsing**: Explicit format prevents misinterpretation
- **Scientific notation**: `Notation::Scientific` and `Notation::Engineering`
- **Flexible grouping**: Supports irregular patterns like Indian format

### Prevents
- **Automatic locale detection**: Users must specify format explicitly
- **Mixed format parsing**: Can't parse "sometimes US, sometimes Euro" in same context

### Future Implications
- Could add more regional presets (Chinese, Arabic, etc.)
- Could add locale detection if OS/environment provides it
- The `ParseOptions` struct can grow to include other parsing hints (strict mode, allow hex, etc.)

## Related

- Plan: v1-serialization (Phases 1 & 2)
- Files: `src/format.rs`
- Source: `../compute-num-rs/src/display.rs`
- User quote: "feels like we are lacking string serialization in this plan? display is not the same unless we have a way to parse said string..."
