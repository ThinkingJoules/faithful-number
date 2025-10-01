use crate::{Number, forward_ref_binop};
use rust_decimal::Decimal;
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};
impl Add for Number {
    type Output = Number;
    fn add(self, rhs: Number) -> Number {
        match (self, rhs) {
            (Number::Finite(a), Number::Finite(b)) => Number::Finite(a + b),
            (Number::Finite(a), Number::NegativeZero) => Number::Finite(a),
            (Number::NegativeZero, Number::Finite(b)) => Number::Finite(b),
            (Number::NegativeZero, Number::NegativeZero) => Number::NegativeZero, // (-0) + (-0) = -0
            (Number::NaN, _) | (_, Number::NaN) => Number::NaN,
            (Number::PositiveInfinity, Number::NegativeInfinity)
            | (Number::NegativeInfinity, Number::PositiveInfinity) => Number::NaN, // ∞ + (-∞) = NaN
            (Number::PositiveInfinity, _) | (_, Number::PositiveInfinity) => {
                Number::PositiveInfinity
            }
            (Number::NegativeInfinity, _) | (_, Number::NegativeInfinity) => {
                Number::NegativeInfinity
            }
        }
    }
}

impl Sub for Number {
    type Output = Number;
    fn sub(self, rhs: Number) -> Number {
        match (self, rhs) {
            (Number::Finite(a), Number::Finite(b)) => Number::Finite(a - b),
            (Number::Finite(a), Number::NegativeZero) => Number::Finite(a), // x - (-0) = x
            (Number::NegativeZero, Number::Finite(b)) => Number::Finite(-b), // (-0) - x = -x
            (Number::NegativeZero, Number::NegativeZero) => Number::ZERO,   // (-0) - (-0) = +0
            (Number::NaN, _) | (_, Number::NaN) => Number::NaN,
            (Number::PositiveInfinity, Number::PositiveInfinity)
            | (Number::NegativeInfinity, Number::NegativeInfinity) => Number::NaN, // ∞ - ∞ = NaN
            (Number::PositiveInfinity, _) => Number::PositiveInfinity,
            (Number::NegativeInfinity, _) => Number::NegativeInfinity,
            (_, Number::PositiveInfinity) => Number::NegativeInfinity,
            (_, Number::NegativeInfinity) => Number::PositiveInfinity,
        }
    }
}

impl Mul for Number {
    type Output = Number;
    fn mul(self, rhs: Number) -> Number {
        match (self, rhs) {
            (Number::Finite(a), Number::Finite(b)) => Number::Finite(a * b),
            (Number::Finite(a), Number::NegativeZero) => {
                if a.is_zero() {
                    Number::NegativeZero // 0 * (-0) = -0 in JS
                } else if a > Decimal::ZERO {
                    Number::NegativeZero // positive * (-0) = -0
                } else {
                    Number::ZERO // negative * (-0) = +0
                }
            }
            (Number::NegativeZero, Number::Finite(b)) => {
                if b.is_zero() {
                    Number::NegativeZero // (-0) * 0 = -0 in JS
                } else if b > Decimal::ZERO {
                    Number::NegativeZero // (-0) * positive = -0
                } else {
                    Number::ZERO // (-0) * negative = +0
                }
            }
            (Number::NegativeZero, Number::NegativeZero) => Number::ZERO, // (-0) * (-0) = +0
            (Number::NaN, _) | (_, Number::NaN) => Number::NaN,
            // 0 * ∞ = NaN in JavaScript
            (Number::Finite(a), Number::PositiveInfinity)
            | (Number::Finite(a), Number::NegativeInfinity)
                if a.is_zero() =>
            {
                Number::NaN
            }
            (Number::PositiveInfinity, Number::Finite(b))
            | (Number::NegativeInfinity, Number::Finite(b))
                if b.is_zero() =>
            {
                Number::NaN
            }
            (Number::PositiveInfinity, Number::NegativeZero)
            | (Number::NegativeInfinity, Number::NegativeZero)
            | (Number::NegativeZero, Number::PositiveInfinity)
            | (Number::NegativeZero, Number::NegativeInfinity) => Number::NaN,
            // Handle infinity multiplication
            (Number::PositiveInfinity, Number::PositiveInfinity)
            | (Number::NegativeInfinity, Number::NegativeInfinity) => Number::PositiveInfinity,
            (Number::PositiveInfinity, Number::NegativeInfinity)
            | (Number::NegativeInfinity, Number::PositiveInfinity) => Number::NegativeInfinity,
            // Infinity * finite number
            (Number::PositiveInfinity, Number::Finite(b))
            | (Number::Finite(b), Number::PositiveInfinity) => {
                if b > Decimal::ZERO {
                    Number::PositiveInfinity
                } else {
                    Number::NegativeInfinity
                }
            }
            (Number::NegativeInfinity, Number::Finite(b))
            | (Number::Finite(b), Number::NegativeInfinity) => {
                if b > Decimal::ZERO {
                    Number::NegativeInfinity
                } else {
                    Number::PositiveInfinity
                }
            }
        }
    }
}

impl Div for Number {
    type Output = Number;
    fn div(self, rhs: Number) -> Number {
        match (self, rhs) {
            (Number::Finite(a), Number::Finite(b)) => {
                if b.is_zero() {
                    if a.is_zero() {
                        Number::NaN // 0/0 = NaN
                    } else if a > Decimal::ZERO {
                        Number::PositiveInfinity // positive/0 = +∞
                    } else {
                        Number::NegativeInfinity // negative/0 = -∞
                    }
                } else {
                    Number::Finite(a / b)
                }
            }
            (Number::Finite(a), Number::NegativeZero) => {
                if a.is_zero() {
                    Number::NaN // 0/(-0) = NaN
                } else if a > Decimal::ZERO {
                    Number::NegativeInfinity // positive/(-0) = -∞
                } else {
                    Number::PositiveInfinity // negative/(-0) = +∞
                }
            }
            (Number::NegativeZero, Number::Finite(b)) => {
                if b.is_zero() {
                    Number::NaN // (-0)/0 = NaN
                } else if b > Decimal::ZERO {
                    Number::NegativeZero // (-0)/positive = -0
                } else {
                    Number::ZERO // (-0)/negative = +0
                }
            }
            (Number::NegativeZero, Number::NegativeZero) => Number::NaN, // (-0)/(-0) = NaN
            (Number::NaN, _) | (_, Number::NaN) => Number::NaN,
            // ∞ / ∞ = NaN, 0 / ∞ = 0
            (Number::PositiveInfinity, Number::PositiveInfinity)
            | (Number::PositiveInfinity, Number::NegativeInfinity)
            | (Number::NegativeInfinity, Number::PositiveInfinity)
            | (Number::NegativeInfinity, Number::NegativeInfinity) => Number::NaN,
            // finite / ∞ = 0 (with appropriate sign)
            (Number::Finite(_), Number::PositiveInfinity)
            | (Number::Finite(_), Number::NegativeInfinity) => Number::Finite(Decimal::ZERO),
            (Number::NegativeZero, Number::PositiveInfinity)
            | (Number::NegativeZero, Number::NegativeInfinity) => Number::NegativeZero,
            // ∞ / finite
            (Number::PositiveInfinity, Number::Finite(b)) => {
                if b > Decimal::ZERO {
                    Number::PositiveInfinity
                } else {
                    Number::NegativeInfinity
                }
            }
            (Number::NegativeInfinity, Number::Finite(b)) => {
                if b > Decimal::ZERO {
                    Number::NegativeInfinity
                } else {
                    Number::PositiveInfinity
                }
            }
            (Number::PositiveInfinity, Number::NegativeZero) => Number::NegativeInfinity,
            (Number::NegativeInfinity, Number::NegativeZero) => Number::PositiveInfinity,
        }
    }
}

impl Rem for Number {
    type Output = Number;
    fn rem(self, rhs: Number) -> Number {
        match (self, rhs) {
            (Number::Finite(a), Number::Finite(b)) => {
                if b.is_zero() {
                    Number::NaN // x % 0 = NaN
                } else {
                    Number::Finite(a % b)
                }
            }
            (Number::Finite(_), Number::NegativeZero) => Number::NaN, // x % (-0) = NaN
            (Number::NegativeZero, Number::Finite(b)) => {
                if b.is_zero() {
                    Number::NaN // (-0) % 0 = NaN
                } else {
                    Number::NegativeZero // (-0) % x = -0
                }
            }
            (Number::NegativeZero, Number::NegativeZero) => Number::NaN, // (-0) % (-0) = NaN
            (Number::NaN, _) | (_, Number::NaN) => Number::NaN,
            // ∞ % anything = NaN, anything % ∞ = the anything
            (Number::PositiveInfinity, _) | (Number::NegativeInfinity, _) => Number::NaN,
            (Number::Finite(a), Number::PositiveInfinity)
            | (Number::Finite(a), Number::NegativeInfinity) => Number::Finite(a),
            (Number::NegativeZero, Number::PositiveInfinity)
            | (Number::NegativeZero, Number::NegativeInfinity) => Number::NegativeZero,
        }
    }
}

impl Neg for Number {
    type Output = Number;
    fn neg(self) -> Number {
        match self {
            Number::Finite(d) => {
                if d.is_zero() {
                    Number::NegativeZero // -(+0) = -0
                } else {
                    Number::Finite(-d)
                }
            }
            Number::NegativeZero => Number::ZERO, // -(-0) = +0
            Number::NaN => Number::NaN,
            Number::PositiveInfinity => Number::NegativeInfinity,
            Number::NegativeInfinity => Number::PositiveInfinity,
        }
    }
}

// Generate all reference variants for arithmetic operators
forward_ref_binop!(impl Add, add for Number);
forward_ref_binop!(impl Sub, sub for Number);
forward_ref_binop!(impl Mul, mul for Number);
forward_ref_binop!(impl Div, div for Number);
forward_ref_binop!(impl Rem, rem for Number);
