use crate::{Number, NumericValue, forward_ref_binop};
use rust_decimal::Decimal;
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};
use num_traits::{Zero, Signed};
impl Add for NumericValue {
    type Output = NumericValue;
    fn add(self, rhs: NumericValue) -> NumericValue {
        match (self, rhs) {
            (NumericValue::Decimal(a), NumericValue::Decimal(b)) => {
                // Try addition, graduate to BigDecimal on overflow
                match a.checked_add(b) {
                    Some(result) => NumericValue::Decimal(result),
                    None => {
                        // Overflow - graduate to BigDecimal
                        use bigdecimal::BigDecimal;
                        let a_str = a.to_string();
                        let b_str = b.to_string();
                        let a_bd: BigDecimal = a_str.parse().unwrap();
                        let b_bd: BigDecimal = b_str.parse().unwrap();
                        NumericValue::BigDecimal(a_bd + b_bd)
                    }
                }
            }
            (NumericValue::Decimal(a), NumericValue::NegativeZero) => NumericValue::Decimal(a),
            (NumericValue::NegativeZero, NumericValue::Decimal(b)) => NumericValue::Decimal(b),
            (NumericValue::NegativeZero, NumericValue::NegativeZero) => NumericValue::NegativeZero, // (-0) + (-0) = -0

            // BigDecimal operations
            (NumericValue::BigDecimal(a), NumericValue::BigDecimal(b)) => {
                NumericValue::BigDecimal(a + b)
            }
            (NumericValue::BigDecimal(a), NumericValue::Decimal(b)) => {
                use bigdecimal::BigDecimal;
                let b_str = b.to_string();
                let b_bd: BigDecimal = b_str.parse().unwrap();
                NumericValue::BigDecimal(a + b_bd)
            }
            (NumericValue::Decimal(a), NumericValue::BigDecimal(b)) => {
                use bigdecimal::BigDecimal;
                let a_str = a.to_string();
                let a_bd: BigDecimal = a_str.parse().unwrap();
                NumericValue::BigDecimal(a_bd + b)
            }

            (NumericValue::NaN, _) | (_, NumericValue::NaN) => NumericValue::NaN,
            (NumericValue::PositiveInfinity, NumericValue::NegativeInfinity)
            | (NumericValue::NegativeInfinity, NumericValue::PositiveInfinity) => NumericValue::NaN, // ∞ + (-∞) = NaN
            (NumericValue::PositiveInfinity, _) | (_, NumericValue::PositiveInfinity) => {
                NumericValue::PositiveInfinity
            }
            (NumericValue::NegativeInfinity, _) | (_, NumericValue::NegativeInfinity) => {
                NumericValue::NegativeInfinity
            }
            // Rational operations not yet implemented
            _ => unimplemented!("Add operation with Rational not yet implemented"),
        }
    }
}

impl Sub for NumericValue {
    type Output = NumericValue;
    fn sub(self, rhs: NumericValue) -> NumericValue {
        match (self, rhs) {
            (NumericValue::Decimal(a), NumericValue::Decimal(b)) => {
                // Try subtraction, graduate to BigDecimal on overflow
                match a.checked_sub(b) {
                    Some(result) => NumericValue::Decimal(result),
                    None => {
                        // Overflow - graduate to BigDecimal
                        use bigdecimal::BigDecimal;
                        let a_str = a.to_string();
                        let b_str = b.to_string();
                        let a_bd: BigDecimal = a_str.parse().unwrap();
                        let b_bd: BigDecimal = b_str.parse().unwrap();
                        NumericValue::BigDecimal(a_bd - b_bd)
                    }
                }
            }
            (NumericValue::Decimal(a), NumericValue::NegativeZero) => NumericValue::Decimal(a), // x - (-0) = x
            (NumericValue::NegativeZero, NumericValue::Decimal(b)) => NumericValue::Decimal(-b), // (-0) - x = -x
            (NumericValue::NegativeZero, NumericValue::NegativeZero) => NumericValue::ZERO, // (-0) - (-0) = +0

            // BigDecimal operations
            (NumericValue::BigDecimal(a), NumericValue::BigDecimal(b)) => {
                NumericValue::BigDecimal(a - b)
            }
            (NumericValue::BigDecimal(a), NumericValue::Decimal(b)) => {
                use bigdecimal::BigDecimal;
                let b_str = b.to_string();
                let b_bd: BigDecimal = b_str.parse().unwrap();
                NumericValue::BigDecimal(a - b_bd)
            }
            (NumericValue::Decimal(a), NumericValue::BigDecimal(b)) => {
                use bigdecimal::BigDecimal;
                let a_str = a.to_string();
                let a_bd: BigDecimal = a_str.parse().unwrap();
                NumericValue::BigDecimal(a_bd - b)
            }

            (NumericValue::NaN, _) | (_, NumericValue::NaN) => NumericValue::NaN,
            (NumericValue::PositiveInfinity, NumericValue::PositiveInfinity)
            | (NumericValue::NegativeInfinity, NumericValue::NegativeInfinity) => NumericValue::NaN, // ∞ - ∞ = NaN
            (NumericValue::PositiveInfinity, _) => NumericValue::PositiveInfinity,
            (NumericValue::NegativeInfinity, _) => NumericValue::NegativeInfinity,
            (_, NumericValue::PositiveInfinity) => NumericValue::NegativeInfinity,
            (_, NumericValue::NegativeInfinity) => NumericValue::PositiveInfinity,
            // Rational operations not yet implemented
            _ => unimplemented!("Sub operation with Rational not yet implemented"),
        }
    }
}

impl Mul for NumericValue {
    type Output = NumericValue;
    fn mul(self, rhs: NumericValue) -> NumericValue {
        match (self, rhs) {
            (NumericValue::Decimal(a), NumericValue::Decimal(b)) => {
                // Try multiplication, graduate to BigDecimal on overflow
                match a.checked_mul(b) {
                    Some(result) => NumericValue::Decimal(result),
                    None => {
                        // Overflow - graduate to BigDecimal
                        use bigdecimal::BigDecimal;
                        let a_str = a.to_string();
                        let b_str = b.to_string();
                        let a_bd: BigDecimal = a_str.parse().unwrap();
                        let b_bd: BigDecimal = b_str.parse().unwrap();
                        NumericValue::BigDecimal(a_bd * b_bd)
                    }
                }
            }
            (NumericValue::Decimal(a), NumericValue::NegativeZero) => {
                if a.is_zero() {
                    NumericValue::NegativeZero // 0 * (-0) = -0 in JS
                } else if a > Decimal::ZERO {
                    NumericValue::NegativeZero // positive * (-0) = -0
                } else {
                    NumericValue::ZERO // negative * (-0) = +0
                }
            }
            (NumericValue::NegativeZero, NumericValue::Decimal(b)) => {
                if b.is_zero() {
                    NumericValue::NegativeZero // (-0) * 0 = -0 in JS
                } else if b > Decimal::ZERO {
                    NumericValue::NegativeZero // (-0) * positive = -0
                } else {
                    NumericValue::ZERO // (-0) * negative = +0
                }
            }
            (NumericValue::NegativeZero, NumericValue::NegativeZero) => NumericValue::ZERO, // (-0) * (-0) = +0
            (NumericValue::NaN, _) | (_, NumericValue::NaN) => NumericValue::NaN,
            // 0 * ∞ = NaN in JavaScript
            (NumericValue::Decimal(a), NumericValue::PositiveInfinity)
            | (NumericValue::Decimal(a), NumericValue::NegativeInfinity)
                if a.is_zero() =>
            {
                NumericValue::NaN
            }
            (NumericValue::PositiveInfinity, NumericValue::Decimal(b))
            | (NumericValue::NegativeInfinity, NumericValue::Decimal(b))
                if b.is_zero() =>
            {
                NumericValue::NaN
            }
            (NumericValue::PositiveInfinity, NumericValue::NegativeZero)
            | (NumericValue::NegativeInfinity, NumericValue::NegativeZero)
            | (NumericValue::NegativeZero, NumericValue::PositiveInfinity)
            | (NumericValue::NegativeZero, NumericValue::NegativeInfinity) => NumericValue::NaN,
            // Handle infinity multiplication
            (NumericValue::PositiveInfinity, NumericValue::PositiveInfinity)
            | (NumericValue::NegativeInfinity, NumericValue::NegativeInfinity) => {
                NumericValue::PositiveInfinity
            }
            (NumericValue::PositiveInfinity, NumericValue::NegativeInfinity)
            | (NumericValue::NegativeInfinity, NumericValue::PositiveInfinity) => {
                NumericValue::NegativeInfinity
            }
            // Infinity * finite number
            (NumericValue::PositiveInfinity, NumericValue::Decimal(b))
            | (NumericValue::Decimal(b), NumericValue::PositiveInfinity) => {
                if b > Decimal::ZERO {
                    NumericValue::PositiveInfinity
                } else {
                    NumericValue::NegativeInfinity
                }
            }
            (NumericValue::NegativeInfinity, NumericValue::Decimal(b))
            | (NumericValue::Decimal(b), NumericValue::NegativeInfinity) => {
                if b > Decimal::ZERO {
                    NumericValue::NegativeInfinity
                } else {
                    NumericValue::PositiveInfinity
                }
            }
            // Rational and BigDecimal operations not yet implemented
            _ => unimplemented!("Mul operation with Rational/BigDecimal not yet implemented"),
        }
    }
}

impl Div for NumericValue {
    type Output = NumericValue;
    fn div(self, rhs: NumericValue) -> NumericValue {
        match (self, rhs) {
            (NumericValue::Decimal(a), NumericValue::Decimal(b)) => {
                if b.is_zero() {
                    if a.is_zero() {
                        NumericValue::NaN // 0/0 = NaN
                    } else if a > Decimal::ZERO {
                        NumericValue::PositiveInfinity // positive/0 = +∞
                    } else {
                        NumericValue::NegativeInfinity // negative/0 = -∞
                    }
                } else {
                    // Try division with Decimal first
                    match a.checked_div(b) {
                        Some(result) => {
                            // Check if this is an exact result or needs Rational representation
                            // If result * b != a, we lost precision, use Rational instead
                            if result.checked_mul(b) == Some(a) {
                                NumericValue::Decimal(result)
                            } else {
                                // Graduate to Rational for exact representation
                                use num_rational::Ratio;
                                // Convert Decimals to integers by scaling
                                let a_mantissa = a.mantissa();
                                let a_scale = a.scale();
                                let b_mantissa = b.mantissa();
                                let b_scale = b.scale();

                                // Check if mantissas fit in i64
                                if let (Ok(a_i64), Ok(b_i64)) = (
                                    a_mantissa.try_into() as Result<i64, _>,
                                    b_mantissa.try_into() as Result<i64, _>
                                ) {
                                    // Adjust for scale difference
                                    let rational = if a_scale >= b_scale {
                                        let scale_diff = a_scale - b_scale;
                                        let factor = 10i64.pow(scale_diff);
                                        Ratio::new(a_i64, b_i64 * factor)
                                    } else {
                                        let scale_diff = b_scale - a_scale;
                                        let factor = 10i64.pow(scale_diff);
                                        Ratio::new(a_i64 * factor, b_i64)
                                    };

                                    NumericValue::Rational(rational)
                                } else {
                                    // Mantissa doesn't fit in i64, use BigDecimal
                                    use bigdecimal::BigDecimal;
                                    let a_str = a.to_string();
                                    let b_str = b.to_string();
                                    let a_bd: BigDecimal = a_str.parse().unwrap();
                                    let b_bd: BigDecimal = b_str.parse().unwrap();
                                    NumericValue::BigDecimal(a_bd / b_bd)
                                }
                            }
                        }
                        None => {
                            // Overflow or underflow - graduate to BigDecimal
                            use bigdecimal::BigDecimal;
                            let a_str = a.to_string();
                            let b_str = b.to_string();
                            let a_bd: BigDecimal = a_str.parse().unwrap();
                            let b_bd: BigDecimal = b_str.parse().unwrap();
                            NumericValue::BigDecimal(a_bd / b_bd)
                        }
                    }
                }
            }
            // BigDecimal division
            (NumericValue::BigDecimal(a), NumericValue::BigDecimal(b)) => {
                if b.is_zero() {
                    if a.is_zero() {
                        NumericValue::NaN
                    } else if a.is_positive() {
                        NumericValue::PositiveInfinity
                    } else {
                        NumericValue::NegativeInfinity
                    }
                } else {
                    NumericValue::BigDecimal(a / b)
                }
            }
            // Mixed BigDecimal/Decimal division
            (NumericValue::BigDecimal(a), NumericValue::Decimal(b)) => {
                if b.is_zero() {
                    if a.is_zero() {
                        NumericValue::NaN
                    } else if a.is_positive() {
                        NumericValue::PositiveInfinity
                    } else {
                        NumericValue::NegativeInfinity
                    }
                } else {
                    use bigdecimal::BigDecimal;
                    let b_str = b.to_string();
                    let b_bd: BigDecimal = b_str.parse().unwrap();
                    NumericValue::BigDecimal(a / b_bd)
                }
            }
            (NumericValue::Decimal(a), NumericValue::BigDecimal(b)) => {
                if b.is_zero() {
                    if a.is_zero() {
                        NumericValue::NaN
                    } else if a > Decimal::ZERO {
                        NumericValue::PositiveInfinity
                    } else {
                        NumericValue::NegativeInfinity
                    }
                } else {
                    use bigdecimal::BigDecimal;
                    let a_str = a.to_string();
                    let a_bd: BigDecimal = a_str.parse().unwrap();
                    NumericValue::BigDecimal(a_bd / b)
                }
            }
            (NumericValue::Decimal(a), NumericValue::NegativeZero) => {
                if a.is_zero() {
                    NumericValue::NaN // 0/(-0) = NaN
                } else if a > Decimal::ZERO {
                    NumericValue::NegativeInfinity // positive/(-0) = -∞
                } else {
                    NumericValue::PositiveInfinity // negative/(-0) = +∞
                }
            }
            (NumericValue::NegativeZero, NumericValue::Decimal(b)) => {
                if b.is_zero() {
                    NumericValue::NaN // (-0)/0 = NaN
                } else if b > Decimal::ZERO {
                    NumericValue::NegativeZero // (-0)/positive = -0
                } else {
                    NumericValue::ZERO // (-0)/negative = +0
                }
            }
            (NumericValue::NegativeZero, NumericValue::NegativeZero) => NumericValue::NaN, // (-0)/(-0) = NaN
            (NumericValue::NaN, _) | (_, NumericValue::NaN) => NumericValue::NaN,
            // ∞ / ∞ = NaN, 0 / ∞ = 0
            (NumericValue::PositiveInfinity, NumericValue::PositiveInfinity)
            | (NumericValue::PositiveInfinity, NumericValue::NegativeInfinity)
            | (NumericValue::NegativeInfinity, NumericValue::PositiveInfinity)
            | (NumericValue::NegativeInfinity, NumericValue::NegativeInfinity) => NumericValue::NaN,
            // finite / ∞ = 0 (with appropriate sign)
            (NumericValue::Decimal(_), NumericValue::PositiveInfinity)
            | (NumericValue::Decimal(_), NumericValue::NegativeInfinity) => {
                NumericValue::Decimal(Decimal::ZERO)
            }
            (NumericValue::NegativeZero, NumericValue::PositiveInfinity)
            | (NumericValue::NegativeZero, NumericValue::NegativeInfinity) => {
                NumericValue::NegativeZero
            }
            // ∞ / finite
            (NumericValue::PositiveInfinity, NumericValue::Decimal(b)) => {
                if b > Decimal::ZERO {
                    NumericValue::PositiveInfinity
                } else {
                    NumericValue::NegativeInfinity
                }
            }
            (NumericValue::NegativeInfinity, NumericValue::Decimal(b)) => {
                if b > Decimal::ZERO {
                    NumericValue::NegativeInfinity
                } else {
                    NumericValue::PositiveInfinity
                }
            }
            (NumericValue::PositiveInfinity, NumericValue::NegativeZero) => {
                NumericValue::NegativeInfinity
            }
            (NumericValue::NegativeInfinity, NumericValue::NegativeZero) => {
                NumericValue::PositiveInfinity
            }
            // Rational and BigDecimal operations not yet implemented
            _ => unimplemented!("Div operation with Rational/BigDecimal not yet implemented"),
        }
    }
}

impl Rem for NumericValue {
    type Output = NumericValue;
    fn rem(self, rhs: NumericValue) -> NumericValue {
        match (self, rhs) {
            (NumericValue::Decimal(a), NumericValue::Decimal(b)) => {
                if b.is_zero() {
                    NumericValue::NaN // x % 0 = NaN
                } else {
                    NumericValue::Decimal(a % b)
                }
            }
            (NumericValue::Decimal(_), NumericValue::NegativeZero) => NumericValue::NaN, // x % (-0) = NaN
            (NumericValue::NegativeZero, NumericValue::Decimal(b)) => {
                if b.is_zero() {
                    NumericValue::NaN // (-0) % 0 = NaN
                } else {
                    NumericValue::NegativeZero // (-0) % x = -0
                }
            }
            (NumericValue::NegativeZero, NumericValue::NegativeZero) => NumericValue::NaN, // (-0) % (-0) = NaN
            (NumericValue::NaN, _) | (_, NumericValue::NaN) => NumericValue::NaN,
            // ∞ % anything = NaN, anything % ∞ = the anything
            (NumericValue::PositiveInfinity, _) | (NumericValue::NegativeInfinity, _) => {
                NumericValue::NaN
            }
            (NumericValue::Decimal(a), NumericValue::PositiveInfinity)
            | (NumericValue::Decimal(a), NumericValue::NegativeInfinity) => {
                NumericValue::Decimal(a)
            }
            (NumericValue::NegativeZero, NumericValue::PositiveInfinity)
            | (NumericValue::NegativeZero, NumericValue::NegativeInfinity) => {
                NumericValue::NegativeZero
            }
            // Rational and BigDecimal operations not yet implemented
            _ => unimplemented!("Rem operation with Rational/BigDecimal not yet implemented"),
        }
    }
}

impl Neg for NumericValue {
    type Output = NumericValue;
    fn neg(self) -> NumericValue {
        match self {
            NumericValue::Decimal(d) => {
                if d.is_zero() {
                    NumericValue::NegativeZero // -(+0) = -0
                } else {
                    NumericValue::Decimal(-d)
                }
            }
            NumericValue::NegativeZero => NumericValue::ZERO, // -(-0) = +0
            NumericValue::NaN => NumericValue::NaN,
            NumericValue::PositiveInfinity => NumericValue::NegativeInfinity,
            NumericValue::NegativeInfinity => NumericValue::PositiveInfinity,
            NumericValue::Rational(r) => NumericValue::Rational(-r),
            NumericValue::BigDecimal(bd) => NumericValue::BigDecimal(-bd),
        }
    }
}

// Generate all reference variants for arithmetic operators
forward_ref_binop!(impl Add, add for NumericValue);
forward_ref_binop!(impl Sub, sub for NumericValue);
forward_ref_binop!(impl Mul, mul for NumericValue);
forward_ref_binop!(impl Div, div for NumericValue);
forward_ref_binop!(impl Rem, rem for NumericValue);

// Number wrapper implementations
impl Add for Number {
    type Output = Number;
    fn add(self, rhs: Number) -> Number {
        Number {
            value: self.value + rhs.value,
            approximated: self.approximated || rhs.approximated,
        }
    }
}

impl Sub for Number {
    type Output = Number;
    fn sub(self, rhs: Number) -> Number {
        Number {
            value: self.value - rhs.value,
            approximated: self.approximated || rhs.approximated,
        }
    }
}

impl Mul for Number {
    type Output = Number;
    fn mul(self, rhs: Number) -> Number {
        Number {
            value: self.value * rhs.value,
            approximated: self.approximated || rhs.approximated,
        }
    }
}

impl Div for Number {
    type Output = Number;
    fn div(self, rhs: Number) -> Number {
        Number {
            value: self.value / rhs.value,
            approximated: self.approximated || rhs.approximated,
        }
    }
}

impl Rem for Number {
    type Output = Number;
    fn rem(self, rhs: Number) -> Number {
        Number {
            value: self.value % rhs.value,
            approximated: self.approximated || rhs.approximated,
        }
    }
}

impl Neg for Number {
    type Output = Number;
    fn neg(self) -> Number {
        Number {
            value: -self.value,
            approximated: self.approximated,
        }
    }
}

forward_ref_binop!(impl Add, add for Number);
forward_ref_binop!(impl Sub, sub for Number);
forward_ref_binop!(impl Mul, mul for Number);
forward_ref_binop!(impl Div, div for Number);
forward_ref_binop!(impl Rem, rem for Number);
