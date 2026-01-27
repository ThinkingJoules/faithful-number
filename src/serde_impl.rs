//! Serde implementations for Number.
//!
//! Two mutually exclusive features:
//! - `serde_str`: String-based serialization (JSON, TOML, etc.)
//! - `serde_bin`: Binary serialization via onenum (bincode, etc.)

use crate::Number;
use crate::core::ApproximationType;

// ============================================================================
// serde_str: String-based serialization
// ============================================================================

#[cfg(all(feature = "serde_str", not(feature = "serde_bin")))]
mod str_impl {
    use super::*;
    use serde::de::{SeqAccess, Visitor};
    use serde::ser::SerializeSeq;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::fmt;

    impl Serialize for Number {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            // Serialize as array: ["value"] or ["value", "approx_type"]
            let value_str = self.to_string();

            match &self.apprx {
                None => {
                    let mut seq = serializer.serialize_seq(Some(1))?;
                    seq.serialize_element(&value_str)?;
                    seq.end()
                }
                Some(approx) => {
                    let mut seq = serializer.serialize_seq(Some(2))?;
                    seq.serialize_element(&value_str)?;
                    seq.serialize_element(&approx)?;
                    seq.end()
                }
            }
        }
    }

    impl Serialize for ApproximationType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let s = match self {
                ApproximationType::Transcendental => "transcendental",
                ApproximationType::RationalApproximation => "rational_approximation",
            };
            serializer.serialize_str(s)
        }
    }

    struct NumberVisitor;

    impl<'de> Visitor<'de> for NumberVisitor {
        type Value = Number;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str(
                "an array with 1 or 2 elements: [\"value\"] or [\"value\", \"approx_type\"]",
            )
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
        {
            use serde::de::Error;

            // First element: value string
            let value_str: String = seq
                .next_element()?
                .ok_or_else(|| Error::invalid_length(0, &"at least 1 element"))?;

            // Parse the value
            let mut num: Number = value_str
                .parse()
                .map_err(|_| Error::custom(format!("invalid number: {}", value_str)))?;

            // Second element (optional): approximation type
            if let Some(approx_str) = seq.next_element::<String>()? {
                let approx = match approx_str.as_str() {
                    "transcendental" => ApproximationType::Transcendental,
                    "rational_approximation" => ApproximationType::RationalApproximation,
                    other => {
                        return Err(Error::custom(format!(
                            "unknown approximation type: {}",
                            other
                        )));
                    }
                };
                num.apprx = Some(approx);
            }

            Ok(num)
        }
    }

    impl<'de> Deserialize<'de> for Number {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_seq(NumberVisitor)
        }
    }

    impl<'de> Deserialize<'de> for ApproximationType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            use serde::de::Error;

            let s = String::deserialize(deserializer)?;
            match s.as_str() {
                "transcendental" => Ok(ApproximationType::Transcendental),
                "rational_approximation" => Ok(ApproximationType::RationalApproximation),
                other => Err(Error::custom(format!(
                    "unknown approximation type: {}",
                    other
                ))),
            }
        }
    }
}

// ============================================================================
// serde_bin: Binary serialization via onenum
// ============================================================================

#[cfg(all(feature = "serde_bin", not(feature = "serde_str")))]
mod bin_impl {
    use super::*;
    use serde::de::Visitor;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::fmt;

    use bigdecimal::num_bigint::BigInt;
    use num_rational::Ratio;
    use onenum::{DefaultEqTolerance, Onum, OnumTrait, SpecialValue};

    // Approx byte encoding (suffix):
    // 0 = exact
    // 1 = transcendental
    // 2 = rational_approximation
    fn approx_to_byte(approx: &Option<ApproximationType>) -> u8 {
        match approx {
            None => 0,
            Some(ApproximationType::Transcendental) => 1,
            Some(ApproximationType::RationalApproximation) => 2,
        }
    }

    fn byte_to_approx(byte: u8) -> Option<ApproximationType> {
        match byte {
            0 => None,
            1 => Some(ApproximationType::Transcendental),
            2 => Some(ApproximationType::RationalApproximation),
            _ => None, // Unknown, treat as exact
        }
    }

    impl Serialize for Number {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            // Convert Number to Onum
            let onum: Onum<DefaultEqTolerance> = number_to_onum(self);

            // Get onenum bytes and append approx byte as suffix
            let onum_bytes = onum.as_bytes();
            let mut bytes = Vec::with_capacity(onum_bytes.len() + 1);
            bytes.extend_from_slice(onum_bytes);
            bytes.push(approx_to_byte(&self.apprx));

            serializer.serialize_bytes(&bytes)
        }
    }

    struct NumberVisitor;

    impl<'de> Visitor<'de> for NumberVisitor {
        type Value = Number;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("onenum encoded bytes with approx suffix")
        }

        fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            if v.is_empty() {
                return Err(E::custom("empty bytes"));
            }

            // Split off the approx suffix byte
            let (onenum_bytes, approx_byte) = v.split_at(v.len() - 1);
            let approx = byte_to_approx(approx_byte[0]);

            // Decode onenum
            let onum: Onum<DefaultEqTolerance> = Onum::from_bytes(onenum_bytes)
                .map_err(|e| E::custom(format!("onenum decode error: {:?}", e)))?;

            // Convert Onum back to Number
            let mut num = onum_to_number(onum);
            num.apprx = approx;

            Ok(num)
        }

        fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            self.visit_bytes(&v)
        }
    }

    impl<'de> Deserialize<'de> for Number {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_bytes(NumberVisitor)
        }
    }

    /// Convert a Number to a onenum Onum
    fn number_to_onum(num: &Number) -> Onum<DefaultEqTolerance> {
        use crate::core::NumericValue;

        match &num.value {
            NumericValue::NaN => Onum::from_special(SpecialValue::NaN),
            NumericValue::PositiveInfinity => Onum::from_special(SpecialValue::PositiveInfinity),
            NumericValue::NegativeInfinity => Onum::from_special(SpecialValue::NegativeInfinity),
            NumericValue::NegativeZero => Onum::from_special(SpecialValue::NegativeZero),
            NumericValue::Rational(r, _) => {
                // Convert Ratio<i64> to Onum
                let numer = *r.numer();
                let denom = *r.denom();
                if denom == 1 {
                    Onum::from_number(numer)
                } else {
                    // Create a BigInt ratio for onenum
                    let n = BigInt::from(numer);
                    let d = BigInt::from(denom);
                    let ratio = Ratio::new(n, d);
                    Onum::from_number(ratio)
                }
            }
            NumericValue::Decimal(d) => {
                // Convert Decimal to ratio: mantissa / 10^scale
                let mantissa = d.mantissa();
                let scale = d.scale();
                let numer = BigInt::from(mantissa);
                let denom = BigInt::from(10i64).pow(scale);
                let ratio = Ratio::new(numer, denom);
                Onum::from_number(ratio)
            }
            NumericValue::BigDecimal(bd) => {
                // Convert BigDecimal to string, then parse as Onum
                // This is not ideal but BigDecimal doesn't expose a clean ratio interface
                let s = bd.to_string();
                s.parse::<Onum<DefaultEqTolerance>>()
                    .unwrap_or_else(|_| Onum::from_special(SpecialValue::NaN))
            }
        }
    }

    /// Convert a onenum Onum back to a Number (via string to let auto-repr work)
    fn onum_to_number(onum: Onum<DefaultEqTolerance>) -> Number {
        // Check special values first
        if let Some(special) = onum.is_special_value() {
            return match special {
                SpecialValue::NaN => Number::NAN,
                SpecialValue::PositiveInfinity => Number::POSITIVE_INFINITY,
                SpecialValue::NegativeInfinity => Number::NEGATIVE_INFINITY,
                SpecialValue::NegativeZero => Number::neg_zero(),
                SpecialValue::PositiveZero => Number::ZERO,
            };
        }

        // For regular numbers, convert via string to let Number's FromStr handle repr selection
        let s = onum.to_string();
        s.parse::<Number>().unwrap_or(Number::NAN)
    }
}

#[cfg(test)]
mod tests {
    #[cfg(all(feature = "serde_str", not(feature = "serde_bin")))]
    mod serde_str_tests {
        use crate::Number;

        #[test]
        fn exact_number_serializes_as_single_element() {
            let n = Number::from(42);
            let json = serde_json::to_string(&n).unwrap();
            assert_eq!(json, r#"["42"]"#);
        }

        #[test]
        fn roundtrip_exact() {
            let original = Number::from(123);
            let json = serde_json::to_string(&original).unwrap();
            let back: Number = serde_json::from_str(&json).unwrap();
            assert_eq!(original, back);
            assert!(back.is_exact());
        }

        #[test]
        fn roundtrip_transcendental() {
            let original = Number::from(2).sqrt();
            let json = serde_json::to_string(&original).unwrap();
            assert!(json.contains(r#""transcendental""#));
            let back: Number = serde_json::from_str(&json).unwrap();
            assert!(!back.is_exact());
        }

        #[test]
        fn special_values() {
            let nan = Number::NAN;
            let json = serde_json::to_string(&nan).unwrap();
            let back: Number = serde_json::from_str(&json).unwrap();
            assert!(back.is_nan());

            let inf = Number::POSITIVE_INFINITY;
            let json = serde_json::to_string(&inf).unwrap();
            let back: Number = serde_json::from_str(&json).unwrap();
            assert!(back.is_positive_infinity());
        }
    }

    #[cfg(all(feature = "serde_bin", not(feature = "serde_str")))]
    mod serde_bin_tests {
        use crate::Number;

        #[test]
        fn roundtrip_exact() {
            let original = Number::from(123);
            let bytes = bincode::serialize(&original).unwrap();
            let back: Number = bincode::deserialize(&bytes).unwrap();
            assert_eq!(original, back);
            assert!(back.is_exact());
        }

        #[test]
        fn roundtrip_negative() {
            let original = Number::from(-456);
            let bytes = bincode::serialize(&original).unwrap();
            let back: Number = bincode::deserialize(&bytes).unwrap();
            assert_eq!(original, back);
        }

        #[test]
        fn roundtrip_decimal() {
            use std::str::FromStr;
            let original = Number::from_str("123.456").unwrap();
            let bytes = bincode::serialize(&original).unwrap();
            let back: Number = bincode::deserialize(&bytes).unwrap();
            assert_eq!(original, back);
        }

        #[test]
        fn roundtrip_transcendental() {
            let original = Number::from(2).sqrt();
            let bytes = bincode::serialize(&original).unwrap();
            let back: Number = bincode::deserialize(&bytes).unwrap();
            // Value should match (approximately, since we go through string)
            let diff = (original.to_f64() - back.to_f64()).abs();
            assert!(diff < 1e-10, "diff was {}", diff);
            // Approximation flag should be preserved
            assert!(!back.is_exact());
        }

        #[test]
        fn special_values() {
            // NaN
            let nan = Number::NAN;
            let bytes = bincode::serialize(&nan).unwrap();
            let back: Number = bincode::deserialize(&bytes).unwrap();
            assert!(back.is_nan());

            // Positive Infinity
            let inf = Number::POSITIVE_INFINITY;
            let bytes = bincode::serialize(&inf).unwrap();
            let back: Number = bincode::deserialize(&bytes).unwrap();
            assert!(back.is_positive_infinity());

            // Negative Infinity
            let neg_inf = Number::NEGATIVE_INFINITY;
            let bytes = bincode::serialize(&neg_inf).unwrap();
            let back: Number = bincode::deserialize(&bytes).unwrap();
            assert!(back.is_negative_infinity());

            // Negative Zero
            let neg_zero = Number::neg_zero();
            let bytes = bincode::serialize(&neg_zero).unwrap();
            let back: Number = bincode::deserialize(&bytes).unwrap();
            assert!(back.is_neg_zero());
        }

        #[test]
        fn bytes_sortable() {
            // Test that onenum bytes (excluding approx suffix) sort correctly
            let nums = vec![
                Number::from(-100),
                Number::from(-1),
                Number::from(0),
                Number::from(1),
                Number::from(100),
            ];

            // Get encoded bytes (without the approx suffix)
            let encoded: Vec<Vec<u8>> = nums
                .iter()
                .map(|n| {
                    let bytes = bincode::serialize(n).unwrap();
                    // bincode adds length prefix, so we need to extract just the onenum bytes
                    // The format is: [length as u64][onenum_bytes][approx_byte]
                    // Skip first 8 bytes (length), strip last byte (approx)
                    bytes[8..bytes.len() - 1].to_vec()
                })
                .collect();

            // Sort the byte arrays
            let sorted_encoded = {
                let mut sorted = encoded.clone();
                sorted.sort();
                sorted
            };

            // The sorted bytes should match the original order since nums were already sorted
            assert_eq!(encoded, sorted_encoded);
        }
    }
}
