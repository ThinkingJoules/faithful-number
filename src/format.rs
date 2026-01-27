//! Rich display formatting and parsing for Numbers.
//!
//! This module provides configurable formatting options for displaying numbers
//! in various regional and scientific formats, with round-trip parsing support.

use crate::Number;
use crate::core::NumericValue;

/// Exponential notation style
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExpNotation {
    /// Use 'e' notation (1.23e6)
    E,
    /// Use '×10^' notation (1.23×10^6)
    Times10,
}

/// Display notation options
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Notation {
    /// Standard decimal notation
    Decimal,
    /// Scientific notation (one digit before decimal)
    Scientific,
    /// Engineering notation (exponent is multiple of 3)
    Engineering,
}

/// Regional formatting preferences
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegionalFormat {
    /// Character used for decimal point ('.' or ',')
    pub decimal_separator: char,
    /// Character used for thousands separator (',', '.', ' ', or None)
    pub thousands_separator: Option<char>,
    /// Grouping size for digit separators (typically 3)
    pub grouping_size: Option<u8>,
    /// Secondary grouping size (for formats like Indian 1,23,45,678)
    pub secondary_grouping_size: Option<u8>,
}

/// Display options for formatting numbers
#[derive(Debug, Clone)]
pub struct DisplayOptions {
    /// Number of decimal places (None = all significant digits)
    pub decimal_places: Option<u8>,
    /// Number of significant figures for scientific/engineering notation
    pub significant_figures: Option<u8>,
    /// Type of notation
    pub notation: Notation,
    /// Exponential notation style (for Scientific/Engineering)
    pub exp_notation: ExpNotation,
    /// Regional formatting
    pub regional_format: RegionalFormat,
}

impl Default for RegionalFormat {
    fn default() -> Self {
        RegionalFormat::plain()
    }
}

impl Default for DisplayOptions {
    fn default() -> Self {
        DisplayOptions {
            decimal_places: None,
            significant_figures: None,
            notation: Notation::Decimal,
            exp_notation: ExpNotation::E,
            regional_format: RegionalFormat::default(),
        }
    }
}

impl RegionalFormat {
    /// US/UK format (dot for decimal, comma for thousands)
    pub fn us() -> Self {
        RegionalFormat {
            decimal_separator: '.',
            thousands_separator: Some(','),
            grouping_size: Some(3),
            secondary_grouping_size: None,
        }
    }

    /// European format (comma for decimal, dot for thousands)
    pub fn european() -> Self {
        RegionalFormat {
            decimal_separator: ',',
            thousands_separator: Some('.'),
            grouping_size: Some(3),
            secondary_grouping_size: None,
        }
    }

    /// SI format (dot for decimal, space for thousands)
    pub fn si() -> Self {
        RegionalFormat {
            decimal_separator: '.',
            thousands_separator: Some(' '),
            grouping_size: Some(3),
            secondary_grouping_size: None,
        }
    }

    /// Indian format (1,23,45,678)
    pub fn indian() -> Self {
        RegionalFormat {
            decimal_separator: '.',
            thousands_separator: Some(','),
            grouping_size: Some(3),
            secondary_grouping_size: Some(2),
        }
    }

    /// No separators (plain format)
    pub fn plain() -> Self {
        RegionalFormat {
            decimal_separator: '.',
            thousands_separator: None,
            grouping_size: None,
            secondary_grouping_size: None,
        }
    }
}

impl DisplayOptions {
    /// Standard decimal display (same as default)
    pub fn standard() -> Self {
        Self::default()
    }

    /// Scientific notation with e-notation
    pub fn scientific() -> Self {
        DisplayOptions {
            notation: Notation::Scientific,
            exp_notation: ExpNotation::E,
            significant_figures: Some(6),
            ..Default::default()
        }
    }

    /// Scientific notation with ×10^ notation
    pub fn scientific_times() -> Self {
        DisplayOptions {
            notation: Notation::Scientific,
            exp_notation: ExpNotation::Times10,
            significant_figures: Some(6),
            ..Default::default()
        }
    }

    /// Engineering notation (powers of 1000)
    pub fn engineering() -> Self {
        DisplayOptions {
            notation: Notation::Engineering,
            exp_notation: ExpNotation::E,
            significant_figures: Some(6),
            ..Default::default()
        }
    }

    /// US regional format with decimal notation
    pub fn us() -> Self {
        DisplayOptions {
            regional_format: RegionalFormat::us(),
            ..Default::default()
        }
    }

    /// European regional format with decimal notation
    pub fn european() -> Self {
        DisplayOptions {
            regional_format: RegionalFormat::european(),
            ..Default::default()
        }
    }

    /// SI regional format with decimal notation
    pub fn si() -> Self {
        DisplayOptions {
            regional_format: RegionalFormat::si(),
            ..Default::default()
        }
    }

    /// Indian regional format with decimal notation
    pub fn indian() -> Self {
        DisplayOptions {
            regional_format: RegionalFormat::indian(),
            ..Default::default()
        }
    }
}

impl Number {
    /// Format the number according to the given display options.
    pub fn format(&self, opts: &DisplayOptions) -> String {
        // Handle special values first
        match &self.value {
            NumericValue::NaN => return "NaN".to_string(),
            NumericValue::PositiveInfinity => return "Infinity".to_string(),
            NumericValue::NegativeInfinity => return "-Infinity".to_string(),
            NumericValue::NegativeZero => {
                // -0 displays as "0" but we need to handle formatting
                return format_zero(opts);
            }
            _ => {}
        }

        // Get the string representation of the number
        let raw = self.to_string();

        // Check if negative
        let (is_negative, raw) = if let Some(rest) = raw.strip_prefix('-') {
            (true, rest)
        } else {
            (false, raw.as_str())
        };

        match opts.notation {
            Notation::Decimal => format_decimal(raw, is_negative, opts),
            Notation::Scientific => format_scientific(raw, is_negative, opts),
            Notation::Engineering => format_engineering(raw, is_negative, opts),
        }
    }
}

fn format_zero(opts: &DisplayOptions) -> String {
    match opts.notation {
        Notation::Decimal => {
            if let Some(dp) = opts.decimal_places {
                if dp > 0 {
                    let zeros: String = "0".repeat(dp as usize);
                    format!("0{}{}", opts.regional_format.decimal_separator, zeros)
                } else {
                    "0".to_string()
                }
            } else {
                "0".to_string()
            }
        }
        Notation::Scientific | Notation::Engineering => {
            let sig_figs = opts.significant_figures.unwrap_or(6) as usize;
            let zeros = if sig_figs > 1 {
                format!("{}0", opts.regional_format.decimal_separator)
            } else {
                String::new()
            };
            match opts.exp_notation {
                ExpNotation::E => format!("0{}e0", zeros),
                ExpNotation::Times10 => format!("0{}×10^0", zeros),
            }
        }
    }
}

fn format_decimal(raw: &str, is_negative: bool, opts: &DisplayOptions) -> String {
    // Split into integer and fractional parts
    let (int_part, frac_part) = if let Some(dot_pos) = raw.find('.') {
        (&raw[..dot_pos], Some(&raw[dot_pos + 1..]))
    } else {
        (raw, None)
    };

    // Apply decimal places limit if specified
    let frac_part = if let Some(dp) = opts.decimal_places {
        if dp == 0 {
            None
        } else if let Some(frac) = frac_part {
            if frac.len() > dp as usize {
                Some(&frac[..dp as usize])
            } else {
                Some(frac)
            }
        } else {
            None
        }
    } else {
        frac_part
    };

    // Format integer part with grouping
    let formatted_int = format_integer_with_grouping(int_part, &opts.regional_format);

    // Build result
    let mut result = String::new();
    if is_negative {
        result.push('-');
    }
    result.push_str(&formatted_int);

    if let Some(frac) = frac_part
        && !frac.is_empty()
    {
        result.push(opts.regional_format.decimal_separator);
        result.push_str(frac);
    }

    result
}

fn format_integer_with_grouping(int_str: &str, fmt: &RegionalFormat) -> String {
    let sep = match fmt.thousands_separator {
        Some(s) => s,
        None => return int_str.to_string(),
    };

    let group_size = match fmt.grouping_size {
        Some(g) if g > 0 => g as usize,
        _ => return int_str.to_string(),
    };

    let secondary_size = fmt.secondary_grouping_size.map(|g| g as usize);

    let chars: Vec<char> = int_str.chars().collect();
    let len = chars.len();

    if len <= group_size {
        return int_str.to_string();
    }

    let mut result = String::with_capacity(len + len / group_size);
    let mut pos = 0;

    // Calculate positions for separators from right to left
    let mut separator_positions = Vec::new();
    let mut remaining = len;

    // First group from the right
    if remaining > group_size {
        remaining -= group_size;
        separator_positions.push(remaining);

        // Subsequent groups use secondary size if available
        let subsequent_size = secondary_size.unwrap_or(group_size);
        while remaining > subsequent_size {
            remaining -= subsequent_size;
            separator_positions.push(remaining);
        }
    }

    separator_positions.reverse();

    for (i, ch) in chars.iter().enumerate() {
        if separator_positions.contains(&i) && pos > 0 {
            result.push(sep);
        }
        result.push(*ch);
        pos += 1;
    }

    result
}

fn format_scientific(raw: &str, is_negative: bool, opts: &DisplayOptions) -> String {
    let (mantissa, exponent) = to_scientific_parts(raw);
    format_exp_notation(mantissa, exponent, is_negative, opts)
}

fn format_engineering(raw: &str, is_negative: bool, opts: &DisplayOptions) -> String {
    let (mantissa, exponent) = to_scientific_parts(raw);

    // Adjust to make exponent a multiple of 3
    let exp_mod = exponent.rem_euclid(3);
    let adjusted_exp = exponent - exp_mod;

    // Shift decimal point in mantissa
    let adjusted_mantissa = shift_mantissa(&mantissa, exp_mod as usize);

    format_exp_notation(adjusted_mantissa, adjusted_exp, is_negative, opts)
}

fn to_scientific_parts(raw: &str) -> (String, i32) {
    // Handle "0" case
    if raw == "0" {
        return ("0".to_string(), 0);
    }

    let (int_part, frac_part) = if let Some(dot_pos) = raw.find('.') {
        (&raw[..dot_pos], &raw[dot_pos + 1..])
    } else {
        (raw, "")
    };

    // Check if already in scientific notation
    if let Some(e_pos) = raw.to_lowercase().find('e') {
        let mantissa = &raw[..e_pos];
        let exp: i32 = raw[e_pos + 1..].parse().unwrap_or(0);
        return (mantissa.to_string(), exp);
    }

    // Find first non-zero digit
    let all_digits: String = format!("{}{}", int_part, frac_part);
    let first_nonzero = all_digits.chars().position(|c| c != '0');

    match first_nonzero {
        None => ("0".to_string(), 0), // All zeros
        Some(pos) => {
            let significant: String = all_digits.chars().skip(pos).collect();

            // Calculate exponent
            let int_len = int_part.len();
            let exponent = if int_part == "0" || int_part.is_empty() {
                // Number is < 1, exponent is negative
                -(pos as i32 - int_len as i32 + 1)
            } else {
                // Number >= 1
                (int_len as i32) - (pos as i32) - 1
            };

            // Format mantissa with decimal after first digit
            let mantissa = if significant.len() > 1 {
                format!("{}.{}", &significant[..1], &significant[1..])
            } else {
                significant
            };

            (mantissa, exponent)
        }
    }
}

fn shift_mantissa(mantissa: &str, shift: usize) -> String {
    if shift == 0 {
        return mantissa.to_string();
    }

    let (int_part, frac_part) = if let Some(dot_pos) = mantissa.find('.') {
        (&mantissa[..dot_pos], &mantissa[dot_pos + 1..])
    } else {
        (mantissa, "")
    };

    let all_digits: String = format!("{}{}", int_part, frac_part);
    let new_int_len = int_part.len() + shift;

    if new_int_len >= all_digits.len() {
        // Pad with zeros if needed
        let padded = format!(
            "{}{}",
            all_digits,
            "0".repeat(new_int_len - all_digits.len())
        );
        padded
    } else {
        format!(
            "{}.{}",
            &all_digits[..new_int_len],
            &all_digits[new_int_len..]
        )
    }
}

fn format_exp_notation(
    mantissa: String,
    exponent: i32,
    is_negative: bool,
    opts: &DisplayOptions,
) -> String {
    // Apply significant figures
    let mantissa = if let Some(sig_figs) = opts.significant_figures {
        truncate_to_sig_figs(
            &mantissa,
            sig_figs as usize,
            opts.regional_format.decimal_separator,
        )
    } else {
        mantissa.replace('.', &opts.regional_format.decimal_separator.to_string())
    };

    let sign = if is_negative { "-" } else { "" };

    match opts.exp_notation {
        ExpNotation::E => format!("{}{}e{}", sign, mantissa, exponent),
        ExpNotation::Times10 => format!("{}{}×10^{}", sign, mantissa, exponent),
    }
}

fn truncate_to_sig_figs(mantissa: &str, sig_figs: usize, decimal_sep: char) -> String {
    let digits: String = mantissa.chars().filter(|c| c.is_ascii_digit()).collect();

    if digits.len() <= sig_figs {
        return mantissa.replace('.', &decimal_sep.to_string());
    }

    // Find decimal position
    let dot_pos = mantissa.find('.').unwrap_or(mantissa.len());

    let truncated_digits: String = digits.chars().take(sig_figs).collect();

    if dot_pos >= sig_figs {
        truncated_digits
    } else {
        format!(
            "{}{}{}",
            &truncated_digits[..dot_pos],
            decimal_sep,
            &truncated_digits[dot_pos..]
        )
    }
}

// ============================================================================
// Parsing
// ============================================================================

/// Error type for formatted number parsing
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    /// Input string was empty
    EmptyInput,
    /// Invalid character found at position
    InvalidCharacter { pos: usize, ch: char },
    /// Multiple decimal separators or other separator issues
    MultipleSeparators,
    /// Input doesn't match expected regional format
    MismatchedFormat,
    /// Number exceeds representable range
    Overflow,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::EmptyInput => write!(f, "empty input"),
            ParseError::InvalidCharacter { pos, ch } => {
                write!(f, "invalid character '{}' at position {}", ch, pos)
            }
            ParseError::MultipleSeparators => write!(f, "multiple separators"),
            ParseError::MismatchedFormat => write!(f, "input doesn't match expected format"),
            ParseError::Overflow => write!(f, "number exceeds representable range"),
        }
    }
}

impl std::error::Error for ParseError {}

/// Options for parsing formatted numbers
#[derive(Debug, Clone)]
pub struct ParseOptions {
    /// Regional format to expect
    pub regional_format: RegionalFormat,
    /// Whether to allow scientific notation (e.g., 1.23e6)
    pub allow_scientific: bool,
}

impl Default for ParseOptions {
    fn default() -> Self {
        ParseOptions {
            regional_format: RegionalFormat::plain(),
            allow_scientific: true,
        }
    }
}

impl ParseOptions {
    /// Parse options matching US format
    pub fn us() -> Self {
        ParseOptions {
            regional_format: RegionalFormat::us(),
            allow_scientific: true,
        }
    }

    /// Parse options matching European format
    pub fn european() -> Self {
        ParseOptions {
            regional_format: RegionalFormat::european(),
            allow_scientific: true,
        }
    }

    /// Parse options matching SI format
    pub fn si() -> Self {
        ParseOptions {
            regional_format: RegionalFormat::si(),
            allow_scientific: true,
        }
    }

    /// Parse options matching Indian format
    pub fn indian() -> Self {
        ParseOptions {
            regional_format: RegionalFormat::indian(),
            allow_scientific: true,
        }
    }
}

impl Number {
    /// Parse a formatted string into a Number.
    ///
    /// This is the inverse of `format()` - it can parse strings produced by
    /// `format()` with the same regional options.
    pub fn parse_formatted(s: &str, opts: &ParseOptions) -> Result<Number, ParseError> {
        let s = s.trim();

        if s.is_empty() {
            return Err(ParseError::EmptyInput);
        }

        // Handle special values
        match s {
            "NaN" => return Ok(Number::NAN),
            "Infinity" => return Ok(Number::POSITIVE_INFINITY),
            "-Infinity" => return Ok(Number::NEGATIVE_INFINITY),
            _ => {}
        }

        // Handle sign
        let (is_negative, s) = if let Some(rest) = s.strip_prefix('-') {
            (true, rest)
        } else if let Some(rest) = s.strip_prefix('+') {
            (false, rest)
        } else {
            (false, s)
        };

        // Check for scientific notation
        let (mantissa_str, exponent) = if opts.allow_scientific {
            parse_scientific_notation(s, &opts.regional_format)?
        } else {
            (s.to_string(), 0i32)
        };

        // Parse the mantissa
        let normalized = normalize_regional_format(&mantissa_str, &opts.regional_format)?;

        // Parse as Number
        let mut num: Number = normalized.parse().map_err(|_| ParseError::Overflow)?;

        // Apply exponent if any
        if exponent != 0 {
            let exp_multiplier = Number::from(10.0).pow(Number::from(exponent));
            num *= exp_multiplier;
        }

        // Apply sign
        if is_negative {
            num = -num;
        }

        Ok(num)
    }
}

fn parse_scientific_notation(s: &str, fmt: &RegionalFormat) -> Result<(String, i32), ParseError> {
    // Check for ×10^ notation first
    if let Some(pos) = s.find("×10^") {
        let mantissa = &s[..pos];
        let exp_str = &s[pos + "×10^".len()..];
        let exponent: i32 = exp_str.parse().map_err(|_| ParseError::MismatchedFormat)?;
        return Ok((mantissa.to_string(), exponent));
    }

    // Check for e/E notation
    let lower = s.to_lowercase();
    if let Some(e_pos) = lower.find('e') {
        // Make sure 'e' is not the decimal separator (unlikely but check)
        if fmt.decimal_separator == 'e' || fmt.decimal_separator == 'E' {
            return Ok((s.to_string(), 0));
        }

        let mantissa = &s[..e_pos];
        let exp_str = &s[e_pos + 1..];
        let exponent: i32 = exp_str.parse().map_err(|_| ParseError::MismatchedFormat)?;
        return Ok((mantissa.to_string(), exponent));
    }

    Ok((s.to_string(), 0))
}

fn normalize_regional_format(s: &str, fmt: &RegionalFormat) -> Result<String, ParseError> {
    let mut result = String::with_capacity(s.len());
    let mut decimal_seen = false;

    for (pos, ch) in s.chars().enumerate() {
        if ch.is_ascii_digit() {
            result.push(ch);
        } else if ch == fmt.decimal_separator {
            if decimal_seen {
                return Err(ParseError::MultipleSeparators);
            }
            decimal_seen = true;
            result.push('.');
        } else if Some(ch) == fmt.thousands_separator {
            // Skip thousands separators (they're just visual)
            continue;
        } else if ch == '-' || ch == '+' {
            // Sign should have been handled already
            return Err(ParseError::InvalidCharacter { pos, ch });
        } else {
            return Err(ParseError::InvalidCharacter { pos, ch });
        }
    }

    if result.is_empty() {
        return Err(ParseError::EmptyInput);
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    mod display_tests {
        use super::*;

        #[test]
        fn default_matches_display() {
            // Use integer to avoid f64 precision issues
            let n = Number::from(123456);
            let formatted = n.format(&DisplayOptions::default());
            let displayed = n.to_string();
            assert_eq!(formatted, displayed);
        }

        #[test]
        fn us_format() {
            // Use FromStr to get exact decimal representation
            let n = Number::from_str("1234567.89").unwrap();
            let formatted = n.format(&DisplayOptions::us());
            assert_eq!(formatted, "1,234,567.89");
        }

        #[test]
        fn european_format() {
            let n = Number::from_str("1234567.89").unwrap();
            let formatted = n.format(&DisplayOptions::european());
            assert_eq!(formatted, "1.234.567,89");
        }

        #[test]
        fn si_format() {
            let n = Number::from_str("1234567.89").unwrap();
            let formatted = n.format(&DisplayOptions::si());
            assert_eq!(formatted, "1 234 567.89");
        }

        #[test]
        fn indian_format() {
            let n = Number::from_str("12345678.9").unwrap();
            let formatted = n.format(&DisplayOptions::indian());
            assert_eq!(formatted, "1,23,45,678.9");
        }

        #[test]
        fn scientific_notation() {
            let n = Number::from(1234567);
            let formatted = n.format(&DisplayOptions::scientific());
            assert!(formatted.contains("e"), "formatted: {}", formatted);
            assert!(formatted.starts_with("1.23456"), "formatted: {}", formatted);
        }

        #[test]
        fn special_values() {
            assert_eq!(Number::NAN.format(&DisplayOptions::default()), "NaN");
            assert_eq!(
                Number::POSITIVE_INFINITY.format(&DisplayOptions::default()),
                "Infinity"
            );
            assert_eq!(
                Number::NEGATIVE_INFINITY.format(&DisplayOptions::default()),
                "-Infinity"
            );
        }

        #[test]
        fn negative_numbers() {
            let n = Number::from_str("-1234.56").unwrap();
            let formatted = n.format(&DisplayOptions::us());
            assert_eq!(formatted, "-1,234.56");
        }

        #[test]
        fn integer_us_format() {
            let n = Number::from(1234567);
            let formatted = n.format(&DisplayOptions::us());
            assert_eq!(formatted, "1,234,567");
        }

        #[test]
        fn small_numbers_no_grouping() {
            let n = Number::from(123);
            let formatted = n.format(&DisplayOptions::us());
            assert_eq!(formatted, "123");
        }
    }

    mod parse_tests {
        use super::*;

        #[test]
        fn parse_us_format() {
            let n = Number::parse_formatted("1,234,567.89", &ParseOptions::us()).unwrap();
            let expected = Number::from_str("1234567.89").unwrap();
            assert_eq!(n, expected);
        }

        #[test]
        fn parse_european_format() {
            let n = Number::parse_formatted("1.234.567,89", &ParseOptions::european()).unwrap();
            let expected = Number::from_str("1234567.89").unwrap();
            assert_eq!(n, expected);
        }

        #[test]
        fn parse_scientific() {
            let n = Number::parse_formatted("1.23e6", &ParseOptions::default()).unwrap();
            assert_eq!(n, Number::from(1230000));
        }

        #[test]
        fn parse_special_values() {
            assert!(
                Number::parse_formatted("NaN", &ParseOptions::default())
                    .unwrap()
                    .is_nan()
            );
            assert!(
                Number::parse_formatted("Infinity", &ParseOptions::default())
                    .unwrap()
                    .is_positive_infinity()
            );
            assert!(
                Number::parse_formatted("-Infinity", &ParseOptions::default())
                    .unwrap()
                    .is_negative_infinity()
            );
        }

        #[test]
        fn parse_empty_error() {
            assert_eq!(
                Number::parse_formatted("", &ParseOptions::default()),
                Err(ParseError::EmptyInput)
            );
        }

        #[test]
        fn parse_invalid_char() {
            let result = Number::parse_formatted("12abc34", &ParseOptions::default());
            assert!(matches!(result, Err(ParseError::InvalidCharacter { .. })));
        }

        #[test]
        fn roundtrip_us() {
            let original = Number::from_str("1234567.89").unwrap();
            let formatted = original.format(&DisplayOptions::us());
            let parsed = Number::parse_formatted(&formatted, &ParseOptions::us()).unwrap();
            assert_eq!(original, parsed);
        }

        #[test]
        fn roundtrip_european() {
            let original = Number::from_str("1234567.89").unwrap();
            let formatted = original.format(&DisplayOptions::european());
            let parsed = Number::parse_formatted(&formatted, &ParseOptions::european()).unwrap();
            assert_eq!(original, parsed);
        }

        #[test]
        fn roundtrip_integer() {
            let original = Number::from(1234567);
            let formatted = original.format(&DisplayOptions::us());
            let parsed = Number::parse_formatted(&formatted, &ParseOptions::us()).unwrap();
            assert_eq!(original, parsed);
        }

        #[test]
        fn roundtrip_scientific() {
            let original = Number::from(1234567);
            let opts = DisplayOptions::scientific();
            let formatted = original.format(&opts);
            let parsed = Number::parse_formatted(&formatted, &ParseOptions::default()).unwrap();
            // Scientific notation with 6 sig figs: 1.23456e6 = 1234560
            // So we expect a small diff
            let diff = (original.to_f64() - parsed.to_f64()).abs();
            assert!(diff < 10.0, "diff was {}", diff);
        }
    }
}
