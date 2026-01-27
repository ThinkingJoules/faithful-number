use bigdecimal::BigDecimal;
use num_rational::{Ratio, Rational64};
use rust_decimal::Decimal;

/// Maximum denominator for continued fractions rational recovery.
/// Set to 10^9 to ensure arithmetic safety: two rationals with denominators
/// up to 10^9 can multiply without overflow (10^9 × 10^9 = 10^18 < i64::MAX).
/// See: project/dev_manual/decisions/011-cf-denominator-limit.md
const CF_MAX_DENOM: i64 = 1_000_000_000;

/// A smart number type that supports multiple internal representations
/// with automatic upgrades for precision and proper handling of IEEE special values
#[derive(Debug, Clone)]
pub(crate) enum NumericValue {
    /// Exact rational number (e.g., 1/3, 2/7) with cached terminating flag
    /// The bool indicates if this is a terminating decimal (can be exactly represented in base 10)
    Rational(Rational64, bool),
    /// Fixed-point decimal with 28 significant digits (renamed from Finite)
    Decimal(Decimal),
    /// Arbitrary precision decimal for very large numbers
    BigDecimal(BigDecimal),
    /// IEEE NaN (Not a Number)
    NaN,
    /// IEEE positive infinity
    PositiveInfinity,
    /// IEEE negative infinity
    NegativeInfinity,
    /// IEEE negative zero (distinct from positive zero)
    NegativeZero,
}

impl NumericValue {
    // Constants
    // pub const NAN: NumericValue = NumericValue::NaN;
    pub const POSITIVE_INFINITY: NumericValue = NumericValue::PositiveInfinity;
    pub const NEGATIVE_INFINITY: NumericValue = NumericValue::NegativeInfinity;
    // ZERO and ONE are Rational for consistency with Number::from(0) and Number::from(1)
    // Note: Not const because Ratio::new is not const-compatible
    #[inline]
    pub fn zero() -> NumericValue {
        NumericValue::Rational(Ratio::from_integer(0), true)
    }
    #[inline]
    pub fn one() -> NumericValue {
        NumericValue::Rational(Ratio::from_integer(1), true)
    }

    // Constructors for Decimal
    pub fn new(num: i64, scale: u32) -> Self {
        Self::Decimal(Decimal::new(num, scale))
    }

    pub const fn new_uint(num: u32) -> Self {
        Self::Decimal(Decimal::from_parts(num, 0, 0, false, 0))
    }

    pub fn try_from_i128_with_scale(num: i128, scale: u32) -> Result<Self, rust_decimal::Error> {
        match Decimal::try_from_i128_with_scale(num, scale) {
            Ok(d) => Ok(Self::Decimal(d)),
            Err(rust_decimal::Error::ExceedsMaximumPossibleValue) => {
                // Fall back to BigDecimal for values that exceed Decimal capacity
                use bigdecimal::BigDecimal;
                let bd = if scale == 0 {
                    BigDecimal::from(num)
                } else {
                    BigDecimal::from(num) / BigDecimal::from(10i128.pow(scale))
                };
                Ok(Self::BigDecimal(bd))
            }
            Err(e) => Err(e),
        }
    }

    // Constructors for new numeric types
    pub fn from_rational(r: Rational64) -> Self {
        let is_term = is_terminating_decimal(*r.numer(), *r.denom());
        NumericValue::Rational(r, is_term)
    }

    pub fn from_decimal(d: Decimal) -> Self {
        // Try to downgrade to Rational first
        if let Some(r) = try_decimal_to_rational(d) {
            let is_term = is_terminating_decimal(*r.numer(), *r.denom());
            NumericValue::Rational(r, is_term)
        } else {
            NumericValue::Decimal(d)
        }
    }

    pub fn from_bigdecimal(bd: BigDecimal) -> Self {
        // Try to downgrade to Decimal first, then Rational
        if let Some(d) = try_bigdecimal_to_decimal(&bd) {
            Self::from_decimal(d)
        } else {
            NumericValue::BigDecimal(bd)
        }
    }

    // Type checking predicates
    pub fn is_nan(&self) -> bool {
        matches!(self, NumericValue::NaN)
    }

    pub fn is_finite(&self) -> bool {
        matches!(
            self,
            NumericValue::Rational(_, _)
                | NumericValue::Decimal(_)
                | NumericValue::BigDecimal(_)
                | NumericValue::NegativeZero
        )
    }

    pub fn is_infinite(&self) -> bool {
        matches!(
            self,
            NumericValue::PositiveInfinity | NumericValue::NegativeInfinity
        )
    }

    pub fn is_positive_infinity(&self) -> bool {
        matches!(self, NumericValue::PositiveInfinity)
    }

    pub fn is_negative_infinity(&self) -> bool {
        matches!(self, NumericValue::NegativeInfinity)
    }

    // Introspection for representation type
    pub fn representation(&self) -> &'static str {
        match self {
            NumericValue::Rational(_, _) => "Rational",
            NumericValue::Decimal(_) => "Decimal",
            NumericValue::BigDecimal(_) => "BigDecimal",
            NumericValue::NaN => "NaN",
            NumericValue::PositiveInfinity => "PositiveInfinity",
            NumericValue::NegativeInfinity => "NegativeInfinity",
            NumericValue::NegativeZero => "NegativeZero",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApproximationType {
    Transcendental,        // From irrational operations
    RationalApproximation, // From Rational→Decimal graduation
}

/// Information about a Number's internal state for introspection.
///
/// Use `Number::info()` to get this struct. It provides visibility into
/// the internal representation and approximation status without exposing
/// the underlying implementation details.
///
/// # Example
/// ```
/// use faithful_number::Number;
///
/// let n = Number::from(1) / Number::from(3);
/// let info = n.info();
/// println!("{}", info); // Rational (exact)
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NumberInfo {
    /// The internal representation type: "Rational", "Decimal", "BigDecimal",
    /// "NaN", "PositiveInfinity", "NegativeInfinity", or "NegativeZero"
    pub representation: &'static str,
    /// Whether the value is exact (no precision was lost)
    pub is_exact: bool,
    /// The type of approximation, if any
    pub approximation_type: Option<ApproximationType>,
}

impl std::fmt::Display for NumberInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.representation)?;
        if self.is_exact {
            write!(f, " (exact)")
        } else {
            match &self.approximation_type {
                Some(ApproximationType::Transcendental) => {
                    write!(f, " (approximate: Transcendental)")
                }
                Some(ApproximationType::RationalApproximation) => {
                    write!(f, " (approximate: RationalApproximation)")
                }
                None => write!(f, " (approximate)"),
            }
        }
    }
}

/// The main public number type - a wrapper around NumericValue that tracks
/// whether the value has been approximated through operations
#[derive(Debug, Clone)]
pub struct Number {
    pub(crate) value: NumericValue,
    pub(crate) apprx: Option<ApproximationType>,
}

impl Number {
    // Constants
    pub const NAN: Number = Number {
        value: NumericValue::NaN,
        apprx: None,
    };
    pub const POSITIVE_INFINITY: Number = Number {
        value: NumericValue::PositiveInfinity,
        apprx: None,
    };
    pub const NEGATIVE_INFINITY: Number = Number {
        value: NumericValue::NegativeInfinity,
        apprx: None,
    };
    // ZERO and ONE as Rational for consistency with Number::from(0/1)
    // Cannot be const because Ratio::new is not const, so we use functions
    #[inline]
    #[allow(non_snake_case)]
    pub fn ZERO() -> Number {
        Number {
            value: NumericValue::zero(),
            apprx: None,
        }
    }
    #[inline]
    #[allow(non_snake_case)]
    pub fn ONE() -> Number {
        Number {
            value: NumericValue::one(),
            apprx: None,
        }
    }
    pub const NEGATIVE_ZERO: Number = Number {
        value: NumericValue::NegativeZero,
        apprx: None,
    };

    // Constructors
    pub fn new(num: i64, scale: u32) -> Self {
        Number {
            value: NumericValue::new(num, scale),
            apprx: None,
        }
    }

    pub fn nan() -> Self {
        Number::NAN
    }

    pub fn infinity() -> Self {
        Number::POSITIVE_INFINITY
    }

    pub fn neg_infinity() -> Self {
        Number::NEGATIVE_INFINITY
    }

    pub fn neg_zero() -> Self {
        Number::NEGATIVE_ZERO
    }

    pub const fn new_uint(num: u32) -> Self {
        Number {
            value: NumericValue::new_uint(num),
            apprx: None,
        }
    }

    pub fn try_from_i128_with_scale(num: i128, scale: u32) -> Result<Self, rust_decimal::Error> {
        // Automatically falls back to BigDecimal if num exceeds Decimal capacity
        Ok(Number {
            value: NumericValue::try_from_i128_with_scale(num, scale)?,
            apprx: None,
        })
    }

    pub fn from_rational(r: Rational64) -> Self {
        Number {
            value: NumericValue::from_rational(r),
            apprx: None,
        }
    }

    pub fn from_decimal(d: Decimal) -> Self {
        Number {
            value: NumericValue::from_decimal(d),

            apprx: None,
        }
    }

    pub fn from_bigdecimal(bd: BigDecimal) -> Self {
        Number {
            value: NumericValue::from_bigdecimal(bd),
            apprx: None,
        }
    }

    // Type checking predicates
    pub fn is_nan(&self) -> bool {
        self.value.is_nan()
    }

    pub fn is_finite(&self) -> bool {
        self.value.is_finite()
    }

    pub fn is_infinite(&self) -> bool {
        self.value.is_infinite()
    }

    pub fn is_positive_infinity(&self) -> bool {
        self.value.is_positive_infinity()
    }

    pub fn is_negative_infinity(&self) -> bool {
        self.value.is_negative_infinity()
    }

    /// Alias for is_negative_infinity
    pub fn is_neg_infinity(&self) -> bool {
        self.value.is_negative_infinity()
    }

    pub fn is_neg_zero(&self) -> bool {
        matches!(self.value, NumericValue::NegativeZero)
    }

    pub fn is_zero(&self) -> bool {
        use num_traits::Zero;
        match &self.value {
            NumericValue::Rational(r, _) => r.is_zero(),
            NumericValue::Decimal(d) => d.is_zero(),
            NumericValue::BigDecimal(bd) => bd.is_zero(),
            NumericValue::NegativeZero => true,
            _ => false,
        }
    }

    // Introspection
    pub fn representation(&self) -> &'static str {
        self.value.representation()
    }

    pub fn is_exact(&self) -> bool {
        self.apprx.is_none()
    }

    pub fn is_transcendental(&self) -> bool {
        matches!(self.apprx, Some(ApproximationType::Transcendental))
    }

    pub fn is_rational_approximation(&self) -> bool {
        matches!(self.apprx, Some(ApproximationType::RationalApproximation))
    }

    /// Returns complete information about this Number's internal state.
    ///
    /// This is useful for debugging and understanding how the library
    /// is representing and tracking your numbers.
    ///
    /// # Example
    /// ```
    /// use faithful_number::Number;
    ///
    /// let n = Number::from(2).sqrt();
    /// let info = n.info();
    /// assert!(!info.is_exact); // sqrt(2) is transcendental
    /// println!("{}", info);     // "Decimal (approximate: Transcendental)"
    /// ```
    pub fn info(&self) -> NumberInfo {
        NumberInfo {
            representation: self.value.representation(),
            is_exact: self.apprx.is_none(),
            approximation_type: self.apprx.clone(),
        }
    }

    // Debug-only unwrap helpers that panic on logic bugs
    #[cfg(debug_assertions)]
    #[allow(dead_code)]
    pub(crate) fn assert_transcendental(&self) {
        assert!(
            matches!(self.apprx, Some(ApproximationType::Transcendental)),
            "Expected Transcendental approximation"
        );
    }

    #[cfg(debug_assertions)]
    #[allow(dead_code)]
    pub(crate) fn assert_rational_approximation(&self) {
        assert!(
            matches!(self.apprx, Some(ApproximationType::RationalApproximation)),
            "Expected RationalApproximation"
        );
    }

    #[cfg(debug_assertions)]
    #[allow(dead_code)]
    pub(crate) fn assert_exact(&self) {
        assert!(self.apprx.is_none(), "Expected exact value");
    }

    // Conversion methods
    pub fn to_i32(&self) -> Option<i32> {
        self.value.to_i32()
    }

    pub fn to_u32(&self) -> Option<u32> {
        self.value.to_u32()
    }

    pub fn to_i64(&self) -> Option<i64> {
        self.value.to_i64()
    }

    pub fn to_f64(&self) -> f64 {
        self.value.to_f64()
    }

    pub fn to_decimal(&self) -> Option<Decimal> {
        self.value.to_decimal()
    }

    /// Extract the exact rational representation if stored internally as one
    pub fn to_rational64(&self) -> Option<Rational64> {
        match &self.value {
            NumericValue::Rational(r, _) => Some(*r),
            _ => None,
        }
    }

    pub(crate) fn value(&self) -> &NumericValue {
        &self.value
    }

    /// Try to demote to simpler representation after operation
    /// This is called after arithmetic operations to recover exact representations when possible
    #[inline]
    pub(crate) fn try_demote(self) -> Self {
        match &self.value {
            NumericValue::BigDecimal(bd) => {
                // CRITICAL: Check magnitude BEFORE attempting rational recovery
                // Only try expensive continued fractions if value is small enough
                if is_small_enough_for_rational(bd) {
                    // Value is small - worth trying rational recovery
                    if let Some(rat) = try_decimal_to_rational_bigdecimal(bd) {
                        let is_term = is_terminating_decimal(*rat.numer(), *rat.denom());
                        return Number {
                            value: NumericValue::Rational(rat, is_term),
                            apprx: None,
                        };
                    }
                }
                // else: value too large, skip rational attempt (would fail anyway)

                // Try Decimal demotion (for magnitude overflow cases)
                // But ONLY if we don't have rational_approximation flag
                if self.apprx != Some(ApproximationType::RationalApproximation)
                    && let Some(dec) = try_bigdecimal_to_decimal(bd)
                {
                    return Number {
                        value: NumericValue::from_decimal(dec),
                        apprx: self.apprx,
                    };
                }

                // Keep as BigDecimal
                self
            }
            NumericValue::Decimal(d) => {
                // Try Rational recovery from Decimal
                if let Some(rat) = try_decimal_to_rational(*d) {
                    let is_term = is_terminating_decimal(*rat.numer(), *rat.denom());
                    return Number {
                        value: NumericValue::Rational(rat, is_term),
                        apprx: None, // Flag cleared if it was set
                    };
                }
                self
            }
            _ => self,
        }
    }

    /// Set the default precision for high-precision transcendental operations.
    ///
    /// This is a convenience method that calls `crate::precision::set_default_precision`.
    /// When the `high_precision` feature is enabled, this controls the precision (in bits)
    /// used for transcendental operations like sin, cos, log, exp, etc.
    ///
    /// # Arguments
    /// * `bits` - Precision in bits. Recommended: 100-200 for most uses, 300+ for high precision.
    ///
    /// # Example
    /// ```
    /// use faithful_number::Number;
    ///
    /// #[cfg(feature = "high_precision")]
    /// {
    ///     Number::set_default_precision(200);
    ///     let result = Number::from(2).sqrt();
    /// }
    /// ```
    pub fn set_default_precision(bits: u32) {
        crate::precision::set_default_precision(bits)
    }

    /// Get the current default precision in bits.
    ///
    /// Returns 0 when the `high_precision` feature is disabled (uses f64).
    pub fn get_default_precision() -> u32 {
        crate::precision::get_default_precision()
    }
}

/// Check if a rational is a terminating decimal
/// Returns true if denominator = 2^a × 5^b (only factors of 2 and 5)
#[inline(always)]
pub(crate) fn is_terminating_decimal(_numer: i64, denom: i64) -> bool {
    let mut d = denom.abs();

    // Fast path: common denominators
    if d == 1 || d == 2 || d == 4 || d == 5 || d == 8 || d == 10 || d == 100 || d == 1000 {
        return true;
    }

    // Fast path: if d is odd and not a power of 5, can't be terminating unless d == 1
    if d & 1 == 1 {
        // d is odd, so no factors of 2. Check if it's a power of 5.
        while d % 5 == 0 {
            d /= 5;
        }
        return d == 1;
    }

    // Remove all factors of 2 using bit shift (much faster than loop)
    // trailing_zeros() counts how many factors of 2 there are
    d >>= d.trailing_zeros();

    // Remove all factors of 5
    while d % 5 == 0 {
        d /= 5;
    }

    // If only 1 remains, it was composed only of 2s and 5s
    d == 1
}

/// Check if a BigDecimal is small enough to potentially fit in Rational64
/// Heuristic: if |value| is very large, unlikely to find valid i64/i64 ratio
#[inline]
pub(crate) fn is_small_enough_for_rational(bd: &BigDecimal) -> bool {
    // Threshold: i64::MAX / 1000
    // Rationale: Rational64 uses i64 numerator and denominator. To be safe,
    // we allow values up to roughly i64::MAX / 1000, which leaves room for
    // both numerator and denominator while accounting for worst-case rounding.
    // This is ~9×10^15, much more permissive than i32::MAX (~2×10^9).
    const THRESHOLD: i64 = i64::MAX / 1000;
    bd.abs() <= BigDecimal::from(THRESHOLD)
}

/// Try to downgrade Decimal to Rational if it represents an exact fraction that fits in i64
///
/// Uses continued fractions to find a rational approximation, then VERIFIES that the
/// rational, when converted back to Decimal, matches the original input exactly across
/// all 28 digits. This prevents false positives where a value close to (but not exactly)
/// a simple fraction gets incorrectly marked as exact.
pub(crate) fn try_decimal_to_rational(d: Decimal) -> Option<Rational64> {
    // Early exit for very large or very small numbers
    // Skip expensive continued fractions algorithm if value can't possibly fit in i64/i64
    // Use same threshold as is_small_enough_for_rational
    const THRESHOLD: i64 = i64::MAX / 1000;
    if d.abs() > Decimal::from(THRESHOLD) {
        return None;
    }

    // Get the mantissa and scale from Decimal
    let mantissa = d.mantissa();
    let scale = d.scale();

    #[cfg(test)]
    println!(
        "try_decimal_to_rational: mantissa={}, scale={}",
        mantissa, scale
    );

    // Try to convert mantissa to i64
    let numerator_opt: Result<i64, _> = mantissa.try_into();
    #[cfg(test)]
    println!("numerator_opt={:?}", numerator_opt);

    // If mantissa fits in i64, try direct conversion
    if let Ok(numerator) = numerator_opt {
        if scale == 0 {
            // Integers are exact by construction - no verification needed
            return Some(Ratio::from_integer(numerator));
        }

        if let Some(denominator) = 10i64.checked_pow(scale) {
            // 10^scale conversions are exact by construction
            return Some(Ratio::new(numerator, denominator));
        }
    }

    #[cfg(test)]
    println!("Direct conversion failed, using rational_approximation");

    // If direct conversion failed, use continued fractions
    let candidate = rational_approximation(d, CF_MAX_DENOM)?;

    // CRITICAL: Verify the candidate matches exactly
    verify_exact_match(d, candidate)
}

/// Verify that a rational, when converted to Decimal, exactly matches the original
/// Returns Some(rational) only if all 28 digits match
fn verify_exact_match(original: Decimal, candidate: Rational64) -> Option<Rational64> {
    // Convert rational back to Decimal
    let reconstructed = Decimal::from(*candidate.numer()) / Decimal::from(*candidate.denom());

    #[cfg(test)]
    {
        println!("Verification:");
        println!("  Original:      {}", original);
        println!("  Reconstructed: {}", reconstructed);
        println!("  Match: {}", original == reconstructed);
    }

    // Only return the rational if it matches EXACTLY
    (reconstructed == original).then_some(candidate)
}

/// Find the best rational approximation using integer-only continued fractions
/// with denominator bounded by max_denom.
///
/// This is 2-3x faster than the Decimal-based version and perfectly accurate
/// because it uses pure integer arithmetic (no floating point rounding errors).
fn rational_approximation(d: Decimal, max_denom: i64) -> Option<Rational64> {
    #[cfg(test)]
    println!("rational_approximation: d={}", d);

    let sign = if d < Decimal::ZERO { -1 } else { 1 };
    let d = d.abs();

    // Extract mantissa/scale directly from Decimal
    let mantissa = d.mantissa();
    let scale = d.scale();
    let scale_factor = 10i128.pow(scale);

    // Now we have: d = mantissa / scale_factor (exact!)
    // Perform CF on (mantissa, scale_factor) using pure integer arithmetic

    let mut a = mantissa;
    let mut b = scale_factor;

    let mut p_prev2 = 1i128;
    let mut q_prev2 = 0i128;

    let a0 = a / b;
    let mut p_prev1 = a0;
    let mut q_prev1 = 1i128;

    a %= b; // Remainder - pure integer op

    let (mut best_n, mut best_d) = (a0, 1i128);

    #[cfg(test)]
    println!("a0={}, best={}/{}", a0, best_n, best_d);

    for _iter in 0..100 {
        if a == 0 {
            break;
        }

        // Standard Euclidean algorithm for CF: work with b/a (reciprocal)
        let a_n = b / a;

        // If a_n is 0, something is wrong (shouldn't happen if a != 0)
        if a_n == 0 {
            break;
        }

        let r = b % a; // Remainder
        b = a;
        a = r;

        let p_n = a_n.checked_mul(p_prev1)?.checked_add(p_prev2)?;
        let q_n = a_n.checked_mul(q_prev1)?.checked_add(q_prev2)?;

        if q_n > max_denom as i128 {
            break;
        }

        best_n = p_n;
        best_d = q_n;

        p_prev2 = p_prev1;
        p_prev1 = p_n;
        q_prev2 = q_prev1;
        q_prev1 = q_n;
    }

    let final_n: i64 = (sign as i128 * best_n).try_into().ok()?;
    let final_d: i64 = best_d.try_into().ok()?;

    Some(Ratio::new(final_n, final_d))
}

/// Try to downgrade BigDecimal to Decimal if it fits
#[inline]
pub(crate) fn try_bigdecimal_to_decimal(bd: &BigDecimal) -> Option<Decimal> {
    use bigdecimal::ToPrimitive;

    // Extract components
    let (bigint, scale) = bd.as_bigint_and_exponent();

    // Decimal scale is i64, but max is 28
    let scale_i32: i32 = scale.try_into().ok()?;
    if !(0..=28).contains(&scale_i32) {
        return None;
    }

    // Check if mantissa fits in i128 (Decimal's internal type)
    let mantissa: i128 = bigint.to_i128()?;

    // Check significant digits
    let digits = mantissa.abs().to_string().len();
    if digits > 28 {
        return None;
    }

    // Construct Decimal
    Decimal::try_from_i128_with_scale(mantissa, scale_i32 as u32).ok()
}

/// Try to recover exact rational from BigDecimal using continued fractions
/// ASSUMES: magnitude check already performed by caller
fn try_decimal_to_rational_bigdecimal(bd: &BigDecimal) -> Option<Rational64> {
    // First, try direct BigDecimal→Decimal conversion
    if let Some(d) = try_bigdecimal_to_decimal(bd) {
        // Use existing Decimal→Rational logic with consistent limit
        let candidate = rational_approximation(d, CF_MAX_DENOM)?;

        // CRITICAL: Verify exact match by converting back to BigDecimal
        let reconstructed =
            BigDecimal::from(*candidate.numer()) / BigDecimal::from(*candidate.denom());

        if reconstructed == *bd {
            return Some(candidate);
        }
    }

    // If direct conversion failed (too many digits), try truncating to 28 significant digits
    // and see if we can find a rational approximation

    // Round to 28 significant figures (Decimal's limit)
    let bd_rounded = bd.with_prec(28);

    if let Some(d) = try_bigdecimal_to_decimal(&bd_rounded) {
        let candidate = rational_approximation(d, CF_MAX_DENOM)?;

        // CRITICAL: Verify exact match against ORIGINAL BigDecimal
        let reconstructed =
            BigDecimal::from(*candidate.numer()) / BigDecimal::from(*candidate.denom());

        if reconstructed == *bd {
            return Some(candidate);
        }
    }

    None
}

#[cfg(test)]
mod test_demot {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_try_decimal_to_rational_integer() {
        let d = Decimal::from(5);
        let r = try_decimal_to_rational(d).unwrap();
        assert_eq!(*r.numer(), 5);
        assert_eq!(*r.denom(), 1);
    }

    #[test]
    fn test_try_decimal_to_rational_half() {
        let d = Decimal::from_str("0.5").unwrap();
        let r = try_decimal_to_rational(d).unwrap();
        // Should reduce to 1/2
        assert_eq!(*r.numer(), 1);
        assert_eq!(*r.denom(), 2);
    }

    #[test]
    fn test_try_decimal_to_rational_third() {
        // 1/3 as Decimal is 0.3333... with many decimal places
        let third = Ratio::new(1, 3);
        let third_dec = Decimal::from(*third.numer()) / Decimal::from(*third.denom());
        println!("1/3 as Decimal: {}", third_dec);
        println!("Scale: {}", third_dec.scale());
        println!("Mantissa: {}", third_dec.mantissa());

        let r = try_decimal_to_rational(third_dec).unwrap();
        println!("Result: {}/{}", r.numer(), r.denom());

        // Should now use continued fractions to find 1/3
        assert_eq!(r, Ratio::new(1, 3));
    }

    #[test]
    fn test_ratio_new_reduces() {
        // Test if Ratio::new automatically reduces
        let r = Ratio::new(2, 4);
        assert_eq!(*r.numer(), 1);
        assert_eq!(*r.denom(), 2);

        let r2 = Ratio::new(333333333, 1000000000);
        println!(
            "Ratio::new(333333333, 1000000000) = {}/{}",
            r2.numer(),
            r2.denom()
        );
        // Check what this reduces to
    }
    #[test]
    fn test_cf_algorithm_five_thirds() {
        // Test the CF algorithm with 5/3, which has CF [1; 1, 2]
        // This tests the critical case where a small remainder could cause premature termination
        let five_thirds = Decimal::new(5, 0) / Decimal::new(3, 0);

        let result = rational_approximation(five_thirds, 1_000_000_000);

        assert!(result.is_some(), "Should find rational for 5/3");
        let r = result.unwrap();

        // The CF algorithm should produce 5/3 (which reduces but Ratio::new handles that)
        // Actually, due to decimal precision, it might not be exactly 5/3
        // Let's verify it matches the original decimal
        let reconstructed = Decimal::from(*r.numer()) / Decimal::from(*r.denom());
        assert_eq!(
            reconstructed,
            five_thirds,
            "CF approximation should match original: got {}/{}",
            r.numer(),
            r.denom()
        );
    }

    #[test]
    fn test_exact_one_third_matches() {
        // True 1/3 in Decimal: 0.3333333333333333333333333333 (28 threes)
        let third_dec = Decimal::from(1) / Decimal::from(3);

        let result = try_decimal_to_rational(third_dec);

        // Should find 1/3 and verify it matches exactly
        assert!(result.is_some(), "Should find 1/3");
        let r = result.unwrap();
        assert_eq!(*r.numer(), 1);
        assert_eq!(*r.denom(), 3);
    }

    #[test]
    fn test_almost_one_third_rejected() {
        // Close to 1/3 but not exact: 0.3333333333333333333333333334
        let almost_third = Decimal::from_str("0.3333333333333333333333333334").unwrap();

        let result = try_decimal_to_rational(almost_third);

        // Continued fractions finds 1/3, but verification should reject it
        // because 1/3 → 0.333...3333 (not ending in 4)
        assert!(
            result.is_none(),
            "Should reject because 28 digits don't match exactly"
        );
    }

    #[test]
    fn test_one_third_plus_epsilon_rejected() {
        let third_dec = Decimal::from(1) / Decimal::from(3);
        let epsilon = Decimal::new(1, 28); // Smallest possible increment
        let sum = third_dec + epsilon;

        println!("1/3 + epsilon: {}", sum);

        let result = try_decimal_to_rational(sum);

        // Even tiny epsilon should cause rejection
        if third_dec != sum {
            assert!(
                result.is_none() || result.unwrap() != Ratio::new(1, 3),
                "Should not find 1/3 for 1/3 + epsilon"
            );
        }
    }

    #[test]
    fn test_ten_twenty_first_exact() {
        // 1/3 + 1/7 = 10/21 exactly
        let third = Decimal::from(1) / Decimal::from(3);
        let seventh = Decimal::from(1) / Decimal::from(7);
        let sum = third + seventh;

        let result = try_decimal_to_rational(sum);

        // Should find 10/21 AND verify it matches
        if let Some(r) = result {
            println!("Found: {}/{}", r.numer(), r.denom());

            // Expected: 10/21 (after reduction)
            let expected = Ratio::new(1, 3) + Ratio::new(1, 7);
            assert_eq!(r, expected, "Should find exact 10/21");
        } else {
            // If not found, that's also acceptable (means precision loss)
            println!("10/21 not found - acceptable if Decimal lost precision");
        }
    }

    #[test]
    fn test_rounding_errors_rejected() {
        // (1/3 * 10) / 10 might have tiny rounding errors
        let third_dec = Decimal::from(1) / Decimal::from(3);
        let scaled = (third_dec * Decimal::from(10)) / Decimal::from(10);

        let result = try_decimal_to_rational(scaled);

        if third_dec == scaled {
            // If Decimal preserved precision, should find 1/3
            assert_eq!(result.unwrap(), Ratio::new(1, 3));
        } else {
            // If rounding error occurred, should reject
            println!("Rounding error detected:");
            println!("  Original: {}", third_dec);
            println!("  Scaled:   {}", scaled);
            assert!(
                result.is_none() || result.unwrap() != Ratio::new(1, 3),
                "Should not find 1/3 if rounding error occurred"
            );
        }
    }
}
