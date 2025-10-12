use crate::{Number, NumericValue, forward_ref_binop};
use bigdecimal::BigDecimal;
use num_traits::{CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, Signed, Zero};
use rust_decimal::Decimal;
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

/// Fast conversion from rust_decimal::Decimal to bigdecimal::BigDecimal
/// Avoids slow string parsing by using mantissa and scale directly
#[inline(always)]
pub(crate) fn decimal_to_bigdecimal(d: Decimal) -> BigDecimal {
    let mantissa = d.mantissa();
    let scale = d.scale() as i64;
    BigDecimal::new(mantissa.into(), scale)
}
impl Add for NumericValue {
    type Output = (NumericValue, bool);
    fn add(self, rhs: NumericValue) -> (NumericValue, bool) {
        match (self, rhs) {
            // Rational + Rational: stays Rational, or graduates to Decimal/BigDecimal if denominator overflows
            (NumericValue::Rational(a, a_term), NumericValue::Rational(b, b_term)) => {
                // Fast path: integer addition (denom=1, no overflow risk for small integers)
                if *a.denom() == 1 && *b.denom() == 1 {
                    let a_num = *a.numer();
                    let b_num = *b.numer();
                    // Range where sum guaranteed to fit in i64
                    if a_num.abs() < 1_000_000_000 && b_num.abs() < 1_000_000_000 {
                        use num_rational::Ratio;
                        return (
                            NumericValue::Rational(Ratio::from_integer(a_num + b_num), true),
                            false,
                        );
                    }
                }

                // Try rational addition
                if let Some(result) = a.checked_add(&b) {
                    let is_term = a_term && b_term; // Cached!
                    (NumericValue::Rational(result, is_term), false)
                } else {
                    // Use cached terminating flags - no recomputation needed!
                    if !a_term || !b_term {
                        // Non-terminating: promote directly to BigDecimal
                        use bigdecimal::{BigDecimal, num_bigint::BigInt};
                        let a_numer_bd = BigDecimal::from(BigInt::from(*a.numer()));
                        let a_denom_bd = BigDecimal::from(BigInt::from(*a.denom()));
                        let a_bd = a_numer_bd / a_denom_bd;
                        let b_numer_bd = BigDecimal::from(BigInt::from(*b.numer()));
                        let b_denom_bd = BigDecimal::from(BigInt::from(*b.denom()));
                        let b_bd = b_numer_bd / b_denom_bd;
                        (NumericValue::BigDecimal(a_bd + b_bd), true) // Non-terminating overflow
                    } else {
                        // Terminating: try Decimal first
                        let a_dec = Decimal::from(*a.numer()) / Decimal::from(*a.denom());
                        let b_dec = Decimal::from(*b.numer()) / Decimal::from(*b.denom());
                        match a_dec.checked_add(b_dec) {
                            Some(result) => (NumericValue::Decimal(result), false),
                            None => {
                                let a_bd = decimal_to_bigdecimal(a_dec);
                                let b_bd = decimal_to_bigdecimal(b_dec);
                                (NumericValue::BigDecimal(a_bd + b_bd), false)
                            }
                        }
                    }
                }
            }

            // Rational + Decimal: graduate Rational to Decimal or BigDecimal
            (NumericValue::Rational(a, a_term), NumericValue::Decimal(b))
            | (NumericValue::Decimal(b), NumericValue::Rational(a, a_term)) => {
                // Use cached terminating flag - no recomputation needed!
                if !a_term {
                    // Non-terminating: promote directly to BigDecimal
                    use bigdecimal::{BigDecimal, num_bigint::BigInt};
                    let numer_bd = BigDecimal::from(BigInt::from(*a.numer()));
                    let denom_bd = BigDecimal::from(BigInt::from(*a.denom()));
                    let a_bd = numer_bd / denom_bd;
                    let b_bd = decimal_to_bigdecimal(b);
                    (NumericValue::BigDecimal(a_bd + b_bd), true) // Non-terminating overflow
                } else {
                    // Terminating: try Decimal first
                    let a_dec = Decimal::from(*a.numer()) / Decimal::from(*a.denom());
                    match a_dec.checked_add(b) {
                        Some(result) => (NumericValue::from_decimal(result), false),
                        None => {
                            // Graduate to BigDecimal
                            let a_bd = decimal_to_bigdecimal(a_dec);
                            let b_bd = decimal_to_bigdecimal(b);
                            (NumericValue::BigDecimal(a_bd + b_bd), false)
                        }
                    }
                }
            }

            // Rational + BigDecimal: graduate Rational to BigDecimal
            (NumericValue::Rational(a, _), NumericValue::BigDecimal(b))
            | (NumericValue::BigDecimal(b), NumericValue::Rational(a, _)) => {
                use bigdecimal::{BigDecimal, num_bigint::BigInt};
                let numer_bd = BigDecimal::from(BigInt::from(*a.numer()));
                let denom_bd = BigDecimal::from(BigInt::from(*a.denom()));
                let a_bd = numer_bd / denom_bd;
                (NumericValue::BigDecimal(a_bd + b), false)
            }

            // Decimal + Decimal
            (NumericValue::Decimal(a), NumericValue::Decimal(b)) => match a.checked_add(b) {
                Some(result) => (NumericValue::Decimal(result), false),
                None => {
                    let a_bd = decimal_to_bigdecimal(a);
                    let b_bd = decimal_to_bigdecimal(b);
                    (NumericValue::BigDecimal(a_bd + b_bd), false)
                }
            },

            // Special cases with NegativeZero
            (NumericValue::Rational(a, a_term), NumericValue::NegativeZero)
            | (NumericValue::NegativeZero, NumericValue::Rational(a, a_term)) => {
                (NumericValue::Rational(a, a_term), false)
            }
            (NumericValue::Decimal(a), NumericValue::NegativeZero)
            | (NumericValue::NegativeZero, NumericValue::Decimal(a)) => {
                (NumericValue::Decimal(a), false)
            }
            (NumericValue::BigDecimal(a), NumericValue::NegativeZero)
            | (NumericValue::NegativeZero, NumericValue::BigDecimal(a)) => {
                (NumericValue::BigDecimal(a), false)
            }
            (NumericValue::NegativeZero, NumericValue::NegativeZero) => {
                (NumericValue::NegativeZero, false)
            }

            // BigDecimal operations
            (NumericValue::BigDecimal(a), NumericValue::BigDecimal(b)) => {
                (NumericValue::BigDecimal(a + b), false)
            }
            (NumericValue::BigDecimal(a), NumericValue::Decimal(b))
            | (NumericValue::Decimal(b), NumericValue::BigDecimal(a)) => {
                let b_bd = decimal_to_bigdecimal(b);
                (NumericValue::BigDecimal(a + b_bd), false)
            }

            // NaN and Infinity handling
            (NumericValue::NaN, _) | (_, NumericValue::NaN) => (NumericValue::NaN, false),
            (NumericValue::PositiveInfinity, NumericValue::NegativeInfinity)
            | (NumericValue::NegativeInfinity, NumericValue::PositiveInfinity) => {
                (NumericValue::NaN, false)
            }
            (NumericValue::PositiveInfinity, _) | (_, NumericValue::PositiveInfinity) => {
                (NumericValue::PositiveInfinity, false)
            }
            (NumericValue::NegativeInfinity, _) | (_, NumericValue::NegativeInfinity) => {
                (NumericValue::NegativeInfinity, false)
            }
        }
    }
}

impl Sub for NumericValue {
    type Output = (NumericValue, bool);
    fn sub(self, rhs: NumericValue) -> (NumericValue, bool) {
        match (self, rhs) {
            // Rational - Rational: stays Rational, or graduates to Decimal if denominator overflows
            (NumericValue::Rational(a, a_term), NumericValue::Rational(b, b_term)) => {
                if let Some(result) = a.checked_sub(&b) {
                    let is_term = a_term && b_term; // Cached!
                    (NumericValue::Rational(result, is_term), false)
                } else {
                    // Use cached terminating flags - no recomputation needed!
                    if !a_term || !b_term {
                        // Non-terminating: promote directly to BigDecimal
                        use bigdecimal::{BigDecimal, num_bigint::BigInt};
                        let a_numer_bd = BigDecimal::from(BigInt::from(*a.numer()));
                        let a_denom_bd = BigDecimal::from(BigInt::from(*a.denom()));
                        let a_bd = a_numer_bd / a_denom_bd;
                        let b_numer_bd = BigDecimal::from(BigInt::from(*b.numer()));
                        let b_denom_bd = BigDecimal::from(BigInt::from(*b.denom()));
                        let b_bd = b_numer_bd / b_denom_bd;
                        (NumericValue::BigDecimal(a_bd - b_bd), true) // Non-terminating overflow
                    } else {
                        // Terminating: try Decimal first
                        let a_dec = Decimal::from(*a.numer()) / Decimal::from(*a.denom());
                        let b_dec = Decimal::from(*b.numer()) / Decimal::from(*b.denom());
                        match a_dec.checked_sub(b_dec) {
                            Some(result) => (NumericValue::Decimal(result), false),
                            None => {
                                let a_bd = decimal_to_bigdecimal(a_dec);
                                let b_bd = decimal_to_bigdecimal(b_dec);
                                (NumericValue::BigDecimal(a_bd - b_bd), false)
                            }
                        }
                    }
                }
            }

            // Rational - Decimal: graduate Rational to Decimal
            (NumericValue::Rational(a, a_term), NumericValue::Decimal(b)) => {
                // Use cached terminating flag - no recomputation needed!
                if !a_term {
                    // Non-terminating: promote directly to BigDecimal
                    use bigdecimal::{BigDecimal, num_bigint::BigInt};
                    let numer_bd = BigDecimal::from(BigInt::from(*a.numer()));
                    let denom_bd = BigDecimal::from(BigInt::from(*a.denom()));
                    let a_bd = numer_bd / denom_bd;
                    let b_bd = decimal_to_bigdecimal(b);
                    (NumericValue::BigDecimal(a_bd - b_bd), true) // Non-terminating overflow
                } else {
                    // Terminating: try Decimal first
                    let a_dec = Decimal::from(*a.numer()) / Decimal::from(*a.denom());
                    match a_dec.checked_sub(b) {
                        Some(result) => (NumericValue::from_decimal(result), false),
                        None => {
                            // Graduate to BigDecimal
                            let a_bd = decimal_to_bigdecimal(a_dec);
                            let b_bd = decimal_to_bigdecimal(b);
                            (NumericValue::BigDecimal(a_bd - b_bd), false)
                        }
                    }
                }
            }
            // Decimal - Rational: graduate Rational to Decimal
            (NumericValue::Decimal(a), NumericValue::Rational(b, b_term)) => {
                // Use cached terminating flag - no recomputation needed!
                if !b_term {
                    // Non-terminating: promote directly to BigDecimal
                    use bigdecimal::{BigDecimal, num_bigint::BigInt};
                    let numer_bd = BigDecimal::from(BigInt::from(*b.numer()));
                    let denom_bd = BigDecimal::from(BigInt::from(*b.denom()));
                    let b_bd = numer_bd / denom_bd;
                    let a_bd = decimal_to_bigdecimal(a);
                    (NumericValue::BigDecimal(a_bd - b_bd), true) // Non-terminating overflow
                } else {
                    // Terminating: try Decimal first
                    let b_dec = Decimal::from(*b.numer()) / Decimal::from(*b.denom());
                    match a.checked_sub(b_dec) {
                        Some(result) => (NumericValue::from_decimal(result), false),
                        None => {
                            // Graduate to BigDecimal
                            let a_bd = decimal_to_bigdecimal(a);
                            let b_bd = decimal_to_bigdecimal(b_dec);
                            (NumericValue::BigDecimal(a_bd - b_bd), false)
                        }
                    }
                }
            }

            // Rational - BigDecimal: graduate Rational to BigDecimal
            (NumericValue::Rational(a, _), NumericValue::BigDecimal(b)) => {
                use bigdecimal::{BigDecimal, num_bigint::BigInt};
                let numer_bd = BigDecimal::from(BigInt::from(*a.numer()));
                let denom_bd = BigDecimal::from(BigInt::from(*a.denom()));
                let a_bd = numer_bd / denom_bd;
                (NumericValue::BigDecimal(a_bd - b), false)
            }
            (NumericValue::BigDecimal(a), NumericValue::Rational(b, _)) => {
                use bigdecimal::{BigDecimal, num_bigint::BigInt};
                let numer_bd = BigDecimal::from(BigInt::from(*b.numer()));
                let denom_bd = BigDecimal::from(BigInt::from(*b.denom()));
                let b_bd = numer_bd / denom_bd;
                (NumericValue::BigDecimal(a - b_bd), false)
            }

            // Decimal - Decimal
            (NumericValue::Decimal(a), NumericValue::Decimal(b)) => {
                // Try subtraction, graduate to BigDecimal on overflow
                match a.checked_sub(b) {
                    Some(result) => (NumericValue::Decimal(result), false),
                    None => {
                        // Overflow - graduate to BigDecimal
                        let a_bd = decimal_to_bigdecimal(a);
                        let b_bd = decimal_to_bigdecimal(b);
                        (NumericValue::BigDecimal(a_bd - b_bd), false)
                    }
                }
            }

            // Special cases with NegativeZero
            (NumericValue::Rational(a, a_term), NumericValue::NegativeZero) => {
                (NumericValue::Rational(a, a_term), false)
            }
            (NumericValue::NegativeZero, NumericValue::Rational(a, a_term)) => {
                (NumericValue::Rational(-a, a_term), false)
            }
            (NumericValue::Decimal(a), NumericValue::NegativeZero) => {
                (NumericValue::Decimal(a), false)
            } // x - (-0) = x
            (NumericValue::NegativeZero, NumericValue::Decimal(b)) => {
                (NumericValue::Decimal(-b), false)
            } // (-0) - x = -x
            (NumericValue::BigDecimal(a), NumericValue::NegativeZero) => {
                (NumericValue::BigDecimal(a), false)
            }
            (NumericValue::NegativeZero, NumericValue::BigDecimal(b)) => {
                (NumericValue::BigDecimal(-b), false)
            }
            (NumericValue::NegativeZero, NumericValue::NegativeZero) => (NumericValue::ZERO, false), // (-0) - (-0) = +0

            // BigDecimal operations
            (NumericValue::BigDecimal(a), NumericValue::BigDecimal(b)) => {
                (NumericValue::BigDecimal(a - b), false)
            }
            (NumericValue::BigDecimal(a), NumericValue::Decimal(b)) => {
                let b_bd = decimal_to_bigdecimal(b);
                (NumericValue::BigDecimal(a - b_bd), false)
            }
            (NumericValue::Decimal(a), NumericValue::BigDecimal(b)) => {
                let a_bd = decimal_to_bigdecimal(a);
                (NumericValue::BigDecimal(a_bd - b), false)
            }

            (NumericValue::NaN, _) | (_, NumericValue::NaN) => (NumericValue::NaN, false),
            (NumericValue::PositiveInfinity, NumericValue::PositiveInfinity)
            | (NumericValue::NegativeInfinity, NumericValue::NegativeInfinity) => {
                (NumericValue::NaN, false)
            } // ∞ - ∞ = NaN
            (NumericValue::PositiveInfinity, _) => (NumericValue::PositiveInfinity, false),
            (NumericValue::NegativeInfinity, _) => (NumericValue::NegativeInfinity, false),
            (_, NumericValue::PositiveInfinity) => (NumericValue::NegativeInfinity, false),
            (_, NumericValue::NegativeInfinity) => (NumericValue::PositiveInfinity, false),
        }
    }
}

impl Mul for NumericValue {
    type Output = (NumericValue, bool);
    fn mul(self, rhs: NumericValue) -> (NumericValue, bool) {
        match (self, rhs) {
            // Rational * Rational: stays Rational, or graduates to Decimal/BigDecimal if overflow
            (NumericValue::Rational(a, a_term), NumericValue::Rational(b, b_term)) => {
                // Fast path: integer multiplication (denom=1, no overflow risk for small integers)
                if *a.denom() == 1 && *b.denom() == 1 {
                    let a_num = *a.numer();
                    let b_num = *b.numer();
                    // Range where product guaranteed to fit in i64
                    if a_num.abs() < 100_000 && b_num.abs() < 100_000 {
                        use num_rational::Ratio;
                        return (
                            NumericValue::Rational(Ratio::from_integer(a_num * b_num), true),
                            false,
                        );
                    }
                }

                if let Some(result) = a.checked_mul(&b) {
                    let is_term = a_term && b_term; // Cached!
                    (NumericValue::Rational(result, is_term), false)
                } else {
                    // Use cached terminating flags - no recomputation needed!
                    if !a_term || !b_term {
                        // Non-terminating: use BigDecimal to preserve precision for recovery
                        use bigdecimal::{BigDecimal, num_bigint::BigInt};
                        let a_bd = BigDecimal::from(BigInt::from(*a.numer()))
                            / BigDecimal::from(BigInt::from(*a.denom()));
                        let b_bd = BigDecimal::from(BigInt::from(*b.numer()))
                            / BigDecimal::from(BigInt::from(*b.denom()));
                        (NumericValue::BigDecimal(a_bd * b_bd), true) // Non-terminating overflow
                    } else {
                        // Terminating: try Decimal first (faster), then BigDecimal if needed
                        let a_dec = Decimal::from(*a.numer()) / Decimal::from(*a.denom());
                        let b_dec = Decimal::from(*b.numer()) / Decimal::from(*b.denom());
                        match a_dec.checked_mul(b_dec) {
                            Some(result) => (NumericValue::Decimal(result), false),
                            None => {
                                // Graduate to BigDecimal - use fast conversion
                                let a_bd = decimal_to_bigdecimal(a_dec);
                                let b_bd = decimal_to_bigdecimal(b_dec);
                                (NumericValue::BigDecimal(a_bd * b_bd), false)
                            }
                        }
                    }
                }
            }

            // Rational * Decimal: graduate Rational to Decimal
            (NumericValue::Rational(a, a_term), NumericValue::Decimal(b))
            | (NumericValue::Decimal(b), NumericValue::Rational(a, a_term)) => {
                // Use cached terminating flag - no recomputation needed!
                if !a_term {
                    // Non-terminating: promote directly to BigDecimal
                    use bigdecimal::{BigDecimal, num_bigint::BigInt};
                    let numer_bd = BigDecimal::from(BigInt::from(*a.numer()));
                    let denom_bd = BigDecimal::from(BigInt::from(*a.denom()));
                    let a_bd = numer_bd / denom_bd;
                    let b_bd = decimal_to_bigdecimal(b);
                    (NumericValue::BigDecimal(a_bd * b_bd), true) // Non-terminating overflow
                } else {
                    // Terminating: try Decimal first
                    let a_dec = Decimal::from(*a.numer()) / Decimal::from(*a.denom());
                    match a_dec.checked_mul(b) {
                        Some(result) => (NumericValue::from_decimal(result), false),
                        None => {
                            // Graduate to BigDecimal
                            let a_bd = decimal_to_bigdecimal(a_dec);
                            let b_bd = decimal_to_bigdecimal(b);
                            (NumericValue::BigDecimal(a_bd * b_bd), false)
                        }
                    }
                }
            }

            // Rational * BigDecimal: graduate Rational to BigDecimal
            (NumericValue::Rational(a, _), NumericValue::BigDecimal(b))
            | (NumericValue::BigDecimal(b), NumericValue::Rational(a, _)) => {
                use bigdecimal::{BigDecimal, num_bigint::BigInt};
                let numer_bd = BigDecimal::from(BigInt::from(*a.numer()));
                let denom_bd = BigDecimal::from(BigInt::from(*a.denom()));
                let a_bd = numer_bd / denom_bd;
                (NumericValue::BigDecimal(a_bd * b), false)
            }

            // Decimal * Decimal
            (NumericValue::Decimal(a), NumericValue::Decimal(b)) => {
                // Try multiplication, graduate to BigDecimal on overflow
                match a.checked_mul(b) {
                    Some(result) => (NumericValue::Decimal(result), false),
                    None => {
                        // Overflow - graduate to BigDecimal
                        let a_bd = decimal_to_bigdecimal(a);
                        let b_bd = decimal_to_bigdecimal(b);
                        (NumericValue::BigDecimal(a_bd * b_bd), false)
                    }
                }
            }

            // BigDecimal operations
            (NumericValue::BigDecimal(a), NumericValue::BigDecimal(b)) => {
                (NumericValue::BigDecimal(a * b), false)
            }
            (NumericValue::BigDecimal(a), NumericValue::Decimal(b))
            | (NumericValue::Decimal(b), NumericValue::BigDecimal(a)) => {
                let b_bd = decimal_to_bigdecimal(b);
                (NumericValue::BigDecimal(a * b_bd), false)
            }

            // Special cases with NegativeZero and Rational
            (NumericValue::Rational(a, _), NumericValue::NegativeZero)
            | (NumericValue::NegativeZero, NumericValue::Rational(a, _)) => {
                if a.is_zero() {
                    (NumericValue::NegativeZero, false) // 0 * (-0) = -0
                } else if *a.numer() > 0 {
                    (NumericValue::NegativeZero, false) // positive * (-0) = -0
                } else {
                    (NumericValue::ZERO, false) // negative * (-0) = +0
                }
            }
            (NumericValue::Decimal(a), NumericValue::NegativeZero)
            | (NumericValue::NegativeZero, NumericValue::Decimal(a)) => {
                if a.is_zero() {
                    (NumericValue::NegativeZero, false) // 0 * (-0) = -0 in JS
                } else if a > Decimal::ZERO {
                    (NumericValue::NegativeZero, false) // positive * (-0) = -0
                } else {
                    (NumericValue::ZERO, false) // negative * (-0) = +0
                }
            }
            (NumericValue::BigDecimal(a), NumericValue::NegativeZero)
            | (NumericValue::NegativeZero, NumericValue::BigDecimal(a)) => {
                if a.is_zero() {
                    (NumericValue::NegativeZero, false)
                } else if a.is_positive() {
                    (NumericValue::NegativeZero, false)
                } else {
                    (NumericValue::ZERO, false)
                }
            }
            (NumericValue::NegativeZero, NumericValue::NegativeZero) => (NumericValue::ZERO, false), // (-0) * (-0) = +0

            (NumericValue::NaN, _) | (_, NumericValue::NaN) => (NumericValue::NaN, false),

            // 0 * ∞ = NaN in JavaScript (Rational case)
            (NumericValue::Rational(a, _), NumericValue::PositiveInfinity)
            | (NumericValue::Rational(a, _), NumericValue::NegativeInfinity)
            | (NumericValue::PositiveInfinity, NumericValue::Rational(a, _))
            | (NumericValue::NegativeInfinity, NumericValue::Rational(a, _))
                if a.is_zero() =>
            {
                (NumericValue::NaN, false)
            }
            // 0 * ∞ = NaN in JavaScript (Decimal case)
            (NumericValue::Decimal(a), NumericValue::PositiveInfinity)
            | (NumericValue::Decimal(a), NumericValue::NegativeInfinity)
            | (NumericValue::PositiveInfinity, NumericValue::Decimal(a))
            | (NumericValue::NegativeInfinity, NumericValue::Decimal(a))
                if a.is_zero() =>
            {
                (NumericValue::NaN, false)
            }
            // 0 * ∞ = NaN in JavaScript (BigDecimal case)
            (NumericValue::BigDecimal(a), NumericValue::PositiveInfinity)
            | (NumericValue::BigDecimal(a), NumericValue::NegativeInfinity)
            | (NumericValue::PositiveInfinity, NumericValue::BigDecimal(a))
            | (NumericValue::NegativeInfinity, NumericValue::BigDecimal(a))
                if a.is_zero() =>
            {
                (NumericValue::NaN, false)
            }
            (NumericValue::PositiveInfinity, NumericValue::NegativeZero)
            | (NumericValue::NegativeInfinity, NumericValue::NegativeZero)
            | (NumericValue::NegativeZero, NumericValue::PositiveInfinity)
            | (NumericValue::NegativeZero, NumericValue::NegativeInfinity) => {
                (NumericValue::NaN, false)
            }

            // Handle infinity multiplication
            (NumericValue::PositiveInfinity, NumericValue::PositiveInfinity)
            | (NumericValue::NegativeInfinity, NumericValue::NegativeInfinity) => {
                (NumericValue::PositiveInfinity, false)
            }
            (NumericValue::PositiveInfinity, NumericValue::NegativeInfinity)
            | (NumericValue::NegativeInfinity, NumericValue::PositiveInfinity) => {
                (NumericValue::NegativeInfinity, false)
            }

            // Infinity * finite Rational
            (NumericValue::PositiveInfinity, NumericValue::Rational(a, _))
            | (NumericValue::Rational(a, _), NumericValue::PositiveInfinity) => {
                if *a.numer() > 0 {
                    (NumericValue::PositiveInfinity, false)
                } else {
                    (NumericValue::NegativeInfinity, false)
                }
            }
            (NumericValue::NegativeInfinity, NumericValue::Rational(a, _))
            | (NumericValue::Rational(a, _), NumericValue::NegativeInfinity) => {
                if *a.numer() > 0 {
                    (NumericValue::NegativeInfinity, false)
                } else {
                    (NumericValue::PositiveInfinity, false)
                }
            }
            // Infinity * finite Decimal
            (NumericValue::PositiveInfinity, NumericValue::Decimal(b))
            | (NumericValue::Decimal(b), NumericValue::PositiveInfinity) => {
                if b > Decimal::ZERO {
                    (NumericValue::PositiveInfinity, false)
                } else {
                    (NumericValue::NegativeInfinity, false)
                }
            }
            (NumericValue::NegativeInfinity, NumericValue::Decimal(b))
            | (NumericValue::Decimal(b), NumericValue::NegativeInfinity) => {
                if b > Decimal::ZERO {
                    (NumericValue::NegativeInfinity, false)
                } else {
                    (NumericValue::PositiveInfinity, false)
                }
            }
            // Infinity * finite BigDecimal
            (NumericValue::PositiveInfinity, NumericValue::BigDecimal(b))
            | (NumericValue::BigDecimal(b), NumericValue::PositiveInfinity) => {
                if b.is_positive() {
                    (NumericValue::PositiveInfinity, false)
                } else {
                    (NumericValue::NegativeInfinity, false)
                }
            }
            (NumericValue::NegativeInfinity, NumericValue::BigDecimal(b))
            | (NumericValue::BigDecimal(b), NumericValue::NegativeInfinity) => {
                if b.is_positive() {
                    (NumericValue::NegativeInfinity, false)
                } else {
                    (NumericValue::PositiveInfinity, false)
                }
            }
        }
    }
}

impl Div for NumericValue {
    type Output = (NumericValue, bool);
    fn div(self, rhs: NumericValue) -> (NumericValue, bool) {
        use num_rational::Ratio;

        match (self, rhs) {
            // Rational / Rational: stays Rational (invert and multiply), or graduates to Decimal if overflow
            (NumericValue::Rational(a, a_term), NumericValue::Rational(b, b_term)) => {
                if b.is_zero() {
                    if a.is_zero() {
                        (NumericValue::NaN, false) // 0/0 = NaN
                    } else if *a.numer() > 0 {
                        (NumericValue::PositiveInfinity, false) // positive/0 = +∞
                    } else {
                        (NumericValue::NegativeInfinity, false) // negative/0 = -∞
                    }
                } else if let Some(result) = a.checked_div(&b) {
                    let is_term = a_term && b_term; // Cached!
                    (NumericValue::Rational(result, is_term), false)
                } else {
                    // Use cached terminating flags - no recomputation needed!
                    if !a_term || !b_term {
                        // Non-terminating: use BigDecimal to preserve precision for recovery
                        use bigdecimal::{BigDecimal, num_bigint::BigInt};
                        let a_bd = BigDecimal::from(BigInt::from(*a.numer()))
                            / BigDecimal::from(BigInt::from(*a.denom()));
                        let b_bd = BigDecimal::from(BigInt::from(*b.numer()))
                            / BigDecimal::from(BigInt::from(*b.denom()));
                        (NumericValue::BigDecimal(a_bd / b_bd), true)
                    } else {
                        // Terminating: try Decimal first (faster), then BigDecimal if needed
                        let a_dec = Decimal::from(*a.numer()) / Decimal::from(*a.denom());
                        let b_dec = Decimal::from(*b.numer()) / Decimal::from(*b.denom());
                        match a_dec.checked_div(b_dec) {
                            Some(result) => (NumericValue::Decimal(result), false),
                            None => {
                                // Graduate to BigDecimal - use fast conversion
                                let a_bd = decimal_to_bigdecimal(a_dec);
                                let b_bd = decimal_to_bigdecimal(b_dec);
                                (NumericValue::BigDecimal(a_bd / b_bd), false)
                            }
                        }
                    }
                }
            }

            // Rational / Decimal: graduate Rational to Decimal
            (NumericValue::Rational(a, a_term), NumericValue::Decimal(b)) => {
                if b.is_zero() {
                    if a.is_zero() {
                        (NumericValue::NaN, false)
                    } else if *a.numer() > 0 {
                        (NumericValue::PositiveInfinity, false)
                    } else {
                        (NumericValue::NegativeInfinity, false)
                    }
                } else {
                    // Use cached terminating flag - no recomputation needed!
                    if !a_term {
                        // Non-terminating: promote directly to BigDecimal
                        use bigdecimal::{BigDecimal, num_bigint::BigInt};
                        let numer_bd = BigDecimal::from(BigInt::from(*a.numer()));
                        let denom_bd = BigDecimal::from(BigInt::from(*a.denom()));
                        let a_bd = numer_bd / denom_bd;
                        let b_bd = decimal_to_bigdecimal(b);
                        (NumericValue::BigDecimal(a_bd / b_bd), true) // Non-terminating overflow
                    } else {
                        // Terminating: try Decimal first
                        let a_dec = Decimal::from(*a.numer()) / Decimal::from(*a.denom());
                        match a_dec.checked_div(b) {
                            Some(result) => (NumericValue::from_decimal(result), false),
                            None => {
                                // Graduate to BigDecimal
                                let a_bd = decimal_to_bigdecimal(a_dec);
                                let b_bd = decimal_to_bigdecimal(b);
                                (NumericValue::BigDecimal(a_bd / b_bd), false)
                            }
                        }
                    }
                }
            }
            // Decimal / Rational: graduate Rational to Decimal
            (NumericValue::Decimal(a), NumericValue::Rational(b, b_term)) => {
                if b.is_zero() {
                    if a.is_zero() {
                        (NumericValue::NaN, false)
                    } else if a > Decimal::ZERO {
                        (NumericValue::PositiveInfinity, false)
                    } else {
                        (NumericValue::NegativeInfinity, false)
                    }
                } else {
                    // Use cached terminating flag - no recomputation needed!
                    if !b_term {
                        // Non-terminating: promote directly to BigDecimal
                        use bigdecimal::{BigDecimal, num_bigint::BigInt};
                        let numer_bd = BigDecimal::from(BigInt::from(*b.numer()));
                        let denom_bd = BigDecimal::from(BigInt::from(*b.denom()));
                        let b_bd = numer_bd / denom_bd;
                        let a_bd = decimal_to_bigdecimal(a);
                        (NumericValue::BigDecimal(a_bd / b_bd), true) // Non-terminating overflow
                    } else {
                        // Terminating: try Decimal first
                        let b_dec = Decimal::from(*b.numer()) / Decimal::from(*b.denom());
                        match a.checked_div(b_dec) {
                            Some(result) => (NumericValue::from_decimal(result), false),
                            None => {
                                // Graduate to BigDecimal
                                let a_bd = decimal_to_bigdecimal(a);
                                let b_bd = decimal_to_bigdecimal(b_dec);
                                (NumericValue::BigDecimal(a_bd / b_bd), false)
                            }
                        }
                    }
                }
            }

            // Rational / BigDecimal: graduate Rational to BigDecimal
            (NumericValue::Rational(a, _), NumericValue::BigDecimal(b)) => {
                if b.is_zero() {
                    if a.is_zero() {
                        (NumericValue::NaN, false)
                    } else if *a.numer() > 0 {
                        (NumericValue::PositiveInfinity, false)
                    } else {
                        (NumericValue::NegativeInfinity, false)
                    }
                } else {
                    use bigdecimal::{BigDecimal, num_bigint::BigInt};
                    let numer_bd = BigDecimal::from(BigInt::from(*a.numer()));
                    let denom_bd = BigDecimal::from(BigInt::from(*a.denom()));
                    let a_bd = numer_bd / denom_bd;
                    (NumericValue::BigDecimal(a_bd / b), false)
                }
            }
            // BigDecimal / Rational: graduate Rational to BigDecimal
            (NumericValue::BigDecimal(a), NumericValue::Rational(b, _)) => {
                if b.is_zero() {
                    if a.is_zero() {
                        (NumericValue::NaN, false)
                    } else if a.is_positive() {
                        (NumericValue::PositiveInfinity, false)
                    } else {
                        (NumericValue::NegativeInfinity, false)
                    }
                } else {
                    use bigdecimal::{BigDecimal, num_bigint::BigInt};
                    let numer_bd = BigDecimal::from(BigInt::from(*b.numer()));
                    let denom_bd = BigDecimal::from(BigInt::from(*b.denom()));
                    let b_bd = numer_bd / denom_bd;
                    (NumericValue::BigDecimal(a / b_bd), false)
                }
            }

            // Special cases with NegativeZero and Rational
            (NumericValue::Rational(a, _), NumericValue::NegativeZero) => {
                if a.is_zero() {
                    (NumericValue::NaN, false) // 0/(-0) = NaN
                } else if *a.numer() > 0 {
                    (NumericValue::NegativeInfinity, false) // positive/(-0) = -∞
                } else {
                    (NumericValue::PositiveInfinity, false) // negative/(-0) = +∞
                }
            }
            (NumericValue::NegativeZero, NumericValue::Rational(b, _)) => {
                if b.is_zero() {
                    (NumericValue::NaN, false) // (-0)/0 = NaN
                } else if *b.numer() > 0 {
                    (NumericValue::NegativeZero, false) // (-0)/positive = -0
                } else {
                    (NumericValue::ZERO, false) // (-0)/negative = +0
                }
            }
            // Special cases with NegativeZero and BigDecimal
            (NumericValue::BigDecimal(a), NumericValue::NegativeZero) => {
                if a.is_zero() {
                    (NumericValue::NaN, false) // 0/(-0) = NaN
                } else if a.is_positive() {
                    (NumericValue::NegativeInfinity, false) // positive/(-0) = -∞
                } else {
                    (NumericValue::PositiveInfinity, false) // negative/(-0) = +∞
                }
            }
            (NumericValue::NegativeZero, NumericValue::BigDecimal(b)) => {
                if b.is_zero() {
                    (NumericValue::NaN, false) // (-0)/0 = NaN
                } else if b.is_positive() {
                    (NumericValue::NegativeZero, false) // (-0)/positive = -0
                } else {
                    (NumericValue::ZERO, false) // (-0)/negative = +0
                }
            }

            // Decimal / Decimal - optimized with direct rational construction
            (NumericValue::Decimal(a), NumericValue::Decimal(b)) => {
                if b.is_zero() {
                    if a.is_zero() {
                        (NumericValue::NaN, false) // 0/0 = NaN
                    } else if a > Decimal::ZERO {
                        (NumericValue::PositiveInfinity, false) // positive/0 = +∞
                    } else {
                        (NumericValue::NegativeInfinity, false) // negative/0 = -∞
                    }
                } else {
                    // Extract mantissas and scales for direct rational construction
                    let a_mantissa = a.mantissa();
                    let a_scale = a.scale();
                    let b_mantissa = b.mantissa();
                    let b_scale = b.scale();

                    // Try direct rational construction (faster than Decimal division + verification)
                    if let (Ok(a_i64), Ok(b_i64)) = (
                        a_mantissa.try_into() as Result<i64, _>,
                        b_mantissa.try_into() as Result<i64, _>,
                    ) {
                        use crate::core::is_terminating_decimal;
                        use num_rational::Ratio;

                        // Construct rational: (a_mantissa/10^a_scale) / (b_mantissa/10^b_scale)
                        //                  = (a_mantissa × 10^b_scale) / (b_mantissa × 10^a_scale)
                        let result_rat = if a_scale >= b_scale {
                            let scale_diff = a_scale - b_scale;
                            if let Some(factor) = 10i64.checked_pow(scale_diff) {
                                Some(Ratio::new(a_i64, b_i64 * factor))
                            } else {
                                None
                            }
                        } else {
                            let scale_diff = b_scale - a_scale;
                            if let Some(factor) = 10i64.checked_pow(scale_diff) {
                                Some(Ratio::new(a_i64 * factor, b_i64))
                            } else {
                                None
                            }
                        };

                        if let Some(rat) = result_rat {
                            // Check if it's terminating - if so, convert to Decimal
                            let is_term = is_terminating_decimal(*rat.numer(), *rat.denom());
                            if is_term {
                                // Terminating - compute exact Decimal representation
                                let result_dec =
                                    Decimal::from(*rat.numer()) / Decimal::from(*rat.denom());
                                (NumericValue::Decimal(result_dec), false)
                            } else {
                                // Non-terminating - keep as Rational for exact representation
                                (NumericValue::Rational(rat, false), false) // Use cached is_term value
                            }
                        } else {
                            // Scale overflow - fallback to BigDecimal
                            let a_bd = decimal_to_bigdecimal(a);
                            let b_bd = decimal_to_bigdecimal(b);
                            (NumericValue::BigDecimal(a_bd / b_bd), false)
                        }
                    } else {
                        // Mantissa doesn't fit in i64 - use BigDecimal
                        let a_bd = decimal_to_bigdecimal(a);
                        let b_bd = decimal_to_bigdecimal(b);
                        (NumericValue::BigDecimal(a_bd / b_bd), false)
                    }
                }
            }
            // BigDecimal division
            (NumericValue::BigDecimal(a), NumericValue::BigDecimal(b)) => {
                if b.is_zero() {
                    if a.is_zero() {
                        (NumericValue::NaN, false)
                    } else if a.is_positive() {
                        (NumericValue::PositiveInfinity, false)
                    } else {
                        (NumericValue::NegativeInfinity, false)
                    }
                } else {
                    (NumericValue::BigDecimal(a / b), false)
                }
            }
            // Mixed BigDecimal/Decimal division
            (NumericValue::BigDecimal(a), NumericValue::Decimal(b)) => {
                if b.is_zero() {
                    if a.is_zero() {
                        (NumericValue::NaN, false)
                    } else if a.is_positive() {
                        (NumericValue::PositiveInfinity, false)
                    } else {
                        (NumericValue::NegativeInfinity, false)
                    }
                } else {
                    let b_bd = decimal_to_bigdecimal(b);
                    (NumericValue::BigDecimal(a / b_bd), false)
                }
            }
            (NumericValue::Decimal(a), NumericValue::BigDecimal(b)) => {
                if b.is_zero() {
                    if a.is_zero() {
                        (NumericValue::NaN, false)
                    } else if a > Decimal::ZERO {
                        (NumericValue::PositiveInfinity, false)
                    } else {
                        (NumericValue::NegativeInfinity, false)
                    }
                } else {
                    let a_bd = decimal_to_bigdecimal(a);
                    (NumericValue::BigDecimal(a_bd / b), false)
                }
            }
            (NumericValue::Decimal(a), NumericValue::NegativeZero) => {
                if a.is_zero() {
                    (NumericValue::NaN, false) // 0/(-0) = NaN
                } else if a > Decimal::ZERO {
                    (NumericValue::NegativeInfinity, false) // positive/(-0) = -∞
                } else {
                    (NumericValue::PositiveInfinity, false) // negative/(-0) = +∞
                }
            }
            (NumericValue::NegativeZero, NumericValue::Decimal(b)) => {
                if b.is_zero() {
                    (NumericValue::NaN, false) // (-0)/0 = NaN
                } else if b > Decimal::ZERO {
                    (NumericValue::NegativeZero, false) // (-0)/positive = -0
                } else {
                    (NumericValue::ZERO, false) // (-0)/negative = +0
                }
            }
            (NumericValue::NegativeZero, NumericValue::NegativeZero) => (NumericValue::NaN, false), // (-0)/(-0) = NaN
            (NumericValue::NaN, _) | (_, NumericValue::NaN) => (NumericValue::NaN, false),
            // ∞ / ∞ = NaN, 0 / ∞ = 0
            (NumericValue::PositiveInfinity, NumericValue::PositiveInfinity)
            | (NumericValue::PositiveInfinity, NumericValue::NegativeInfinity)
            | (NumericValue::NegativeInfinity, NumericValue::PositiveInfinity)
            | (NumericValue::NegativeInfinity, NumericValue::NegativeInfinity) => {
                (NumericValue::NaN, false)
            }
            // finite / ∞ = 0 (with appropriate sign)
            (NumericValue::Rational(_, _), NumericValue::PositiveInfinity)
            | (NumericValue::Rational(_, _), NumericValue::NegativeInfinity) => {
                (NumericValue::Rational(Ratio::from_integer(0), true), false)
            }
            (NumericValue::Decimal(_), NumericValue::PositiveInfinity)
            | (NumericValue::Decimal(_), NumericValue::NegativeInfinity) => {
                (NumericValue::Decimal(Decimal::ZERO), false)
            }
            (NumericValue::BigDecimal(_), NumericValue::PositiveInfinity)
            | (NumericValue::BigDecimal(_), NumericValue::NegativeInfinity) => {
                use bigdecimal::BigDecimal;
                (NumericValue::BigDecimal(BigDecimal::from(0)), false)
            }
            (NumericValue::NegativeZero, NumericValue::PositiveInfinity)
            | (NumericValue::NegativeZero, NumericValue::NegativeInfinity) => {
                (NumericValue::NegativeZero, false)
            }

            // ∞ / finite Rational
            (NumericValue::PositiveInfinity, NumericValue::Rational(b, _)) => {
                if *b.numer() > 0 {
                    (NumericValue::PositiveInfinity, false)
                } else {
                    (NumericValue::NegativeInfinity, false)
                }
            }
            (NumericValue::NegativeInfinity, NumericValue::Rational(b, _)) => {
                if *b.numer() > 0 {
                    (NumericValue::NegativeInfinity, false)
                } else {
                    (NumericValue::PositiveInfinity, false)
                }
            }
            // ∞ / finite Decimal
            (NumericValue::PositiveInfinity, NumericValue::Decimal(b)) => {
                if b > Decimal::ZERO {
                    (NumericValue::PositiveInfinity, false)
                } else {
                    (NumericValue::NegativeInfinity, false)
                }
            }
            (NumericValue::NegativeInfinity, NumericValue::Decimal(b)) => {
                if b > Decimal::ZERO {
                    (NumericValue::NegativeInfinity, false)
                } else {
                    (NumericValue::PositiveInfinity, false)
                }
            }
            // ∞ / finite BigDecimal
            (NumericValue::PositiveInfinity, NumericValue::BigDecimal(b)) => {
                if b.is_positive() {
                    (NumericValue::PositiveInfinity, false)
                } else {
                    (NumericValue::NegativeInfinity, false)
                }
            }
            (NumericValue::NegativeInfinity, NumericValue::BigDecimal(b)) => {
                if b.is_positive() {
                    (NumericValue::NegativeInfinity, false)
                } else {
                    (NumericValue::PositiveInfinity, false)
                }
            }

            (NumericValue::PositiveInfinity, NumericValue::NegativeZero) => {
                (NumericValue::NegativeInfinity, false)
            }
            (NumericValue::NegativeInfinity, NumericValue::NegativeZero) => {
                (NumericValue::PositiveInfinity, false)
            }
        }
    }
}

impl Rem for NumericValue {
    type Output = NumericValue;
    fn rem(self, rhs: NumericValue) -> NumericValue {
        match (self, rhs) {
            // Rational % Rational: convert to Decimal for operation
            (NumericValue::Rational(a, _), NumericValue::Rational(b, _)) => {
                if b.is_zero() {
                    NumericValue::NaN
                } else {
                    let a_dec = Decimal::from(*a.numer()) / Decimal::from(*a.denom());
                    let b_dec = Decimal::from(*b.numer()) / Decimal::from(*b.denom());
                    NumericValue::Decimal(a_dec % b_dec)
                }
            }
            (NumericValue::Rational(a, _), NumericValue::Decimal(b)) => {
                if b.is_zero() {
                    NumericValue::NaN
                } else {
                    let a_dec = Decimal::from(*a.numer()) / Decimal::from(*a.denom());
                    NumericValue::Decimal(a_dec % b)
                }
            }
            (NumericValue::Decimal(a), NumericValue::Rational(b, _)) => {
                if b.is_zero() {
                    NumericValue::NaN
                } else {
                    let b_dec = Decimal::from(*b.numer()) / Decimal::from(*b.denom());
                    NumericValue::Decimal(a % b_dec)
                }
            }
            (NumericValue::Rational(a, _), NumericValue::BigDecimal(b)) => {
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
            (NumericValue::BigDecimal(a), NumericValue::Rational(b, _)) => {
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
            (NumericValue::Rational(_a, _), NumericValue::NegativeZero) => NumericValue::NaN,
            (NumericValue::NegativeZero, NumericValue::Rational(b, _)) => {
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
                    let b_bd = decimal_to_bigdecimal(b);
                    NumericValue::BigDecimal(a % b_bd)
                }
            }
            (NumericValue::Decimal(a), NumericValue::BigDecimal(b)) => {
                if b.is_zero() {
                    NumericValue::NaN
                } else {
                    let a_bd = decimal_to_bigdecimal(a);
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
            (NumericValue::Rational(a, a_term), NumericValue::PositiveInfinity)
            | (NumericValue::Rational(a, a_term), NumericValue::NegativeInfinity) => {
                NumericValue::Rational(a, a_term)
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
            NumericValue::Rational(r, r_term) => NumericValue::Rational(-r, r_term),
            NumericValue::BigDecimal(bd) => NumericValue::BigDecimal(-bd),
        }
    }
}

// Generate all reference variants for arithmetic operators
// forward_ref_binop!(impl Add, add for NumericValue);
// forward_ref_binop!(impl Sub, sub for NumericValue);
// forward_ref_binop!(impl Mul, mul for NumericValue);
// forward_ref_binop!(impl Div, div for NumericValue);
// forward_ref_binop!(impl Rem, rem for NumericValue);

// Helper function to combine approximation flags from operands and operation result
#[inline(always)]
pub(crate) fn combine_approximation_flags(
    self_trans: bool,
    rhs_trans: bool,
    self_rat_approx: bool,
    rhs_rat_approx: bool,
    rat_overflow: bool,
    result_value: &NumericValue,
) -> Option<crate::ApproximationType> {
    use crate::ApproximationType;

    if self_trans || rhs_trans {
        // Transcendental dominates all other flags
        Some(ApproximationType::Transcendental)
    } else if self_rat_approx || rhs_rat_approx {
        // Propagate existing rational approximation unless result demoted to Rational
        if matches!(result_value, NumericValue::Rational(_, _)) {
            None // Demoted back to exact Rational - flag clears
        } else {
            Some(ApproximationType::RationalApproximation)
        }
    } else if rat_overflow {
        // Lower layer signaled non-terminating rational overflowed
        Some(ApproximationType::RationalApproximation)
    } else {
        None
    }
}

// Number wrapper implementations
impl Add for Number {
    type Output = Number;
    fn add(self, rhs: Number) -> Number {
        // Check flags BEFORE moving
        let self_trans = self.is_transcendental();
        let rhs_trans = rhs.is_transcendental();
        let self_rat_approx = self.is_rational_approximation();
        let rhs_rat_approx = rhs.is_rational_approximation();

        // Compute ONCE - lower layer returns flag for non-terminating rational overflow
        let (result_value, rat_overflow) = self.value + rhs.value;

        // Combine flags using helper
        let apprx = combine_approximation_flags(
            self_trans,
            rhs_trans,
            self_rat_approx,
            rhs_rat_approx,
            rat_overflow,
            &result_value,
        );

        Number {
            value: result_value,
            apprx,
        }
    }
}

impl Sub for Number {
    type Output = Number;
    fn sub(self, rhs: Number) -> Number {
        // Check flags BEFORE moving
        let self_trans = self.is_transcendental();
        let rhs_trans = rhs.is_transcendental();
        let self_rat_approx = self.is_rational_approximation();
        let rhs_rat_approx = rhs.is_rational_approximation();

        // Compute ONCE - lower layer returns flag for non-terminating rational overflow
        let (result_value, rat_overflow) = self.value - rhs.value;

        // Combine flags using helper
        let apprx = combine_approximation_flags(
            self_trans,
            rhs_trans,
            self_rat_approx,
            rhs_rat_approx,
            rat_overflow,
            &result_value,
        );

        let result = Number {
            value: result_value,
            apprx,
        };

        // Skip demotion for obviously-large BigDecimals (saves ~20-30ns)
        match &result.value {
            NumericValue::BigDecimal(bd) => {
                use bigdecimal::BigDecimal;
                const LARGE_THRESHOLD: i64 = i64::MAX / 1000;
                if bd.abs() > BigDecimal::from(LARGE_THRESHOLD) {
                    return result; // Too large to demote, skip expensive checks
                }
            }
            _ => {}
        }

        result.try_demote()
    }
}

impl Mul for Number {
    type Output = Number;
    fn mul(self, rhs: Number) -> Number {
        // Check flags BEFORE moving
        let self_trans = self.is_transcendental();
        let rhs_trans = rhs.is_transcendental();
        let self_rat_approx = self.is_rational_approximation();
        let rhs_rat_approx = rhs.is_rational_approximation();

        // Compute ONCE - lower layer handles terminating checks and returns flag
        let (result_value, rat_overflow) = self.value * rhs.value;

        // Combine flags using helper
        let apprx = combine_approximation_flags(
            self_trans,
            rhs_trans,
            self_rat_approx,
            rhs_rat_approx,
            rat_overflow,
            &result_value,
        );

        let result = Number {
            value: result_value,
            apprx,
        };

        // Skip demotion for obviously-large BigDecimals (saves ~20-30ns)
        match &result.value {
            NumericValue::BigDecimal(bd) => {
                use bigdecimal::BigDecimal;
                const LARGE_THRESHOLD: i64 = i64::MAX / 1000;
                if bd.abs() > BigDecimal::from(LARGE_THRESHOLD) {
                    return result; // Too large to demote, skip expensive checks
                }
            }
            _ => {}
        }

        result.try_demote()
    }
}

impl Div for Number {
    type Output = Number;
    fn div(self, rhs: Number) -> Number {
        // Check flags BEFORE moving
        let self_trans = self.is_transcendental();
        let rhs_trans = rhs.is_transcendental();
        let self_rat_approx = self.is_rational_approximation();
        let rhs_rat_approx = rhs.is_rational_approximation();

        // Compute ONCE - lower layer returns flag for non-terminating rational overflow
        let (result_value, rat_overflow) = self.value / rhs.value;

        // Combine flags using helper
        let apprx = combine_approximation_flags(
            self_trans,
            rhs_trans,
            self_rat_approx,
            rhs_rat_approx,
            rat_overflow,
            &result_value,
        );

        let result = Number {
            value: result_value,
            apprx,
        };

        // Skip demotion for obviously-large BigDecimals (saves ~20-30ns)
        match &result.value {
            NumericValue::BigDecimal(bd) => {
                use bigdecimal::BigDecimal;
                const LARGE_THRESHOLD: i64 = i64::MAX / 1000;
                if bd.abs() > BigDecimal::from(LARGE_THRESHOLD) {
                    return result; // Too large to demote, skip expensive checks
                }
            }
            _ => {}
        }

        result.try_demote()
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

        let result = Number {
            value: self.value % rhs.value,
            apprx,
        };

        // Try to demote Decimal result back to Rational when possible
        result.try_demote()
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
