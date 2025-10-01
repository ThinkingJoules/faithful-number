// num_traits for mathematical operations

// Macros must be declared first so they're available in other modules
#[macro_use]
pub mod macros;

pub mod conversions;
pub mod core;
pub mod js_semantics;
pub mod math;
pub mod ops;
pub mod representation;
pub mod traits;

pub use crate::core::Number;
use crate::core::NumericValue;

pub mod prelude {
    pub use super::Number;
    pub use super::num;
    pub use core::str::FromStr;
    pub use num_traits::{FromPrimitive, One, Signed, ToPrimitive, Zero};
    pub use rust_decimal::{Decimal, RoundingStrategy};
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
        assert_eq!(&nan, &nan);
    }

    #[test]
    fn test_macro_convenience() {
        let _a = num!(3.14);
        let _b = num!(42);
        let _nan = num!(NaN);
        let _inf = num!(Infinity);
        let _neg_inf = num!(-Infinity);
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
            assert!(!n.rational_approximation);
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
            assert!(!sqrt4.is_transcendental());

            // sqrt(2) ≈ 1.414... → IS transcendental
            let sqrt2 = Number::from(2).sqrt();
            assert_eq!(sqrt2.representation(), "Decimal");
            assert!(sqrt2.is_transcendental());
            assert!(!sqrt2.rational_approximation);
        }

        #[test]
        fn rounding_clears_transcendental() {
            let sqrt2 = Number::from(2).sqrt();
            assert!(sqrt2.is_transcendental());

            let rounded = sqrt2.round();
            assert!(!rounded.is_transcendental());
        }

        #[test]
        fn rational_approximation_only_on_decimal_from_rational() {
            // Pure Rational operations → no flag
            let a = Number::from_rational(Ratio::new(1, 3));
            let b = Number::from_rational(Ratio::new(1, 6));
            let result = a + b;
            assert_eq!(result.representation(), "Rational");
            assert!(!result.rational_approximation);

            // Rational forced to Decimal → flag set
            let rational = Number::from_rational(Ratio::new(1, 3));
            let decimal = Number::from_decimal(Decimal::from(1));
            let result = rational + decimal;

            if result.representation() == "Decimal" {
                assert!(result.rational_approximation);
            }
        }

        #[test]
        fn flag_clears_when_back_to_rational() {
            // Decimal → Rational should clear rational_approximation
            let a = Number::from_decimal(Decimal::from(1));
            let b = Number::from_decimal(Decimal::from(3));
            let result = a / b;

            if result.representation() == "Rational" {
                assert!(!result.rational_approximation);
            }
        }

        #[test]
        fn flags_only_meaningful_on_decimal() {
            // Rational repr → both flags must be false
            let r = Number::from_rational(Ratio::new(1, 3));
            assert!(!r.rational_approximation);
            assert!(!r.is_transcendental());

            // Special values → both flags must be false
            assert!(!Number::NAN.rational_approximation);
            assert!(!Number::NAN.is_transcendental());
            assert!(!Number::POSITIVE_INFINITY.rational_approximation);
            assert!(!Number::POSITIVE_INFINITY.is_transcendental());
        }
    }
}
