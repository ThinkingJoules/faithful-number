use bigdecimal::BigDecimal;
use num_rational::Ratio;
use num_traits::ToPrimitive;
use rust_decimal::Decimal;

/// Type alias for Rational64 (exact fractions with i64 numerator/denominator)
pub type Rational64 = Ratio<i64>;

/// A smart number type that supports multiple internal representations
/// with automatic upgrades for precision and proper handling of IEEE special values
#[derive(Debug, Clone)]
pub(crate) enum NumericValue {
    /// Exact rational number (e.g., 1/3, 2/7)
    Rational(Rational64),
    /// Fixed-point decimal with 28 significant digits (renamed from Finite)
    Decimal(Decimal),
    /// Arbitrary precision decimal for very large numbers
    BigDecimal(BigDecimal),
    /// JavaScript NaN (Not a Number)
    NaN,
    /// JavaScript positive infinity
    PositiveInfinity,
    /// JavaScript negative infinity
    NegativeInfinity,
    /// JavaScript negative zero (distinct from positive zero)
    NegativeZero,
}

impl NumericValue {
    // Constants
    pub const NAN: NumericValue = NumericValue::NaN;
    pub const POSITIVE_INFINITY: NumericValue = NumericValue::PositiveInfinity;
    pub const NEGATIVE_INFINITY: NumericValue = NumericValue::NegativeInfinity;
    pub const ZERO: NumericValue = NumericValue::Decimal(Decimal::ZERO);
    pub const ONE: NumericValue = NumericValue::Decimal(Decimal::ONE);
    pub const NEGATIVE_ZERO: NumericValue = NumericValue::NegativeZero;

    // Constructors for Decimal
    pub fn new(num: i64, scale: u32) -> Self {
        Self::Decimal(Decimal::new(num, scale))
    }

    pub const fn new_uint(num: u32) -> Self {
        Self::Decimal(Decimal::from_parts(num, 0, 0, false, 0))
    }

    pub fn try_from_i128_with_scale(num: i128, scale: u32) -> Result<Self, rust_decimal::Error> {
        Ok(Self::Decimal(Decimal::try_from_i128_with_scale(
            num, scale,
        )?))
    }

    // Constructors for new numeric types
    pub fn from_rational(r: Rational64) -> Self {
        NumericValue::Rational(r)
    }

    pub fn from_decimal(d: Decimal) -> Self {
        // Try to downgrade to Rational first
        if let Some(r) = try_decimal_to_rational(d) {
            NumericValue::Rational(r)
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
            NumericValue::Rational(_)
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
    pub fn representation(&self) -> &str {
        match self {
            NumericValue::Rational(_) => "Rational",
            NumericValue::Decimal(_) => "Decimal",
            NumericValue::BigDecimal(_) => "BigDecimal",
            NumericValue::NaN => "NaN",
            NumericValue::PositiveInfinity => "PositiveInfinity",
            NumericValue::NegativeInfinity => "NegativeInfinity",
            NumericValue::NegativeZero => "NegativeZero",
        }
    }
}

#[derive(Debug, Clone)]
pub enum ApproximationType {
    Transcendental,        // From irrational operations
    RationalApproximation, // From Rational→Decimal graduation
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
    pub const ZERO: Number = Number {
        value: NumericValue::Decimal(Decimal::ZERO),
        apprx: None,
    };
    pub const ONE: Number = Number {
        value: NumericValue::Decimal(Decimal::ONE),
        apprx: None,
    };
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

    // Introspection
    pub fn representation(&self) -> &str {
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

    // Debug-only unwrap helpers that panic on logic bugs
    #[cfg(debug_assertions)]
    pub(crate) fn assert_transcendental(&self) {
        assert!(
            matches!(self.apprx, Some(ApproximationType::Transcendental)),
            "Expected Transcendental approximation"
        );
    }

    #[cfg(debug_assertions)]
    pub(crate) fn assert_rational_approximation(&self) {
        assert!(
            matches!(self.apprx, Some(ApproximationType::RationalApproximation)),
            "Expected RationalApproximation"
        );
    }

    #[cfg(debug_assertions)]
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

    pub(crate) fn value(&self) -> &NumericValue {
        &self.value
    }
}

/// Try to downgrade Decimal to Rational if it represents an exact fraction that fits in i64
fn try_decimal_to_rational(d: Decimal) -> Option<Rational64> {
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
        // If scale is 0, it's an integer
        if scale == 0 {
            return Some(Ratio::from_integer(numerator));
        }

        // Otherwise, denominator is 10^scale
        // Check if 10^scale fits in i64
        if let Some(denominator) = 10i64.checked_pow(scale) {
            let ratio = Ratio::new(numerator, denominator);

            // Check if this is a "nice" rational (small denominator after reduction)
            // If the denominator is still large, try rational approximation to find simpler form
            if *ratio.denom() > 1000 {
                #[cfg(test)]
                println!("Large denominator {}, trying approximation", ratio.denom());

                // Try to find a simpler rational approximation
                if let Some(approx) = rational_approximation(d, 1_000_000) {
                    // Check if the approximation is close enough (within Decimal precision)
                    let ratio_dec = Decimal::from(*ratio.numer()) / Decimal::from(*ratio.denom());
                    let approx_dec = Decimal::from(*approx.numer()) / Decimal::from(*approx.denom());

                    #[cfg(test)]
                    println!("Checking approximation: ratio_dec={}, approx_dec={}, diff={}", ratio_dec, approx_dec, (ratio_dec - approx_dec).abs());

                    // Use a threshold of 1e-9 (should be close enough for most practical purposes)
                    if (ratio_dec - approx_dec).abs() < Decimal::new(1, 9) {
                        #[cfg(test)]
                        println!("Using approximation {}/{} instead of {}/{}", approx.numer(), approx.denom(), ratio.numer(), ratio.denom());
                        return Some(approx);
                    }
                }
            }

            return Some(ratio);
        }
    }

    #[cfg(test)]
    println!("Direct conversion failed, using rational_approximation");

    // If direct conversion failed (mantissa or denominator overflow), use rational approximation
    // with a reasonable denominator bound (e.g., 1 billion)
    rational_approximation(d, 1_000_000_000)
}

/// Find the best rational approximation using continued fractions
/// with denominator bounded by max_denom
fn rational_approximation(d: Decimal, max_denom: i64) -> Option<Rational64> {
    #[cfg(test)]
    println!("rational_approximation: d={}", d);

    let sign = if d < Decimal::ZERO { -1 } else { 1 };
    let mut x = d.abs();

    // Standard continued fractions algorithm
    // p_{-1} = 1, q_{-1} = 0, p_0 = a_0, q_0 = 1
    // p_n = a_n * p_{n-1} + p_{n-2}
    // q_n = a_n * q_{n-1} + q_{n-2}

    let mut p_prev2 = 1i128; // p_{-1}
    let mut q_prev2 = 0i128; // q_{-1}

    let a0 = x.floor().to_i128()?;
    let mut p_prev1 = a0; // p_0
    let mut q_prev1 = 1i128; // q_0

    let (mut best_n, mut best_d) = (a0, 1i128);

    #[cfg(test)]
    println!("a0={}, best={}/{}", a0, best_n, best_d);

    x = x - Decimal::from(a0);

    for _iter in 0..100 {
        // Stop if remainder is very small (we've found a good approximation)
        if x < Decimal::new(1, 12) {
            break;
        }

        if x.is_zero() {
            break;
        }

        // Stop if we've already exceeded the bound
        if q_prev1 > max_denom as i128 {
            break;
        }

        x = Decimal::ONE / x;
        let a_n = x.floor().to_i128()?;

        #[cfg(test)]
        println!("Iter {}: x={}, a_n={}", _iter, x, a_n);

        // Compute next convergent
        let p_n = a_n.checked_mul(p_prev1)?.checked_add(p_prev2)?;
        let q_n = a_n.checked_mul(q_prev1)?.checked_add(q_prev2)?;

        #[cfg(test)]
        println!("  p_n={}, q_n={}, convergent={}/{}", p_n, q_n, p_n, q_n);

        if q_n > max_denom as i128 {
            // Don't update best if we've exceeded the bound
            break;
        }

        best_n = p_n;
        best_d = q_n;

        x = x - Decimal::from(a_n);

        // If remainder is very small after this convergent, we've found a good approximation
        if x < Decimal::new(1, 12) {
            break;
        }

        p_prev2 = p_prev1;
        p_prev1 = p_n;
        q_prev2 = q_prev1;
        q_prev1 = q_n;
    }

    // Convert back to i64
    let final_n: i64 = (sign as i128 * best_n).try_into().ok()?;
    let final_d: i64 = best_d.try_into().ok()?;

    Some(Ratio::new(final_n, final_d))
}

/// Try to downgrade BigDecimal to Decimal if it fits
fn try_bigdecimal_to_decimal(_bd: &BigDecimal) -> Option<Decimal> {
    // TODO: implement BigDecimal → Decimal conversion
    // For now, return None to keep as BigDecimal
    None
}
#[cfg(test)]
mod tests {
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
}
