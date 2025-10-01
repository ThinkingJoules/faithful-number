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

    // !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
    // EXPECTED FAILURE: This test demonstrates the NaN == NaN bug.
    // JavaScript requires NaN != NaN, but Rust's Eq trait requires reflexivity.
    // We chose HashMap/HashSet compatibility over JavaScript semantics.
    // This test SHOULD FAIL to remind us of the trade-off.
    // See PartialEq implementation in traits.rs for the intentional bug.
    // !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
    #[test]
    fn test_nan_semantics() {
        let nan = Number::NAN;
        assert_ne!(&nan, &nan); // NaN != NaN in JS - use references
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
mod js_semantics_tests {
    use super::*;
    use rust_decimal::Decimal;
    use std::str::FromStr;

    macro_rules! assert_js_eq {
        ($left:expr, $right:expr) => {
            assert!(
                $left.js_strict_equals(&$right),
                "Expected {:?} === {:?} (JS strict equality)",
                $left,
                $right
            );
        };
    }

    macro_rules! assert_js_ne {
        ($left:expr, $right:expr) => {
            assert!(
                !$left.js_strict_equals(&$right),
                "Expected {:?} !== {:?} (JS strict inequality)",
                $left,
                $right
            );
        };
    }

    // =================== CONSTANTS AND BASIC PROPERTIES ===================

    #[test]
    fn test_constants() {
        assert!(Number::NAN.is_nan());
        assert!(Number::POSITIVE_INFINITY.is_positive_infinity());
        assert!(Number::NEGATIVE_INFINITY.is_negative_infinity());
        assert!(Number::ZERO.is_finite());
        assert!(Number::ONE.is_finite());

        assert!(!Number::NAN.is_finite());
        assert!(!Number::POSITIVE_INFINITY.is_finite());
        assert!(!Number::NEGATIVE_INFINITY.is_finite());
    }

    #[test]
    fn test_type_predicates() {
        let finite = num!(42.5);
        let nan = Number::NAN;
        let pos_inf = Number::POSITIVE_INFINITY;
        let neg_inf = Number::NEGATIVE_INFINITY;

        // is_finite
        assert!(finite.is_finite());
        assert!(!nan.is_finite());
        assert!(!pos_inf.is_finite());
        assert!(!neg_inf.is_finite());

        // is_infinite
        assert!(!finite.is_infinite());
        assert!(!nan.is_infinite());
        assert!(pos_inf.is_infinite());
        assert!(neg_inf.is_infinite());

        // is_nan
        assert!(!finite.is_nan());
        assert!(nan.is_nan());
        assert!(!pos_inf.is_nan());
        assert!(!neg_inf.is_nan());
    }

    // =================== BASIC ARITHMETIC ===================

    #[test]
    fn test_basic_arithmetic() {
        let a = num!(5);
        let b = num!(3);

        assert_js_eq!(&a + &b, num!(8));
        assert_js_eq!(&a - &b, num!(2));
        assert_js_eq!(&a * &b, num!(15));

        // Division precision test
        let result = &a / &b;
        assert!(result.is_finite());
        let expected = Number::from(Decimal::from_str("1.6666666666666666667").unwrap());
        let diff = (result - expected).abs();
        assert!(diff < Number::from(Decimal::from_str("0.0000000000000000001").unwrap()));

        assert_js_eq!(&a % &b, num!(2));

        assert_js_eq!(-a.clone(), num!(-5));
        assert_js_eq!(-(-a.clone()), num!(5));
    }

    #[test]
    fn test_decimal_precision() {
        let a = Number::from(Decimal::from_str("0.1").unwrap());
        let b = Number::from(Decimal::from_str("0.2").unwrap());
        let expected = Number::from(Decimal::from_str("0.3").unwrap());

        // This should work with Decimal (unlike floating point)
        assert_js_eq!(&a + &b, expected);
    }

    // =================== SPECIAL VALUE ARITHMETIC ===================

    #[test]
    fn test_nan_arithmetic() {
        let nan = Number::NAN;
        let finite = num!(5);
        let inf = Number::POSITIVE_INFINITY;

        // NaN + anything = NaN
        assert!((&nan + &finite).is_nan());
        assert!((&finite + &nan).is_nan());
        assert!((&nan + &inf).is_nan());
        assert!((&nan + &nan).is_nan());

        // NaN with all operations
        assert!((&nan - &finite).is_nan());
        assert!((&nan * &finite).is_nan());
        assert!((&nan / &finite).is_nan());
        assert!((&nan % &finite).is_nan());
        assert!((-nan.clone()).is_nan());
    }

    #[test]
    fn test_infinity_arithmetic() {
        let pos_inf = Number::POSITIVE_INFINITY;
        let neg_inf = Number::NEGATIVE_INFINITY;
        let finite = num!(5);
        let zero = num!(0);

        // Infinity + finite = Infinity
        assert_js_eq!(&pos_inf + &finite, pos_inf);
        assert_js_eq!(&neg_inf + &finite, neg_inf);
        assert_js_eq!(&finite + &pos_inf, pos_inf);

        // Infinity - Infinity = NaN
        assert!((&pos_inf - &pos_inf).is_nan());
        assert!((&neg_inf - &neg_inf).is_nan());

        // Infinity + (-Infinity) = NaN
        assert!((&pos_inf + &neg_inf).is_nan());

        // Infinity * finite = Infinity (with sign rules)
        assert_js_eq!(&pos_inf * &finite, pos_inf);
        assert_js_eq!(&neg_inf * &finite, neg_inf);
        assert_js_eq!(&pos_inf * &num!(-5), neg_inf);

        // Infinity * 0 = NaN
        assert!((&pos_inf * &zero).is_nan());
        assert!((&neg_inf * &zero).is_nan());

        // Infinity / finite = Infinity
        assert_js_eq!(&pos_inf / &finite, pos_inf);
        assert_js_eq!(&neg_inf / &finite, neg_inf);

        // Infinity / Infinity = NaN
        assert!((&pos_inf / &pos_inf).is_nan());
        assert!((&pos_inf / &neg_inf).is_nan());

        // finite / Infinity = 0
        assert_js_eq!(&finite / &pos_inf, zero);
        assert_js_eq!(&finite / &neg_inf, zero);
    }

    #[test]
    fn test_division_by_zero() {
        let pos = num!(5);
        let neg = num!(-5);
        let zero = num!(0);

        // Positive / 0 = +Infinity
        assert_js_eq!(&pos / &zero, Number::POSITIVE_INFINITY);

        // Negative / 0 = -Infinity
        assert_js_eq!(&neg / &zero, Number::NEGATIVE_INFINITY);

        // 0 / 0 = NaN
        assert!((&zero / &zero).is_nan());
    }

    #[test]
    fn test_modulo_special_cases() {
        let finite = num!(5);
        let zero = num!(0);
        let inf = Number::POSITIVE_INFINITY;

        // x % 0 = NaN
        assert!((&finite % &zero).is_nan());

        // Infinity % x = NaN
        assert!((&inf % &finite).is_nan());

        // x % Infinity = x
        assert_js_eq!(&finite % &inf, finite);

        // Test negative modulo behavior (JS-specific)
        assert_js_eq!(num!(-5) % num!(3), num!(-2));
        assert_js_eq!(num!(5) % num!(-3), num!(2));
        assert_js_eq!(num!(-5) % num!(-3), num!(-2));
    }

    // =================== COMPARISON SEMANTICS ===================

    // !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
    // EXPECTED FAILURE: This test demonstrates the NaN == NaN bug.
    // The assert_ne!(&nan, &nan) line WILL FAIL because our PartialEq returns true.
    // This is an intentional trade-off for Rust's Eq trait and HashMap compatibility.
    // This test SHOULD FAIL to remind us that NaN semantics don't match JavaScript.
    // See PartialEq implementation in traits.rs for the intentional bug.
    // !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
    #[test]
    fn test_nan_comparison_semantics() {
        let nan = Number::NAN;
        let finite = num!(5);

        // NaN != anything
        assert_js_ne!(&nan, &finite);
        assert_js_ne!(&finite, &nan);
        assert_js_ne!(&nan, &Number::POSITIVE_INFINITY);

        // NaN comparisons always return None/false
        assert_eq!(nan.partial_cmp(&finite), None);
        assert_eq!(finite.partial_cmp(&nan), None);
        assert_eq!(nan.partial_cmp(&Number::NAN), None);

        assert_eq!(nan.js_less_than(&finite), None);
        assert_eq!(finite.js_less_than(&nan), None);
        assert_eq!(&nan, &nan); // Rust PartialEq requires NaN == NaN
        // NaN != NaN (most important JS quirk)
        assert_js_ne!(&nan, &nan);
    }

    #[test]
    fn test_infinity_comparison() {
        let pos_inf = Number::POSITIVE_INFINITY;
        let neg_inf = Number::NEGATIVE_INFINITY;
        let finite = num!(1000000);

        // +Infinity > everything except +Infinity
        assert!(&pos_inf > &finite);
        assert!(&pos_inf > &neg_inf);
        assert_js_eq!(&pos_inf, &pos_inf);

        // -Infinity < everything except -Infinity
        assert!(&neg_inf < &finite);
        assert!(&neg_inf < &pos_inf);
        assert_js_eq!(&neg_inf, &neg_inf);
    }

    #[test]
    fn test_finite_comparison() {
        let a = num!(5);
        let b = num!(3);
        let c = num!(5);

        assert!(&a > &b);
        assert!(&b < &a);
        assert_js_eq!(&a, &c);
        assert!(&a >= &c);
        assert!(&a <= &c);
        assert!(&a >= &b);
        assert!(&b <= &a);
    }

    // !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
    // EXPECTED FAILURE: This test demonstrates the NaN == NaN bug.
    // The js_equals and js_strict_equals methods use our PartialEq internally,
    // which returns true for NaN == NaN (breaking JavaScript semantics).
    // This test SHOULD FAIL to remind us that NaN equality doesn't work correctly.
    // See PartialEq implementation in traits.rs for the intentional bug.
    // !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
    #[test]
    fn test_js_equality_vs_strict_equality() {
        let a = num!(5);
        let b = num!(5.0);
        let c = num!(3);

        // Should be the same for numbers
        assert_eq!(a.js_equals(&b), a.js_strict_equals(&b));
        assert_eq!(a.js_equals(&c), a.js_strict_equals(&c));

        // Test with special values - THESE WILL FAIL (see comment above)
        let nan = Number::NAN;
        assert!(!nan.js_equals(&nan));
        assert!(!nan.js_strict_equals(&nan));
    }

    // =================== BITWISE OPERATIONS ===================

    #[test]
    fn test_bitwise_operations() {
        let a = num!(12); // 0b1100
        let b = num!(5); // 0b0101

        // Basic bitwise operations
        assert_js_eq!(&a & &b, num!(4)); // 0b0100
        assert_js_eq!(&a | &b, num!(13)); // 0b1101
        assert_js_eq!(&a ^ &b, num!(9)); // 0b1001

        // Bitwise NOT
        assert_js_eq!(!a.clone(), num!(-13)); // ~12 = -13 in two's complement

        // Shifts
        assert_js_eq!(&a << &num!(1), num!(24)); // 12 << 1 = 24
        assert_js_eq!(&a >> &num!(1), num!(6)); // 12 >> 1 = 6
    }

    #[test]
    fn test_bitwise_with_decimals() {
        // JS converts to i32 for bitwise ops, truncating decimals
        let decimal = num!(12.7);
        let integer = num!(5);

        assert_js_eq!(&decimal & &integer, num!(4)); // 12 & 5 = 4
        assert_js_eq!(&decimal | &integer, num!(13)); // 12 | 5 = 13
    }

    #[test]
    fn test_bitwise_with_special_values() {
        let nan = Number::NAN;
        let inf = Number::POSITIVE_INFINITY;
        let finite = num!(5);

        // Bitwise with NaN -> 0 (NaN converts to 0 in bitwise operations)
        assert_js_eq!(&nan & &finite, num!(0));
        assert_js_eq!(&finite & &nan, num!(0));

        // Bitwise with Infinity -> treat as 0
        assert_js_eq!(&inf & &finite, num!(0));
        assert_js_eq!(&finite & &inf, num!(0));
    }

    #[test]
    fn test_unsigned_right_shift() {
        let a = num!(-1);
        let shift = num!(1);

        // >>> is different from >> for negative numbers
        assert_js_eq!(
            a.clone().unsigned_right_shift(shift.clone()),
            num!(2147483647)
        );

        let b = num!(8);
        assert_js_eq!(b.clone().unsigned_right_shift(num!(2)), num!(2));
    }

    // =================== MATHEMATICAL FUNCTIONS ===================

    #[test]
    fn test_abs() {
        assert_js_eq!(num!(5).abs(), num!(5));
        assert_js_eq!(num!(-5).abs(), num!(5));
        assert_js_eq!(num!(0).abs(), num!(0));

        // Special values
        assert!(Number::NAN.abs().is_nan());
        assert_js_eq!(Number::POSITIVE_INFINITY.abs(), Number::POSITIVE_INFINITY);
        assert_js_eq!(Number::NEGATIVE_INFINITY.abs(), Number::POSITIVE_INFINITY);
    }

    #[test]
    fn test_floor_ceil_round_trunc() {
        let positive = num!(3.7);
        let negative = num!(-3.7);

        // Floor
        assert_js_eq!(positive.clone().floor(), num!(3));
        assert_js_eq!(negative.clone().floor(), num!(-4));

        // Ceil
        assert_js_eq!(positive.clone().ceil(), num!(4));
        assert_js_eq!(negative.clone().ceil(), num!(-3));

        // Round (ties to even in JS)
        assert_js_eq!(num!(3.5).round(), num!(4));
        assert_js_eq!(num!(-3.5).round(), num!(-3));

        // Trunc
        assert_js_eq!(positive.clone().trunc(), num!(3));
        assert_js_eq!(negative.clone().trunc(), num!(-3));

        // Special values
        assert!(Number::NAN.floor().is_nan());
        assert_js_eq!(Number::POSITIVE_INFINITY.floor(), Number::POSITIVE_INFINITY);
    }

    #[test]
    fn test_sqrt() {
        assert_js_eq!(num!(9).sqrt(), num!(3));
        assert_js_eq!(num!(0).sqrt(), num!(0));

        // Negative -> NaN
        assert!(num!(-1).sqrt().is_nan());

        // Special values
        assert!(Number::NAN.sqrt().is_nan());
        assert_js_eq!(Number::POSITIVE_INFINITY.sqrt(), Number::POSITIVE_INFINITY);
        assert!(Number::NEGATIVE_INFINITY.sqrt().is_nan());
    }

    #[test]
    fn test_pow() {
        assert_js_eq!(num!(2).pow(num!(3)), num!(8));
        assert_js_eq!(num!(9).pow(num!(0.5)), num!(3)); // sqrt

        // Special cases
        assert_js_eq!(num!(0).pow(num!(0)), num!(1)); // 0^0 = 1 in JS
        assert_js_eq!(num!(1).pow(Number::POSITIVE_INFINITY), num!(1));
        assert!(num!(-1).pow(num!(0.5)).is_nan()); // Negative^fractional = NaN

        // Infinity cases
        assert_js_eq!(
            Number::POSITIVE_INFINITY.pow(num!(2)),
            Number::POSITIVE_INFINITY
        );
        assert_js_eq!(Number::POSITIVE_INFINITY.pow(num!(0)), num!(1));
        assert!(Number::POSITIVE_INFINITY.pow(Number::NAN).is_nan());
    }

    #[test]
    fn test_trigonometric_functions() {
        // Basic trig (approximate due to decimal precision)
        let pi_half = Number::from(Decimal::from_str("1.5707963267948966").unwrap());

        assert!((num!(0).sin() - num!(0)).abs().to_f64() < 1e-10);
        assert!((pi_half.sin() - num!(1)).abs().to_f64() < 1e-10);
        assert!((num!(0).cos() - num!(1)).abs().to_f64() < 1e-10);

        // Special values
        assert!(Number::NAN.sin().is_nan());
        assert!(Number::POSITIVE_INFINITY.sin().is_nan());
        assert!(Number::NEGATIVE_INFINITY.cos().is_nan());
    }

    #[test]
    fn test_logarithms() {
        assert_js_eq!(num!(1).log(), num!(0));
        assert!((num!(10).log10() - num!(1)).abs().to_f64() < 1e-10);
        assert!((num!(8).log2() - num!(3)).abs().to_f64() < 1e-10);

        // Negative -> NaN
        assert!(num!(-1).log().is_nan());
        assert!(num!(0).log().is_negative_infinity());

        // Special values
        assert!(Number::NAN.log().is_nan());
        assert_js_eq!(Number::POSITIVE_INFINITY.log(), Number::POSITIVE_INFINITY);
    }

    // =================== TYPE CONVERSIONS ===================

    #[test]
    fn test_to_i32_conversion() {
        assert_eq!(num!(42).to_i32_js_coerce(), 42);
        assert_eq!(num!(42.7).to_i32_js_coerce(), 42); // Truncation
        assert_eq!(num!(-42.7).to_i32_js_coerce(), -42);

        // Special values
        assert_eq!(Number::NAN.to_i32_js_coerce(), 0);
        assert_eq!(Number::POSITIVE_INFINITY.to_i32_js_coerce(), 0);
        assert_eq!(Number::NEGATIVE_INFINITY.to_i32_js_coerce(), 0);
    }

    #[test]
    fn test_to_u32_conversion() {
        assert_eq!(num!(42).to_u32_js_coerce(), 42);
        assert_eq!(num!(-1).to_u32_js_coerce(), 4294967295); // Wraps to max u32

        // Special values become 0
        assert_eq!(Number::NAN.to_u32_js_coerce(), 0);
        assert_eq!(Number::POSITIVE_INFINITY.to_u32_js_coerce(), 0);
    }

    #[test]
    fn test_to_f64_conversion() {
        assert_eq!(num!(42.5).to_f64(), 42.5);
        assert!(Number::NAN.to_f64().is_nan());
        assert_eq!(Number::POSITIVE_INFINITY.to_f64(), f64::INFINITY);
        assert_eq!(Number::NEGATIVE_INFINITY.to_f64(), f64::NEG_INFINITY);
    }

    #[test]
    fn test_from_f64_conversion() {
        assert_js_eq!(Number::from(42.5), num!(42.5));
        assert!(Number::from(f64::NAN).is_nan());
        assert_js_eq!(Number::from(f64::INFINITY), Number::POSITIVE_INFINITY);
        assert_js_eq!(Number::from(f64::NEG_INFINITY), Number::NEGATIVE_INFINITY);
    }

    #[test]
    fn test_try_from_conversions() {
        let finite = num!(42);
        let nan = Number::NAN;

        assert_eq!(i64::try_from(finite), Ok(42));
        assert_eq!(i64::try_from(nan), Err(()));
        assert_eq!(i64::try_from(Number::POSITIVE_INFINITY), Err(()));
    }

    // =================== STRING CONVERSIONS ===================

    #[test]
    fn test_to_js_string() {
        assert_eq!(num!(42).to_js_string(), "42");
        assert_eq!(num!(42.5).to_js_string(), "42.5");
        assert_eq!(num!(0).to_js_string(), "0");
        assert_eq!(num!(-0).to_js_string(), "0"); // -0 becomes "0"

        // Special values
        assert_eq!(Number::NAN.to_js_string(), "NaN");
        assert_eq!(Number::POSITIVE_INFINITY.to_js_string(), "Infinity");
        assert_eq!(Number::NEGATIVE_INFINITY.to_js_string(), "-Infinity");

        // Large/small numbers (scientific notation handling)
        assert_eq!(num!(1e21).to_js_string(), "1e+21");
        assert_eq!(num!(1e-7).to_js_string(), "1e-7");
    }

    #[test]
    fn test_display_trait() {
        assert_eq!(format!("{}", num!(42)), "42");
        assert_eq!(format!("{}", Number::NAN), "NaN");
        assert_eq!(format!("{}", Number::POSITIVE_INFINITY), "Infinity");
    }

    #[test]
    fn test_from_str() {
        assert_js_eq!(Number::from_str("42").unwrap(), num!(42));
        assert_js_eq!(Number::from_str("42.5").unwrap(), num!(42.5));
        assert_js_eq!(Number::from_str("-42").unwrap(), num!(-42));

        // Whitespace handling (JS trims)
        assert_js_eq!(Number::from_str("  42  ").unwrap(), num!(42));

        // Special values
        assert!(Number::from_str("NaN").unwrap().is_nan());
        assert_js_eq!(
            Number::from_str("Infinity").unwrap(),
            Number::POSITIVE_INFINITY
        );
        assert_js_eq!(
            Number::from_str("-Infinity").unwrap(),
            Number::NEGATIVE_INFINITY
        );

        // Empty string converts to 0 in JS
        assert_js_eq!(Number::from_str("").unwrap(), Number::ZERO);

        // Invalid strings
        assert!(Number::from_str("not a number").is_err());
    }

    // =================== TRUTHINESS SEMANTICS ===================

    #[test]
    fn test_truthiness() {
        // Falsy values
        assert!(num!(0).is_falsy());
        assert!(num!(-0).is_falsy());
        assert!(Number::NAN.is_falsy());

        // Truthy values
        assert!(num!(1).is_truthy());
        assert!(num!(-1).is_truthy());
        assert!(num!(0.1).is_truthy());
        assert!(Number::POSITIVE_INFINITY.is_truthy());
        assert!(Number::NEGATIVE_INFINITY.is_truthy());

        // Inverse relationship
        assert_eq!(num!(0).is_truthy(), !num!(0).is_falsy());
        assert_eq!(num!(42).is_truthy(), !num!(42).is_falsy());
    }

    // =================== ASSIGNMENT OPERATORS ===================

    #[test]
    fn test_assignment_operators() {
        let mut a = num!(5);

        a += num!(3);
        assert_js_eq!(&a, &num!(8));

        a -= num!(2);
        assert_js_eq!(&a, &num!(6));

        a *= num!(2);
        assert_js_eq!(&a, &num!(12));

        a /= num!(3);
        assert_js_eq!(&a, &num!(4));

        a %= num!(3);
        assert_js_eq!(&a, &num!(1));
    }

    #[test]
    fn test_bitwise_assignment_operators() {
        let mut a = num!(12); // 0b1100

        a &= num!(5); // 0b0101
        assert_js_eq!(&a, &num!(4)); // 0b0100

        a |= num!(8); // 0b1000
        assert_js_eq!(&a, &num!(12)); // 0b1100

        a ^= num!(3); // 0b0011
        assert_js_eq!(&a, &num!(15)); // 0b1111

        a <<= num!(1);
        assert_js_eq!(&a, &num!(30));

        a >>= num!(2);
        assert_js_eq!(&a, &num!(7));
    }

    // =================== INCREMENT/DECREMENT ===================

    #[test]
    fn test_increment_decrement() {
        let a = num!(5);

        assert_js_eq!(a.clone().increment(), num!(6));
        assert_js_eq!(a.clone().decrement(), num!(4));

        // Special values
        assert!(Number::NAN.increment().is_nan());
        assert_js_eq!(
            Number::POSITIVE_INFINITY.increment(),
            Number::POSITIVE_INFINITY
        );
        assert_js_eq!(
            Number::NEGATIVE_INFINITY.decrement(),
            Number::NEGATIVE_INFINITY
        );
    }

    // =================== EDGE CASES AND COMPLEX SCENARIOS ===================

    #[test]
    fn test_complex_arithmetic_chains() {
        // Test operator precedence and associativity
        let result = num!(2) + num!(3) * num!(4); // Should be 14, not 20
        assert_js_eq!(result, num!(14));

        // Mixed special values
        let complex = &Number::POSITIVE_INFINITY - &Number::POSITIVE_INFINITY + num!(5);
        assert!(complex.is_nan()); // Inf - Inf = NaN, NaN + 5 = NaN
    }

    #[test]
    fn test_precision_edge_cases() {
        // Very large numbers - use a number that Decimal can handle
        let large = Number::from(Decimal::from_str("999999999999999999999999999").unwrap());
        assert!(large.is_finite());

        // Very small numbers
        let small = Number::from(Decimal::from_str("0.000000000000000000000000001").unwrap());
        assert!(small.is_finite());
        assert!(&small > &num!(0));
    }

    #[test]
    fn test_js_safe_integer_range() {
        // JavaScript's MAX_SAFE_INTEGER is 2^53 - 1
        let max_safe = num!(9007199254740991_i64);
        let beyond_safe = num!(9007199254740992_i64);

        assert!(max_safe.is_finite());
        assert!(beyond_safe.is_finite());
        // Note: Decimal should handle these accurately unlike f64
    }

    #[test]
    fn test_zero_edge_cases() {
        let pos_zero = num!(0);
        let neg_zero = num!(-0);

        // Should be equal in most contexts
        assert_js_eq!(&pos_zero, &neg_zero);

        // But different in some operations
        assert_js_eq!(&num!(1) / &pos_zero, Number::POSITIVE_INFINITY);
        assert_js_eq!(&num!(1) / &neg_zero, Number::NEGATIVE_INFINITY);
    }

    #[test]
    fn test_reference_operations() {
        // Test that reference operations work correctly
        let a = num!(5);
        let b = num!(3);

        assert_js_eq!(&a + &b, num!(8));
        assert_js_eq!(a.clone() + &b, num!(8));
        assert_js_eq!(&a + b.clone(), num!(8));
        // Consuming both
        assert_js_eq!(a.clone() + b.clone(), num!(8));
    }

    #[test]
    fn test_error_conditions() {
        // Operations that should never panic
        let nan = Number::NAN;
        let inf = Number::POSITIVE_INFINITY;
        let finite = num!(42);

        // These should all return well-defined results, never panic
        let _results = vec![
            &nan + &inf,
            &inf - &inf,
            &nan * &num!(0),
            &finite / &num!(0),
            &num!(0) / &num!(0),
            &inf % &finite,
            nan.sqrt(),
            num!(-1).sqrt(),
            num!(0).log(),
            num!(-1).log(),
        ];

        // All should succeed without panicking
    }

    // =================== PROPERTY-BASED TESTS ===================

    #[test]
    fn test_arithmetic_properties() {
        let a = num!(7);
        let b = num!(3);
        let c = num!(2);

        // Associativity (when no special values involved)
        assert_js_eq!((&a + &b) + &c, &a + (&b + &c));
        assert_js_eq!((&a * &b) * &c, &a * (&b * &c));

        // Commutativity
        assert_js_eq!(&a + &b, &b + &a);
        assert_js_eq!(&a * &b, &b * &a);

        // Distributivity
        assert_js_eq!(&a * (&b + &c), (&a * &b) + (&a * &c));

        // Identity elements
        assert_js_eq!(&a + &num!(0), a);
        assert_js_eq!(&a * &num!(1), a);

        // Inverse elements
        let a = num!(7); // Fresh binding for consuming
        assert_js_eq!(a.clone() + (-a.clone()), num!(0));
        let a = num!(7); // Fresh binding for consuming
        assert_js_eq!(&a / &a, num!(1)); // when a != 0
    }

    #[test]
    fn test_comparison_properties() {
        let a = num!(5);
        let b = num!(3);

        // Transitivity
        if &a > &b && &b > &num!(1) {
            assert!(&a > &num!(1));
        }

        // Antisymmetry
        assert!(!(&a > &b && &b > &a));

        // Reflexivity for equality
        assert_js_eq!(&a, &a);

        // Note: NaN breaks many of these properties, which is expected
    }
}
