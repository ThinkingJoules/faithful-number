// num_traits for mathematical operations

// Macros must be declared first so they're available in other modules
#[macro_use]
pub mod macros;

pub mod conversions;
pub mod core;
pub mod js_semantics;
pub mod math;
pub mod ops;
pub mod ordered;
pub mod precision;
pub mod representation;
pub mod traits;

#[cfg(feature = "format")]
pub mod format;

#[cfg(any(feature = "serde_str", feature = "serde_bin"))]
mod serde_impl;

use crate::core::NumericValue;
pub use crate::core::{ApproximationType, Number};
pub use crate::ordered::OrderedNumber;
pub use crate::precision::{get_default_precision, set_default_precision};

#[cfg(feature = "format")]
pub use crate::format::{
    DisplayOptions, ExpNotation, Notation, ParseError, ParseOptions, RegionalFormat,
};

pub mod repr {
    pub use bigdecimal::BigDecimal;
    pub use num_rational::Rational64;
    pub use rust_decimal::Decimal;
    pub use rust_decimal::RoundingStrategy;
}

pub mod prelude {
    pub use super::Number;
    pub use super::num;
    pub use super::repr::*;
    pub use core::str::FromStr;
    pub use num_traits::{FromPrimitive, One, Signed, ToPrimitive, Zero};
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constants() {
        assert!(Number::NAN.is_nan());
        assert!(Number::POSITIVE_INFINITY.is_positive_infinity());
        assert!(Number::NEGATIVE_INFINITY.is_negative_infinity());
    }

    #[test]
    fn test_arithmetic() {
        let a = num!(5);
        let b = num!(3);
        let result = &a + &b; // Use references to keep a and b
        assert_eq!(result.to_f64(), 8.0);
    }

    #[test]
    fn test_nan_semantics() {
        let nan = Number::NAN;
        // NaN behavior depends on feature flag
        #[cfg(feature = "js_nan_equality")]
        assert_eq!(&nan, &nan);
        #[cfg(not(feature = "js_nan_equality"))]
        assert_ne!(&nan, &nan); // IEEE 754: NaN != NaN
    }

    #[test]
    fn test_ergonomic_usage() {
        use num_traits::{One, Zero};

        // Natural arithmetic with references
        let a = Number::from(10);
        let b = Number::from(3);
        let _result = &a + &b * Number::one(); // Reference operations work

        // num_traits integration
        let _zero = Number::zero();
        let _one = Number::one();

        // Method chaining (consumes self)
        let _result = Number::from(16).sqrt().abs();
    }
}

#[cfg(test)]
mod metadata_tests {
    use super::*;
    use num_rational::Ratio;
    use rust_decimal::Decimal;
    use std::str::FromStr;

    /// Test what representation each constructor creates
    mod constructor_representations {
        use super::*;

        #[test]
        fn from_integer_creates_rational() {
            assert_eq!(Number::from(0).representation(), "Rational");
            assert_eq!(Number::from(1).representation(), "Rational");
            assert_eq!(Number::from(42).representation(), "Rational");
            assert_eq!(Number::from(-5).representation(), "Rational");
        }

        #[test]
        fn from_decimal_tries_rational_first() {
            // Even explicit from_decimal should prefer Rational when exact
            let d = Decimal::from(5);
            assert_eq!(Number::from_decimal(d).representation(), "Rational");

            // 0.5 = 1/2 should be Rational
            let d = Decimal::from_str("0.5").unwrap();
            let n = Number::from_decimal(d);
            // Could be Rational or Decimal depending on implementation
            // Just test that it has no approximation flag
            assert!(n.is_exact());
        }

        #[test]
        fn from_rational_creates_rational() {
            let r = Ratio::new(1, 3);
            assert_eq!(Number::from_rational(r).representation(), "Rational");
        }
    }

    /// Core metadata behavior tests
    mod metadata_behavior {
        use super::*;

        #[test]
        fn transcendental_only_on_decimal_approximations() {
            // sqrt(4) = 2 exactly → NOT transcendental
            let sqrt4 = Number::from(4).sqrt();
            sqrt4.assert_exact();

            // sqrt(2) ≈ 1.414... → IS transcendental
            let sqrt2 = Number::from(2).sqrt();
            // With high_precision feature, transcendental ops return BigDecimal
            #[cfg(feature = "high_precision")]
            assert_eq!(sqrt2.representation(), "BigDecimal");
            #[cfg(not(feature = "high_precision"))]
            assert_eq!(sqrt2.representation(), "Decimal");
            sqrt2.assert_transcendental();
        }

        #[test]
        fn rounding_clears_transcendental() {
            let sqrt2 = Number::from(2).sqrt();
            sqrt2.assert_transcendental();

            let rounded = sqrt2.round();
            rounded.assert_exact();
        }

        #[test]
        fn flag_clears_when_back_to_rational() {
            // Decimal → Rational should clear rational_approximation
            let a = Number::from_decimal(Decimal::from(1));
            let b = Number::from_decimal(Decimal::from(3));
            let result = a / b;

            if result.representation() == "Rational" {
                result.assert_exact();
            }
        }

        #[test]
        fn flags_only_meaningful_on_decimal() {
            // Rational repr → exact
            let r = Number::from_rational(Ratio::new(1, 3));
            r.assert_exact();

            // Special values → exact
            Number::NAN.assert_exact();
            Number::POSITIVE_INFINITY.assert_exact();
        }

        #[test]
        fn transcendental_propagates_through_operations() {
            let sqrt2 = Number::from(2).sqrt();
            sqrt2.assert_transcendental();

            // Transcendental + Rational → Transcendental
            let result = sqrt2.clone() + Number::from(3);
            result.assert_transcendental();

            // Rational + Transcendental → Transcendental
            let result = Number::from(5) + sqrt2.clone();
            result.assert_transcendental();

            // Transcendental * Rational → Transcendental
            let result = sqrt2.clone() * Number::from(2);
            result.assert_transcendental();

            // Transcendental / Rational → Transcendental
            let result = sqrt2 / Number::from(2);
            result.assert_transcendental();
        }

        #[test]
        fn transcendental_trumps_rational_approximation() {
            // Create a rational approximation via overflow
            let third = Number::from_rational(Ratio::new(1, 3)); // Non-terminating
            let huge1 = Number::from_rational(Ratio::new(1, 4_000_000_000));
            let huge2 = Number::from_rational(Ratio::new(1, 3_000_000_000));
            let rat_approx = third * huge1 * huge2; // Overflows to Decimal

            // MUST have rational_approximation and be BigDecimal to preserve precision
            assert_eq!(rat_approx.representation(), "BigDecimal");
            rat_approx.assert_rational_approximation();

            // Transcendental operation should trump
            let sqrt_of_approx = rat_approx.sqrt();
            sqrt_of_approx.assert_transcendental();
        }

        #[test]
        fn rounding_clears_all_approximation_flags() {
            // Rounding removes approximate decimal digits - result is exact

            // Transcendental: sqrt(2) ≈ 1.414... → rounds to 1 (exact)
            let sqrt2 = Number::from(2).sqrt();
            sqrt2.assert_transcendental();
            sqrt2.clone().round().assert_exact();
            sqrt2.clone().floor().assert_exact();
            sqrt2.clone().ceil().assert_exact();

            // Rational approximation: also cleared by rounding
            let third = Number::from_rational(Ratio::new(1, 3)); // Non-terminating
            let huge1 = Number::from_rational(Ratio::new(1, 4_000_000_000));
            let huge2 = Number::from_rational(Ratio::new(1, 3_000_000_000));
            let rat_approx = third * huge1 * huge2; // Overflows to BigDecimal (non-terminating)

            assert_eq!(rat_approx.representation(), "BigDecimal");
            rat_approx.assert_rational_approximation();

            rat_approx.round().assert_exact(); // Rounds to 0 (exact)
        }

        #[test]
        fn rational_approximation_must_preserve_precision_for_recovery() {
            // This test demonstrates that rational approximations MUST use BigDecimal
            // to preserve enough precision to recover the original rational after inverse operations

            let third = Number::from_rational(Ratio::new(1, 3)); // Non-terminating
            let huge1 = Number::from_rational(Ratio::new(1, 4_000_000_000));
            let huge2 = Number::from_rational(Ratio::new(1, 3_000_000_000));

            // Multiply - causes overflow, creates rational approximation
            let intermediate = third.clone() * huge1.clone() * huge2.clone();

            println!(
                "Intermediate representation: {}",
                intermediate.representation()
            );
            println!(
                "Is rational approximation? {}",
                intermediate.is_rational_approximation()
            );

            // KEY TEST: Can we recover the original 1/3?
            let recovered = intermediate / huge1 / huge2;

            println!("Recovered representation: {}", recovered.representation());
            println!("Recovered value: {:?}", recovered.to_rational64());

            // If intermediate was stored as Decimal (28 digits), we LOSE precision
            // If intermediate was stored as BigDecimal, we CAN recover exactly

            // EXPECTED: recovered should be Rational(1, 3) - the original value
            assert_eq!(
                recovered.representation(),
                "Rational",
                "Should recover to Rational representation"
            );
            assert_eq!(
                recovered.to_rational64(),
                Some(Ratio::new(1, 3)),
                "Should recover exact 1/3, not a truncated approximation"
            );
        }

        #[test]
        fn multiplication_should_demote_when_result_is_simple() {
            // This test demonstrates that try_demote() is necessary after multiplication
            // When multiplying BigDecimal values that result in simple rationals, we should demote

            // Create a huge BigDecimal (10^50, way beyond Decimal's range)
            let huge = Number::from(10.0).pow(Number::from(50));
            assert_eq!(
                huge.representation(),
                "BigDecimal",
                "10^50 should be BigDecimal"
            );

            // Create a tiny fraction (1/10^50) - also BigDecimal
            let tiny = Number::from(1.0) / Number::from(10.0).pow(Number::from(50));
            assert_eq!(
                tiny.representation(),
                "BigDecimal",
                "10^-50 should be BigDecimal"
            );

            // Multiply: 10^50 * 10^-50 = 1
            let result = huge * tiny;

            // Should demote back to Rational(1, 1)
            assert_eq!(
                result.representation(),
                "Rational",
                "BigDecimal * BigDecimal = 1 should demote to Rational"
            );
            assert_eq!(
                result.to_rational64(),
                Some(Ratio::new(1, 1)),
                "Should be exact 1/1"
            );
        }
    }
}
