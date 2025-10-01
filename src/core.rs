use bigdecimal::BigDecimal;
use num_rational::Ratio;
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
        NumericValue::Decimal(d)
    }

    pub fn from_bigdecimal(bd: BigDecimal) -> Self {
        NumericValue::BigDecimal(bd)
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

/// The main public number type - a wrapper around NumericValue that tracks
/// whether the value has been approximated through operations
#[derive(Debug, Clone)]
pub struct Number {
    pub(crate) value: NumericValue,
    pub(crate) approximated: bool,
}

impl Number {
    // Constants
    pub const NAN: Number = Number {
        value: NumericValue::NaN,
        approximated: false,
    };
    pub const POSITIVE_INFINITY: Number = Number {
        value: NumericValue::PositiveInfinity,
        approximated: false,
    };
    pub const NEGATIVE_INFINITY: Number = Number {
        value: NumericValue::NegativeInfinity,
        approximated: false,
    };
    pub const ZERO: Number = Number {
        value: NumericValue::Decimal(Decimal::ZERO),
        approximated: false,
    };
    pub const ONE: Number = Number {
        value: NumericValue::Decimal(Decimal::ONE),
        approximated: false,
    };
    pub const NEGATIVE_ZERO: Number = Number {
        value: NumericValue::NegativeZero,
        approximated: false,
    };

    // Constructors
    pub fn new(num: i64, scale: u32) -> Self {
        Number {
            value: NumericValue::new(num, scale),
            approximated: false,
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
            approximated: false,
        }
    }

    pub fn try_from_i128_with_scale(num: i128, scale: u32) -> Result<Self, rust_decimal::Error> {
        Ok(Number {
            value: NumericValue::try_from_i128_with_scale(num, scale)?,
            approximated: false,
        })
    }

    pub fn from_rational(r: Rational64) -> Self {
        Number {
            value: NumericValue::from_rational(r),
            approximated: false,
        }
    }

    pub fn from_decimal(d: Decimal) -> Self {
        Number {
            value: NumericValue::from_decimal(d),
            approximated: false,
        }
    }

    pub fn from_bigdecimal(bd: BigDecimal) -> Self {
        Number {
            value: NumericValue::from_bigdecimal(bd),
            approximated: false,
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
        !self.approximated
    }

    pub fn is_approximated(&self) -> bool {
        self.approximated
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

    // Internal helper to create an approximated number
    pub(crate) fn approximated(value: NumericValue) -> Self {
        Number {
            value,
            approximated: true,
        }
    }

    // Internal helper to access the value
    pub(crate) fn into_value(self) -> NumericValue {
        self.value
    }

    pub(crate) fn value(&self) -> &NumericValue {
        &self.value
    }
}
