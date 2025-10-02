use crate::{Number, NumericValue, forward_ref_binop};
use num_traits::{CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, Signed, Zero};
use rust_decimal::Decimal;
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};
impl Add for NumericValue {
    type Output = NumericValue;
    fn add(self, rhs: NumericValue) -> NumericValue {
        match (self, rhs) {
            // Rational + Rational: stays Rational, or graduates to Decimal/BigDecimal if denominator overflows
            (NumericValue::Rational(a), NumericValue::Rational(b)) => {
                // Try rational addition
                if let Some(result) = a.checked_add(&b) {
                    NumericValue::Rational(result)
                } else {
                    use crate::core::is_terminating_decimal;

                    // Denominator overflow - check if either is non-terminating
                    let a_non_terminating = !is_terminating_decimal(*a.numer(), *a.denom());
                    let b_non_terminating = !is_terminating_decimal(*b.numer(), *b.denom());

                    if a_non_terminating || b_non_terminating {
                        // Non-terminating: promote directly to BigDecimal
                        use bigdecimal::{BigDecimal, num_bigint::BigInt};
                        let a_numer_bd = BigDecimal::from(BigInt::from(*a.numer()));
                        let a_denom_bd = BigDecimal::from(BigInt::from(*a.denom()));
                        let a_bd = a_numer_bd / a_denom_bd;
                        let b_numer_bd = BigDecimal::from(BigInt::from(*b.numer()));
                        let b_denom_bd = BigDecimal::from(BigInt::from(*b.denom()));
                        let b_bd = b_numer_bd / b_denom_bd;
                        NumericValue::BigDecimal(a_bd + b_bd)
                    } else {
                        // Terminating: try Decimal first
                        let a_dec = Decimal::from(*a.numer()) / Decimal::from(*a.denom());
                        let b_dec = Decimal::from(*b.numer()) / Decimal::from(*b.denom());
                        match a_dec.checked_add(b_dec) {
                            Some(result) => NumericValue::Decimal(result),
                            None => {
                                // Graduate to BigDecimal
                                use bigdecimal::BigDecimal;
                                let a_str = a_dec.to_string();
                                let b_str = b_dec.to_string();
                                let a_bd: BigDecimal = a_str.parse().unwrap();
                                let b_bd: BigDecimal = b_str.parse().unwrap();
                                NumericValue::BigDecimal(a_bd + b_bd)
                            }
                        }
                    }
                }
            }

            // Rational + Decimal: graduate Rational to Decimal or BigDecimal
            (NumericValue::Rational(a), NumericValue::Decimal(b))
            | (NumericValue::Decimal(b), NumericValue::Rational(a)) => {
                use crate::core::is_terminating_decimal;

                // Check if rational is non-terminating
                let a_non_terminating = !is_terminating_decimal(*a.numer(), *a.denom());

                if a_non_terminating {
                    // Non-terminating: promote directly to BigDecimal
                    use bigdecimal::{BigDecimal, num_bigint::BigInt};
                    let numer_bd = BigDecimal::from(BigInt::from(*a.numer()));
                    let denom_bd = BigDecimal::from(BigInt::from(*a.denom()));
                    let a_bd = numer_bd / denom_bd;
                    let b_str = b.to_string();
                    let b_bd: BigDecimal = b_str.parse().unwrap();
                    NumericValue::BigDecimal(a_bd + b_bd)
                } else {
                    // Terminating: try Decimal first
                    let a_dec = Decimal::from(*a.numer()) / Decimal::from(*a.denom());
                    match a_dec.checked_add(b) {
                        Some(result) => NumericValue::from_decimal(result),
                        None => {
                            // Graduate to BigDecimal
                            use bigdecimal::BigDecimal;
                            let a_str = a_dec.to_string();
                            let b_str = b.to_string();
                            let a_bd: BigDecimal = a_str.parse().unwrap();
                            let b_bd: BigDecimal = b_str.parse().unwrap();
                            NumericValue::BigDecimal(a_bd + b_bd)
                        }
                    }
                }
            }

            // Rational + BigDecimal: graduate Rational to BigDecimal
            (NumericValue::Rational(a), NumericValue::BigDecimal(b))
            | (NumericValue::BigDecimal(b), NumericValue::Rational(a)) => {
                use bigdecimal::{BigDecimal, num_bigint::BigInt};
                let numer_bd = BigDecimal::from(BigInt::from(*a.numer()));
                let denom_bd = BigDecimal::from(BigInt::from(*a.denom()));
                let a_bd = numer_bd / denom_bd;
                NumericValue::BigDecimal(a_bd + b)
            }

            // Decimal + Decimal
            (NumericValue::Decimal(a), NumericValue::Decimal(b)) => match a.checked_add(b) {
                Some(result) => NumericValue::Decimal(result),
                None => {
                    use bigdecimal::BigDecimal;
                    let a_str = a.to_string();
                    let b_str = b.to_string();
                    let a_bd: BigDecimal = a_str.parse().unwrap();
                    let b_bd: BigDecimal = b_str.parse().unwrap();
                    NumericValue::BigDecimal(a_bd + b_bd)
                }
            },

            // Special cases with NegativeZero
            (NumericValue::Rational(a), NumericValue::NegativeZero)
            | (NumericValue::NegativeZero, NumericValue::Rational(a)) => NumericValue::Rational(a),
            (NumericValue::Decimal(a), NumericValue::NegativeZero)
            | (NumericValue::NegativeZero, NumericValue::Decimal(a)) => NumericValue::Decimal(a),
            (NumericValue::BigDecimal(a), NumericValue::NegativeZero)
            | (NumericValue::NegativeZero, NumericValue::BigDecimal(a)) => {
                NumericValue::BigDecimal(a)
            }
            (NumericValue::NegativeZero, NumericValue::NegativeZero) => NumericValue::NegativeZero,

            // BigDecimal operations
            (NumericValue::BigDecimal(a), NumericValue::BigDecimal(b)) => {
                NumericValue::BigDecimal(a + b)
            }
            (NumericValue::BigDecimal(a), NumericValue::Decimal(b))
            | (NumericValue::Decimal(b), NumericValue::BigDecimal(a)) => {
                use bigdecimal::BigDecimal;
                let b_str = b.to_string();
                let b_bd: BigDecimal = b_str.parse().unwrap();
                NumericValue::BigDecimal(a + b_bd)
            }

            // NaN and Infinity handling
            (NumericValue::NaN, _) | (_, NumericValue::NaN) => NumericValue::NaN,
            (NumericValue::PositiveInfinity, NumericValue::NegativeInfinity)
            | (NumericValue::NegativeInfinity, NumericValue::PositiveInfinity) => NumericValue::NaN,
            (NumericValue::PositiveInfinity, _) | (_, NumericValue::PositiveInfinity) => {
                NumericValue::PositiveInfinity
            }
            (NumericValue::NegativeInfinity, _) | (_, NumericValue::NegativeInfinity) => {
                NumericValue::NegativeInfinity
            }
        }
    }
}

impl Sub for NumericValue {
    type Output = NumericValue;
    fn sub(self, rhs: NumericValue) -> NumericValue {
        match (self, rhs) {
            // Rational - Rational: stays Rational, or graduates to Decimal if denominator overflows
            (NumericValue::Rational(a), NumericValue::Rational(b)) => {
                if let Some(result) = a.checked_sub(&b) {
                    NumericValue::Rational(result)
                } else {
                    // Denominator overflow - graduate to Decimal
                    let a_dec = Decimal::from(*a.numer()) / Decimal::from(*a.denom());
                    let b_dec = Decimal::from(*b.numer()) / Decimal::from(*b.denom());
                    match a_dec.checked_sub(b_dec) {
                        Some(result) => NumericValue::Decimal(result),
                        None => {
                            // Graduate to BigDecimal
                            use bigdecimal::BigDecimal;
                            let a_str = a_dec.to_string();
                            let b_str = b_dec.to_string();
                            let a_bd: BigDecimal = a_str.parse().unwrap();
                            let b_bd: BigDecimal = b_str.parse().unwrap();
                            NumericValue::BigDecimal(a_bd - b_bd)
                        }
                    }
                }
            }

            // Rational - Decimal: graduate Rational to Decimal
            (NumericValue::Rational(a), NumericValue::Decimal(b)) => {
                let a_dec = Decimal::from(*a.numer()) / Decimal::from(*a.denom());
                match a_dec.checked_sub(b) {
                    Some(result) => NumericValue::from_decimal(result),
                    None => {
                        // Graduate to BigDecimal
                        use bigdecimal::BigDecimal;
                        let a_str = a_dec.to_string();
                        let b_str = b.to_string();
                        let a_bd: BigDecimal = a_str.parse().unwrap();
                        let b_bd: BigDecimal = b_str.parse().unwrap();
                        NumericValue::BigDecimal(a_bd - b_bd)
                    }
                }
            }
            // Decimal - Rational: graduate Rational to Decimal
            (NumericValue::Decimal(a), NumericValue::Rational(b)) => {
                let b_dec = Decimal::from(*b.numer()) / Decimal::from(*b.denom());
                match a.checked_sub(b_dec) {
                    Some(result) => NumericValue::from_decimal(result),
                    None => {
                        // Graduate to BigDecimal
                        use bigdecimal::BigDecimal;
                        let a_str = a.to_string();
                        let b_str = b_dec.to_string();
                        let a_bd: BigDecimal = a_str.parse().unwrap();
                        let b_bd: BigDecimal = b_str.parse().unwrap();
                        NumericValue::BigDecimal(a_bd - b_bd)
                    }
                }
            }

            // Rational - BigDecimal: graduate Rational to BigDecimal
            (NumericValue::Rational(a), NumericValue::BigDecimal(b)) => {
                use bigdecimal::{BigDecimal, num_bigint::BigInt};
                let numer_bd = BigDecimal::from(BigInt::from(*a.numer()));
                let denom_bd = BigDecimal::from(BigInt::from(*a.denom()));
                let a_bd = numer_bd / denom_bd;
                NumericValue::BigDecimal(a_bd - b)
            }
            (NumericValue::BigDecimal(a), NumericValue::Rational(b)) => {
                use bigdecimal::{BigDecimal, num_bigint::BigInt};
                let numer_bd = BigDecimal::from(BigInt::from(*b.numer()));
                let denom_bd = BigDecimal::from(BigInt::from(*b.denom()));
                let b_bd = numer_bd / denom_bd;
                NumericValue::BigDecimal(a - b_bd)
            }

            // Decimal - Decimal
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

            // Special cases with NegativeZero
            (NumericValue::Rational(a), NumericValue::NegativeZero) => NumericValue::Rational(a),
            (NumericValue::NegativeZero, NumericValue::Rational(a)) => NumericValue::Rational(-a),
            (NumericValue::Decimal(a), NumericValue::NegativeZero) => NumericValue::Decimal(a), // x - (-0) = x
            (NumericValue::NegativeZero, NumericValue::Decimal(b)) => NumericValue::Decimal(-b), // (-0) - x = -x
            (NumericValue::BigDecimal(a), NumericValue::NegativeZero) => {
                NumericValue::BigDecimal(a)
            }
            (NumericValue::NegativeZero, NumericValue::BigDecimal(b)) => {
                NumericValue::BigDecimal(-b)
            }
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
        }
    }
}

impl Mul for NumericValue {
    type Output = NumericValue;
    fn mul(self, rhs: NumericValue) -> NumericValue {
        match (self, rhs) {
            // Rational * Rational: stays Rational, or graduates to Decimal if denominator overflows
            (NumericValue::Rational(a), NumericValue::Rational(b)) => {
                if let Some(result) = a.checked_mul(&b) {
                    NumericValue::Rational(result)
                } else {
                    // Overflow - graduate to Decimal
                    let a_dec = Decimal::from(*a.numer()) / Decimal::from(*a.denom());
                    let b_dec = Decimal::from(*b.numer()) / Decimal::from(*b.denom());
                    match a_dec.checked_mul(b_dec) {
                        Some(result) => NumericValue::Decimal(result),
                        None => {
                            // Graduate to BigDecimal
                            use bigdecimal::BigDecimal;
                            let a_str = a_dec.to_string();
                            let b_str = b_dec.to_string();
                            let a_bd: BigDecimal = a_str.parse().unwrap();
                            let b_bd: BigDecimal = b_str.parse().unwrap();
                            NumericValue::BigDecimal(a_bd * b_bd)
                        }
                    }
                }
            }

            // Rational * Decimal: graduate Rational to Decimal
            (NumericValue::Rational(a), NumericValue::Decimal(b))
            | (NumericValue::Decimal(b), NumericValue::Rational(a)) => {
                let a_dec = Decimal::from(*a.numer()) / Decimal::from(*a.denom());
                match a_dec.checked_mul(b) {
                    Some(result) => NumericValue::from_decimal(result),
                    None => {
                        // Graduate to BigDecimal
                        use bigdecimal::BigDecimal;
                        let a_str = a_dec.to_string();
                        let b_str = b.to_string();
                        let a_bd: BigDecimal = a_str.parse().unwrap();
                        let b_bd: BigDecimal = b_str.parse().unwrap();
                        NumericValue::BigDecimal(a_bd * b_bd)
                    }
                }
            }

            // Rational * BigDecimal: graduate Rational to BigDecimal
            (NumericValue::Rational(a), NumericValue::BigDecimal(b))
            | (NumericValue::BigDecimal(b), NumericValue::Rational(a)) => {
                use bigdecimal::{BigDecimal, num_bigint::BigInt};
                let numer_bd = BigDecimal::from(BigInt::from(*a.numer()));
                let denom_bd = BigDecimal::from(BigInt::from(*a.denom()));
                let a_bd = numer_bd / denom_bd;
                NumericValue::BigDecimal(a_bd * b)
            }

            // Decimal * Decimal
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

            // BigDecimal operations
            (NumericValue::BigDecimal(a), NumericValue::BigDecimal(b)) => {
                NumericValue::BigDecimal(a * b)
            }
            (NumericValue::BigDecimal(a), NumericValue::Decimal(b))
            | (NumericValue::Decimal(b), NumericValue::BigDecimal(a)) => {
                use bigdecimal::BigDecimal;
                let b_str = b.to_string();
                let b_bd: BigDecimal = b_str.parse().unwrap();
                NumericValue::BigDecimal(a * b_bd)
            }

            // Special cases with NegativeZero and Rational
            (NumericValue::Rational(a), NumericValue::NegativeZero)
            | (NumericValue::NegativeZero, NumericValue::Rational(a)) => {
                if a.is_zero() {
                    NumericValue::NegativeZero // 0 * (-0) = -0
                } else if *a.numer() > 0 {
                    NumericValue::NegativeZero // positive * (-0) = -0
                } else {
                    NumericValue::ZERO // negative * (-0) = +0
                }
            }
            (NumericValue::Decimal(a), NumericValue::NegativeZero)
            | (NumericValue::NegativeZero, NumericValue::Decimal(a)) => {
                if a.is_zero() {
                    NumericValue::NegativeZero // 0 * (-0) = -0 in JS
                } else if a > Decimal::ZERO {
                    NumericValue::NegativeZero // positive * (-0) = -0
                } else {
                    NumericValue::ZERO // negative * (-0) = +0
                }
            }
            (NumericValue::BigDecimal(a), NumericValue::NegativeZero)
            | (NumericValue::NegativeZero, NumericValue::BigDecimal(a)) => {
                if a.is_zero() {
                    NumericValue::NegativeZero
                } else if a.is_positive() {
                    NumericValue::NegativeZero
                } else {
                    NumericValue::ZERO
                }
            }
            (NumericValue::NegativeZero, NumericValue::NegativeZero) => NumericValue::ZERO, // (-0) * (-0) = +0

            (NumericValue::NaN, _) | (_, NumericValue::NaN) => NumericValue::NaN,

            // 0 * ∞ = NaN in JavaScript (Rational case)
            (NumericValue::Rational(a), NumericValue::PositiveInfinity)
            | (NumericValue::Rational(a), NumericValue::NegativeInfinity)
            | (NumericValue::PositiveInfinity, NumericValue::Rational(a))
            | (NumericValue::NegativeInfinity, NumericValue::Rational(a))
                if a.is_zero() =>
            {
                NumericValue::NaN
            }
            // 0 * ∞ = NaN in JavaScript (Decimal case)
            (NumericValue::Decimal(a), NumericValue::PositiveInfinity)
            | (NumericValue::Decimal(a), NumericValue::NegativeInfinity)
            | (NumericValue::PositiveInfinity, NumericValue::Decimal(a))
            | (NumericValue::NegativeInfinity, NumericValue::Decimal(a))
                if a.is_zero() =>
            {
                NumericValue::NaN
            }
            // 0 * ∞ = NaN in JavaScript (BigDecimal case)
            (NumericValue::BigDecimal(a), NumericValue::PositiveInfinity)
            | (NumericValue::BigDecimal(a), NumericValue::NegativeInfinity)
            | (NumericValue::PositiveInfinity, NumericValue::BigDecimal(a))
            | (NumericValue::NegativeInfinity, NumericValue::BigDecimal(a))
                if a.is_zero() =>
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

            // Infinity * finite Rational
            (NumericValue::PositiveInfinity, NumericValue::Rational(a))
            | (NumericValue::Rational(a), NumericValue::PositiveInfinity) => {
                if *a.numer() > 0 {
                    NumericValue::PositiveInfinity
                } else {
                    NumericValue::NegativeInfinity
                }
            }
            (NumericValue::NegativeInfinity, NumericValue::Rational(a))
            | (NumericValue::Rational(a), NumericValue::NegativeInfinity) => {
                if *a.numer() > 0 {
                    NumericValue::NegativeInfinity
                } else {
                    NumericValue::PositiveInfinity
                }
            }
            // Infinity * finite Decimal
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
            // Infinity * finite BigDecimal
            (NumericValue::PositiveInfinity, NumericValue::BigDecimal(b))
            | (NumericValue::BigDecimal(b), NumericValue::PositiveInfinity) => {
                if b.is_positive() {
                    NumericValue::PositiveInfinity
                } else {
                    NumericValue::NegativeInfinity
                }
            }
            (NumericValue::NegativeInfinity, NumericValue::BigDecimal(b))
            | (NumericValue::BigDecimal(b), NumericValue::NegativeInfinity) => {
                if b.is_positive() {
                    NumericValue::NegativeInfinity
                } else {
                    NumericValue::PositiveInfinity
                }
            }
        }
    }
}

impl Div for NumericValue {
    type Output = NumericValue;
    fn div(self, rhs: NumericValue) -> NumericValue {
        use num_rational::Ratio;

        match (self, rhs) {
            // Rational / Rational: stays Rational (invert and multiply), or graduates to Decimal if overflow
            (NumericValue::Rational(a), NumericValue::Rational(b)) => {
                if b.is_zero() {
                    if a.is_zero() {
                        NumericValue::NaN // 0/0 = NaN
                    } else if *a.numer() > 0 {
                        NumericValue::PositiveInfinity // positive/0 = +∞
                    } else {
                        NumericValue::NegativeInfinity // negative/0 = -∞
                    }
                } else if let Some(result) = a.checked_div(&b) {
                    NumericValue::Rational(result)
                } else {
                    // Overflow - graduate to Decimal
                    let a_dec = Decimal::from(*a.numer()) / Decimal::from(*a.denom());
                    let b_dec = Decimal::from(*b.numer()) / Decimal::from(*b.denom());
                    match a_dec.checked_div(b_dec) {
                        Some(result) => NumericValue::Decimal(result),
                        None => {
                            // Graduate to BigDecimal
                            use bigdecimal::BigDecimal;
                            let a_str = a_dec.to_string();
                            let b_str = b_dec.to_string();
                            let a_bd: BigDecimal = a_str.parse().unwrap();
                            let b_bd: BigDecimal = b_str.parse().unwrap();
                            NumericValue::BigDecimal(a_bd / b_bd)
                        }
                    }
                }
            }

            // Rational / Decimal: graduate Rational to Decimal
            (NumericValue::Rational(a), NumericValue::Decimal(b)) => {
                if b.is_zero() {
                    if a.is_zero() {
                        NumericValue::NaN
                    } else if *a.numer() > 0 {
                        NumericValue::PositiveInfinity
                    } else {
                        NumericValue::NegativeInfinity
                    }
                } else {
                    let a_dec = Decimal::from(*a.numer()) / Decimal::from(*a.denom());
                    match a_dec.checked_div(b) {
                        Some(result) => NumericValue::from_decimal(result),
                        None => {
                            // Graduate to BigDecimal
                            use bigdecimal::BigDecimal;
                            let a_str = a_dec.to_string();
                            let b_str = b.to_string();
                            let a_bd: BigDecimal = a_str.parse().unwrap();
                            let b_bd: BigDecimal = b_str.parse().unwrap();
                            NumericValue::BigDecimal(a_bd / b_bd)
                        }
                    }
                }
            }
            // Decimal / Rational: graduate Rational to Decimal
            (NumericValue::Decimal(a), NumericValue::Rational(b)) => {
                if b.is_zero() {
                    if a.is_zero() {
                        NumericValue::NaN
                    } else if a > Decimal::ZERO {
                        NumericValue::PositiveInfinity
                    } else {
                        NumericValue::NegativeInfinity
                    }
                } else {
                    let b_dec = Decimal::from(*b.numer()) / Decimal::from(*b.denom());
                    match a.checked_div(b_dec) {
                        Some(result) => NumericValue::from_decimal(result),
                        None => {
                            // Graduate to BigDecimal
                            use bigdecimal::BigDecimal;
                            let a_str = a.to_string();
                            let b_str = b_dec.to_string();
                            let a_bd: BigDecimal = a_str.parse().unwrap();
                            let b_bd: BigDecimal = b_str.parse().unwrap();
                            NumericValue::BigDecimal(a_bd / b_bd)
                        }
                    }
                }
            }

            // Rational / BigDecimal: graduate Rational to BigDecimal
            (NumericValue::Rational(a), NumericValue::BigDecimal(b)) => {
                if b.is_zero() {
                    if a.is_zero() {
                        NumericValue::NaN
                    } else if *a.numer() > 0 {
                        NumericValue::PositiveInfinity
                    } else {
                        NumericValue::NegativeInfinity
                    }
                } else {
                    use bigdecimal::{BigDecimal, num_bigint::BigInt};
                    let numer_bd = BigDecimal::from(BigInt::from(*a.numer()));
                    let denom_bd = BigDecimal::from(BigInt::from(*a.denom()));
                    let a_bd = numer_bd / denom_bd;
                    NumericValue::BigDecimal(a_bd / b)
                }
            }
            // BigDecimal / Rational: graduate Rational to BigDecimal
            (NumericValue::BigDecimal(a), NumericValue::Rational(b)) => {
                if b.is_zero() {
                    if a.is_zero() {
                        NumericValue::NaN
                    } else if a.is_positive() {
                        NumericValue::PositiveInfinity
                    } else {
                        NumericValue::NegativeInfinity
                    }
                } else {
                    use bigdecimal::{BigDecimal, num_bigint::BigInt};
                    let numer_bd = BigDecimal::from(BigInt::from(*b.numer()));
                    let denom_bd = BigDecimal::from(BigInt::from(*b.denom()));
                    let b_bd = numer_bd / denom_bd;
                    NumericValue::BigDecimal(a / b_bd)
                }
            }

            // Special cases with NegativeZero and Rational
            (NumericValue::Rational(a), NumericValue::NegativeZero) => {
                if a.is_zero() {
                    NumericValue::NaN // 0/(-0) = NaN
                } else if *a.numer() > 0 {
                    NumericValue::NegativeInfinity // positive/(-0) = -∞
                } else {
                    NumericValue::PositiveInfinity // negative/(-0) = +∞
                }
            }
            (NumericValue::NegativeZero, NumericValue::Rational(b)) => {
                if b.is_zero() {
                    NumericValue::NaN // (-0)/0 = NaN
                } else if *b.numer() > 0 {
                    NumericValue::NegativeZero // (-0)/positive = -0
                } else {
                    NumericValue::ZERO // (-0)/negative = +0
                }
            }
            // Special cases with NegativeZero and BigDecimal
            (NumericValue::BigDecimal(a), NumericValue::NegativeZero) => {
                if a.is_zero() {
                    NumericValue::NaN // 0/(-0) = NaN
                } else if a.is_positive() {
                    NumericValue::NegativeInfinity // positive/(-0) = -∞
                } else {
                    NumericValue::PositiveInfinity // negative/(-0) = +∞
                }
            }
            (NumericValue::NegativeZero, NumericValue::BigDecimal(b)) => {
                if b.is_zero() {
                    NumericValue::NaN // (-0)/0 = NaN
                } else if b.is_positive() {
                    NumericValue::NegativeZero // (-0)/positive = -0
                } else {
                    NumericValue::ZERO // (-0)/negative = +0
                }
            }

            // Decimal / Decimal
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
                                    b_mantissa.try_into() as Result<i64, _>,
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
            (NumericValue::Rational(_), NumericValue::PositiveInfinity)
            | (NumericValue::Rational(_), NumericValue::NegativeInfinity) => {
                NumericValue::Rational(Ratio::from_integer(0))
            }
            (NumericValue::Decimal(_), NumericValue::PositiveInfinity)
            | (NumericValue::Decimal(_), NumericValue::NegativeInfinity) => {
                NumericValue::Decimal(Decimal::ZERO)
            }
            (NumericValue::BigDecimal(_), NumericValue::PositiveInfinity)
            | (NumericValue::BigDecimal(_), NumericValue::NegativeInfinity) => {
                use bigdecimal::BigDecimal;
                NumericValue::BigDecimal(BigDecimal::from(0))
            }
            (NumericValue::NegativeZero, NumericValue::PositiveInfinity)
            | (NumericValue::NegativeZero, NumericValue::NegativeInfinity) => {
                NumericValue::NegativeZero
            }

            // ∞ / finite Rational
            (NumericValue::PositiveInfinity, NumericValue::Rational(b)) => {
                if *b.numer() > 0 {
                    NumericValue::PositiveInfinity
                } else {
                    NumericValue::NegativeInfinity
                }
            }
            (NumericValue::NegativeInfinity, NumericValue::Rational(b)) => {
                if *b.numer() > 0 {
                    NumericValue::NegativeInfinity
                } else {
                    NumericValue::PositiveInfinity
                }
            }
            // ∞ / finite Decimal
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
            // ∞ / finite BigDecimal
            (NumericValue::PositiveInfinity, NumericValue::BigDecimal(b)) => {
                if b.is_positive() {
                    NumericValue::PositiveInfinity
                } else {
                    NumericValue::NegativeInfinity
                }
            }
            (NumericValue::NegativeInfinity, NumericValue::BigDecimal(b)) => {
                if b.is_positive() {
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
        }
    }
}

impl Rem for NumericValue {
    type Output = NumericValue;
    fn rem(self, rhs: NumericValue) -> NumericValue {
        match (self, rhs) {
            // Rational % Rational: convert to Decimal for operation
            (NumericValue::Rational(a), NumericValue::Rational(b)) => {
                if b.is_zero() {
                    NumericValue::NaN
                } else {
                    let a_dec = Decimal::from(*a.numer()) / Decimal::from(*a.denom());
                    let b_dec = Decimal::from(*b.numer()) / Decimal::from(*b.denom());
                    NumericValue::Decimal(a_dec % b_dec)
                }
            }
            (NumericValue::Rational(a), NumericValue::Decimal(b)) => {
                if b.is_zero() {
                    NumericValue::NaN
                } else {
                    let a_dec = Decimal::from(*a.numer()) / Decimal::from(*a.denom());
                    NumericValue::Decimal(a_dec % b)
                }
            }
            (NumericValue::Decimal(a), NumericValue::Rational(b)) => {
                if b.is_zero() {
                    NumericValue::NaN
                } else {
                    let b_dec = Decimal::from(*b.numer()) / Decimal::from(*b.denom());
                    NumericValue::Decimal(a % b_dec)
                }
            }
            (NumericValue::Rational(a), NumericValue::BigDecimal(b)) => {
                if b.is_zero() {
                    NumericValue::NaN
                } else {
                    use bigdecimal::{BigDecimal, num_bigint::BigInt};
                    let numer_bd = BigDecimal::from(BigInt::from(*a.numer()));
                    let denom_bd = BigDecimal::from(BigInt::from(*a.denom()));
                    let a_bd = numer_bd / denom_bd;
                    NumericValue::BigDecimal(a_bd % b)
                }
            }
            (NumericValue::BigDecimal(a), NumericValue::Rational(b)) => {
                if b.is_zero() {
                    NumericValue::NaN
                } else {
                    use bigdecimal::{BigDecimal, num_bigint::BigInt};
                    let numer_bd = BigDecimal::from(BigInt::from(*b.numer()));
                    let denom_bd = BigDecimal::from(BigInt::from(*b.denom()));
                    let b_bd = numer_bd / denom_bd;
                    NumericValue::BigDecimal(a % b_bd)
                }
            }
            (NumericValue::Rational(_a), NumericValue::NegativeZero) => NumericValue::NaN,
            (NumericValue::NegativeZero, NumericValue::Rational(b)) => {
                if b.is_zero() {
                    NumericValue::NaN
                } else {
                    NumericValue::NegativeZero
                }
            }

            // BigDecimal % operations
            (NumericValue::BigDecimal(a), NumericValue::BigDecimal(b)) => {
                if b.is_zero() {
                    NumericValue::NaN
                } else {
                    NumericValue::BigDecimal(a % b)
                }
            }
            (NumericValue::BigDecimal(a), NumericValue::Decimal(b)) => {
                if b.is_zero() {
                    NumericValue::NaN
                } else {
                    use bigdecimal::BigDecimal;
                    let b_str = b.to_string();
                    let b_bd: BigDecimal = b_str.parse().unwrap();
                    NumericValue::BigDecimal(a % b_bd)
                }
            }
            (NumericValue::Decimal(a), NumericValue::BigDecimal(b)) => {
                if b.is_zero() {
                    NumericValue::NaN
                } else {
                    use bigdecimal::BigDecimal;
                    let a_str = a.to_string();
                    let a_bd: BigDecimal = a_str.parse().unwrap();
                    NumericValue::BigDecimal(a_bd % b)
                }
            }
            (NumericValue::BigDecimal(_), NumericValue::NegativeZero) => NumericValue::NaN,
            (NumericValue::NegativeZero, NumericValue::BigDecimal(b)) => {
                if b.is_zero() {
                    NumericValue::NaN
                } else {
                    NumericValue::NegativeZero
                }
            }

            // Decimal % Decimal
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
            (NumericValue::Rational(a), NumericValue::PositiveInfinity)
            | (NumericValue::Rational(a), NumericValue::NegativeInfinity) => {
                NumericValue::Rational(a)
            }
            (NumericValue::Decimal(a), NumericValue::PositiveInfinity)
            | (NumericValue::Decimal(a), NumericValue::NegativeInfinity) => {
                NumericValue::Decimal(a)
            }
            (NumericValue::BigDecimal(a), NumericValue::PositiveInfinity)
            | (NumericValue::BigDecimal(a), NumericValue::NegativeInfinity) => {
                NumericValue::BigDecimal(a)
            }
            (NumericValue::NegativeZero, NumericValue::PositiveInfinity)
            | (NumericValue::NegativeZero, NumericValue::NegativeInfinity) => {
                NumericValue::NegativeZero
            }
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
        use crate::ApproximationType;
        use crate::core::is_terminating_decimal;

        // Check flags and types BEFORE moving
        let self_trans = self.is_transcendental();
        let rhs_trans = rhs.is_transcendental();
        let self_rat_approx = self.is_rational_approximation();
        let rhs_rat_approx = rhs.is_rational_approximation();
        let self_rational = matches!(self.value, NumericValue::Rational(_));
        let rhs_rational = matches!(rhs.value, NumericValue::Rational(_));

        // Check if either rational is non-terminating
        let self_non_terminating = if let NumericValue::Rational(r) = &self.value {
            !is_terminating_decimal(*r.numer(), *r.denom())
        } else {
            false
        };
        let rhs_non_terminating = if let NumericValue::Rational(r) = &rhs.value {
            !is_terminating_decimal(*r.numer(), *r.denom())
        } else {
            false
        };

        // Compute ONCE
        let result_value = self.value + rhs.value;

        // Transcendental dominates
        let apprx = if self_trans || rhs_trans {
            Some(ApproximationType::Transcendental)
        } else if self_rat_approx || rhs_rat_approx {
            // Check if result went back to Rational (flag should clear)
            if matches!(result_value, NumericValue::Rational(_)) {
                None
            } else {
                Some(ApproximationType::RationalApproximation)
            }
        } else if (self_rational || rhs_rational)
            && (matches!(result_value, NumericValue::Decimal(_))
                || matches!(result_value, NumericValue::BigDecimal(_)))
        {
            // Rational graduated to Decimal or BigDecimal
            // Only set flag if a NON-TERMINATING rational graduated
            if self_non_terminating || rhs_non_terminating {
                Some(ApproximationType::RationalApproximation)
            } else {
                None // Terminating rational -> Decimal is exact
            }
        } else {
            None
        };

        Number {
            value: result_value,
            apprx,
        }
    }
}

impl Sub for Number {
    type Output = Number;
    fn sub(self, rhs: Number) -> Number {
        use crate::ApproximationType;
        use crate::core::is_terminating_decimal;

        // Check flags and types BEFORE moving
        let self_trans = self.is_transcendental();
        let rhs_trans = rhs.is_transcendental();
        let self_rat_approx = self.is_rational_approximation();
        let rhs_rat_approx = rhs.is_rational_approximation();
        let self_rational = matches!(self.value, NumericValue::Rational(_));
        let rhs_rational = matches!(rhs.value, NumericValue::Rational(_));

        // Check if either rational is non-terminating
        let self_non_terminating = if let NumericValue::Rational(r) = &self.value {
            !is_terminating_decimal(*r.numer(), *r.denom())
        } else {
            false
        };
        let rhs_non_terminating = if let NumericValue::Rational(r) = &rhs.value {
            !is_terminating_decimal(*r.numer(), *r.denom())
        } else {
            false
        };

        // Compute ONCE
        let result_value = self.value - rhs.value;

        let apprx = if self_trans || rhs_trans {
            Some(ApproximationType::Transcendental)
        } else if self_rat_approx || rhs_rat_approx {
            if matches!(result_value, NumericValue::Rational(_)) {
                None
            } else {
                Some(ApproximationType::RationalApproximation)
            }
        } else if (self_rational || rhs_rational)
            && matches!(result_value, NumericValue::Decimal(_))
        {
            // Only set flag if a NON-TERMINATING rational graduated
            if self_non_terminating || rhs_non_terminating {
                Some(ApproximationType::RationalApproximation)
            } else {
                None // Terminating rational -> Decimal is exact
            }
        } else {
            None
        };

        Number {
            value: result_value,
            apprx,
        }
        .try_demote()
    }
}

impl Mul for Number {
    type Output = Number;
    fn mul(self, rhs: Number) -> Number {
        use crate::ApproximationType;
        use crate::core::is_terminating_decimal;

        // Check flags and types BEFORE moving
        let self_trans = self.is_transcendental();
        let rhs_trans = rhs.is_transcendental();
        let self_rat_approx = self.is_rational_approximation();
        let rhs_rat_approx = rhs.is_rational_approximation();
        let self_rational = matches!(self.value, NumericValue::Rational(_));
        let rhs_rational = matches!(rhs.value, NumericValue::Rational(_));

        // Check if either rational is non-terminating
        let self_non_terminating = if let NumericValue::Rational(r) = &self.value {
            !is_terminating_decimal(*r.numer(), *r.denom())
        } else {
            false
        };
        let rhs_non_terminating = if let NumericValue::Rational(r) = &rhs.value {
            !is_terminating_decimal(*r.numer(), *r.denom())
        } else {
            false
        };

        // Compute ONCE
        let result_value = self.value * rhs.value;

        let apprx = if self_trans || rhs_trans {
            Some(ApproximationType::Transcendental)
        } else if self_rat_approx || rhs_rat_approx {
            if matches!(result_value, NumericValue::Rational(_)) {
                None
            } else {
                Some(ApproximationType::RationalApproximation)
            }
        } else if (self_rational || rhs_rational)
            && matches!(result_value, NumericValue::Decimal(_))
        {
            // Only set flag if a NON-TERMINATING rational graduated
            if self_non_terminating || rhs_non_terminating {
                Some(ApproximationType::RationalApproximation)
            } else {
                None // Terminating rational -> Decimal is exact
            }
        } else {
            None
        };

        Number {
            value: result_value,
            apprx,
        }
    }
}

impl Div for Number {
    type Output = Number;
    fn div(self, rhs: Number) -> Number {
        use crate::ApproximationType;
        use crate::core::is_terminating_decimal;

        // Check flags and types BEFORE moving
        let self_trans = self.is_transcendental();
        let rhs_trans = rhs.is_transcendental();
        let self_rat_approx = self.is_rational_approximation();
        let rhs_rat_approx = rhs.is_rational_approximation();
        let self_rational = matches!(self.value, NumericValue::Rational(_));
        let rhs_rational = matches!(rhs.value, NumericValue::Rational(_));

        // Check if either rational is non-terminating
        let self_non_terminating = if let NumericValue::Rational(r) = &self.value {
            !is_terminating_decimal(*r.numer(), *r.denom())
        } else {
            false
        };
        let rhs_non_terminating = if let NumericValue::Rational(r) = &rhs.value {
            !is_terminating_decimal(*r.numer(), *r.denom())
        } else {
            false
        };

        // Compute ONCE
        let result_value = self.value / rhs.value;

        let apprx = if self_trans || rhs_trans {
            Some(ApproximationType::Transcendental)
        } else if self_rat_approx || rhs_rat_approx {
            // Check if result went back to Rational (flag should clear)
            if matches!(result_value, NumericValue::Rational(_)) {
                None
            } else {
                Some(ApproximationType::RationalApproximation)
            }
        } else if (self_rational || rhs_rational)
            && matches!(result_value, NumericValue::Decimal(_))
        {
            // Only set flag if a NON-TERMINATING rational graduated
            if self_non_terminating || rhs_non_terminating {
                Some(ApproximationType::RationalApproximation)
            } else {
                None // Terminating rational -> Decimal is exact
            }
        } else {
            None
        };

        Number {
            value: result_value,
            apprx,
        }
    }
}

impl Rem for Number {
    type Output = Number;
    fn rem(self, rhs: Number) -> Number {
        use crate::ApproximationType;

        // Check flags BEFORE moving
        let self_trans = self.is_transcendental();
        let rhs_trans = rhs.is_transcendental();
        let self_rat_approx = self.is_rational_approximation();
        let rhs_rat_approx = rhs.is_rational_approximation();

        let apprx = if self_trans || rhs_trans {
            Some(ApproximationType::Transcendental)
        } else if self_rat_approx || rhs_rat_approx {
            Some(ApproximationType::RationalApproximation)
        } else {
            None
        };

        Number {
            value: self.value % rhs.value,
            apprx,
        }
    }
}

impl Neg for Number {
    type Output = Number;
    fn neg(self) -> Number {
        Number {
            value: -self.value,
            apprx: self.apprx,
        }
    }
}

forward_ref_binop!(impl Add, add for Number);
forward_ref_binop!(impl Sub, sub for Number);
forward_ref_binop!(impl Mul, mul for Number);
forward_ref_binop!(impl Div, div for Number);
forward_ref_binop!(impl Rem, rem for Number);
