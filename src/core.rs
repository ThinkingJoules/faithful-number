use rust_decimal::Decimal;

/// A JavaScript-semantic number that uses Decimal for finite arithmetic
/// but properly handles IEEE special values (NaN, ±Infinity, -0)
#[derive(Debug, Clone, Copy)]
pub enum Number {
    /// A finite decimal number
    Finite(Decimal),
    /// JavaScript NaN (Not a Number)
    NaN,
    /// JavaScript positive infinity
    PositiveInfinity,
    /// JavaScript negative infinity
    NegativeInfinity,
    /// JavaScript negative zero (distinct from positive zero)
    NegativeZero,
}

impl Number {
    // Constants
    pub const NAN: Number = Number::NaN;
    pub const POSITIVE_INFINITY: Number = Number::PositiveInfinity;
    pub const NEGATIVE_INFINITY: Number = Number::NegativeInfinity;
    pub const ZERO: Number = Number::Finite(Decimal::ZERO);
    pub const ONE: Number = Number::Finite(Decimal::ONE);
    pub const NEGATIVE_ZERO: Number = Number::NegativeZero;

    pub fn new(num: i64, scale: u32) -> Self {
        Self::Finite(Decimal::new(num, scale))
    }
    pub const fn new_uint(num: u32) -> Self {
        Self::Finite(Decimal::from_parts(num, 0, 0, false, 0))
    }

    pub fn try_from_i128_with_scale(num: i128, scale: u32) -> Result<Self, rust_decimal::Error> {
        Ok(Self::Finite(Decimal::try_from_i128_with_scale(num, scale)?))
    }

    // Type checking predicates
    pub fn is_nan(&self) -> bool {
        matches!(self, Number::NaN)
    }

    pub fn is_finite(&self) -> bool {
        matches!(self, Number::Finite(_) | Number::NegativeZero)
    }

    pub fn is_infinite(&self) -> bool {
        matches!(self, Number::PositiveInfinity | Number::NegativeInfinity)
    }

    pub fn is_positive_infinity(&self) -> bool {
        matches!(self, Number::PositiveInfinity)
    }

    pub fn is_negative_infinity(&self) -> bool {
        matches!(self, Number::NegativeInfinity)
    }
}
