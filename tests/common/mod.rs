use faithful_number::{ApproximationType, Number};
use num_rational::Ratio;
use rust_decimal::Decimal;

pub type Rational64 = Ratio<i64>;

/// Core meta-test structure with #[track_caller] for proper panic location
pub struct ArithmeticTestCase {
    pub name: &'static str,
    pub left: Number,
    pub right: Number,
}

impl ArithmeticTestCase {
    pub fn new(name: &'static str, left: Number, right: Number) -> Self {
        Self { name, left, right }
    }

    /// Test addition with full introspection
    #[track_caller]
    pub fn assert_add(
        &self,
        expected_value: Number,
        expected_repr: &str,
        expected_apprx: Option<ApproximationType>,
    ) -> &Self {
        let result = self.left.clone() + self.right.clone();
        self.assert_result("add", result, expected_value, expected_repr, expected_apprx);
        self
    }

    #[track_caller]
    pub fn assert_sub(
        &self,
        expected_value: Number,
        expected_repr: &str,
        expected_apprx: Option<ApproximationType>,
    ) -> &Self {
        let result = self.left.clone() - self.right.clone();
        self.assert_result("sub", result, expected_value, expected_repr, expected_apprx);
        self
    }

    #[track_caller]
    pub fn assert_mul(
        &self,
        expected_value: Number,
        expected_repr: &str,
        expected_apprx: Option<ApproximationType>,
    ) -> &Self {
        let result = self.left.clone() * self.right.clone();
        self.assert_result("mul", result, expected_value, expected_repr, expected_apprx);
        self
    }

    #[track_caller]
    pub fn assert_div(
        &self,
        expected_value: Number,
        expected_repr: &str,
        expected_apprx: Option<ApproximationType>,
    ) -> &Self {
        let result = self.left.clone() / self.right.clone();
        self.assert_result("div", result, expected_value, expected_repr, expected_apprx);
        self
    }

    #[track_caller]
    pub fn assert_rem(
        &self,
        expected_value: Number,
        expected_repr: &str,
        expected_apprx: Option<ApproximationType>,
    ) -> &Self {
        let result = self.left.clone() % self.right.clone();
        self.assert_result("rem", result, expected_value, expected_repr, expected_apprx);
        self
    }

    /// Core assertion helper - all operations funnel through here
    #[track_caller]
    fn assert_result(
        &self,
        op_name: &str,
        result: Number,
        expected: Number,
        expected_repr: &str,
        expected_apprx: Option<ApproximationType>,
    ) {
        // Value equality - handle NaN specially since NaN != NaN by default
        let values_match = if result.is_nan() && expected.is_nan() {
            true // Both NaN is a match
        } else {
            result == expected
        };
        assert!(
            values_match,
            "[{}:{}] Value mismatch: got {:?}, expected {:?}",
            self.name, op_name, result, expected
        );

        // Representation check
        assert_eq!(
            result.representation(),
            expected_repr,
            "[{}:{}] Representation mismatch: got {}, expected {}",
            self.name,
            op_name,
            result.representation(),
            expected_repr
        );

        // Approximation type check
        let result_apprx = if result.is_transcendental() {
            Some(ApproximationType::Transcendental)
        } else if result.is_rational_approximation() {
            Some(ApproximationType::RationalApproximation)
        } else {
            None
        };

        assert_eq!(
            result_apprx, expected_apprx,
            "[{}:{}] ApproximationType mismatch: got {:?}, expected {:?}",
            self.name, op_name, result_apprx, expected_apprx
        );
    }
}

/// Helper to create exact expectation (no approximation)
pub const fn exact() -> Option<ApproximationType> {
    None
}

/// Helper to create transcendental expectation
pub const fn is_transcendental() -> Option<ApproximationType> {
    Some(ApproximationType::Transcendental)
}

/// Helper to create rational approximation expectation
pub const fn is_rational_apprx() -> Option<ApproximationType> {
    Some(ApproximationType::RationalApproximation)
}

/// Extended test case for comprehensive property testing
pub struct CombinatorialTest {
    pub name: &'static str,
    pub operands: Vec<(&'static str, Number)>,
}

impl CombinatorialTest {
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            operands: Vec::new(),
        }
    }

    pub fn operand(mut self, label: &'static str, value: Number) -> Self {
        self.operands.push((label, value));
        self
    }

    /// Test commutativity: a + b == b + a
    #[track_caller]
    pub fn assert_commutative_add(&self) -> &Self {
        for (i, (label_a, a)) in self.operands.iter().enumerate() {
            for (label_b, b) in self.operands.iter().skip(i + 1) {
                let ab = a.clone() + b.clone();
                let ba = b.clone() + a.clone();
                assert_eq!(
                    ab, ba,
                    "[{}] Addition not commutative for {} + {} vs {} + {}",
                    self.name, label_a, label_b, label_b, label_a
                );
            }
        }
        self
    }

    /// Test commutativity: a * b == b * a
    #[track_caller]
    pub fn assert_commutative_mul(&self) -> &Self {
        for (i, (label_a, a)) in self.operands.iter().enumerate() {
            for (label_b, b) in self.operands.iter().skip(i + 1) {
                let ab = a.clone() * b.clone();
                let ba = b.clone() * a.clone();
                assert_eq!(
                    ab, ba,
                    "[{}] Multiplication not commutative for {} * {} vs {} * {}",
                    self.name, label_a, label_b, label_b, label_a
                );
            }
        }
        self
    }

    /// Test associativity: (a + b) + c == a + (b + c)
    #[track_caller]
    pub fn assert_associative_add(&self) -> &Self {
        for (label_a, a) in &self.operands {
            for (label_b, b) in &self.operands {
                for (label_c, c) in &self.operands {
                    let left = (a.clone() + b.clone()) + c.clone();
                    let right = a.clone() + (b.clone() + c.clone());
                    assert_eq!(
                        left, right,
                        "[{}] Addition not associative: ({} + {}) + {} vs {} + ({} + {})",
                        self.name, label_a, label_b, label_c, label_a, label_b, label_c
                    );
                }
            }
        }
        self
    }

    /// Test associativity: (a * b) * c == a * (b * c)
    #[track_caller]
    pub fn assert_associative_mul(&self) -> &Self {
        for (label_a, a) in &self.operands {
            for (label_b, b) in &self.operands {
                for (label_c, c) in &self.operands {
                    let left = (a.clone() * b.clone()) * c.clone();
                    let right = a.clone() * (b.clone() * c.clone());
                    assert_eq!(
                        left, right,
                        "[{}] Multiplication not associative: ({} * {}) * {} vs {} * ({} * {})",
                        self.name, label_a, label_b, label_c, label_a, label_b, label_c
                    );
                }
            }
        }
        self
    }

    /// Test distributivity: a * (b + c) == a * b + a * c
    #[track_caller]
    pub fn assert_distributive(&self) -> &Self {
        for (label_a, a) in &self.operands {
            for (label_b, b) in &self.operands {
                for (label_c, c) in &self.operands {
                    let left = a.clone() * (b.clone() + c.clone());
                    let right = a.clone() * b.clone() + a.clone() * c.clone();
                    assert_eq!(
                        left, right,
                        "[{}] Distributivity failed: {} * ({} + {}) vs {} * {} + {} * {}",
                        self.name, label_a, label_b, label_c, label_a, label_b, label_a, label_c
                    );
                }
            }
        }
        self
    }

    /// Test identity: a + 0 == a
    #[track_caller]
    pub fn assert_additive_identity(&self) -> &Self {
        let zero = Number::ZERO;
        for (label, num) in &self.operands {
            let result = num.clone() + zero.clone();
            assert_eq!(
                result, *num,
                "[{}] Additive identity failed for {}: {} + 0",
                self.name, label, label
            );
        }
        self
    }

    /// Test identity: a * 1 == a
    #[track_caller]
    pub fn assert_multiplicative_identity(&self) -> &Self {
        let one = Number::ONE;
        for (label, num) in &self.operands {
            let result = num.clone() * one.clone();
            assert_eq!(
                result, *num,
                "[{}] Multiplicative identity failed for {}: {} * 1",
                self.name, label, label
            );
        }
        self
    }

    /// Test inverse: a + (-a) == 0
    #[track_caller]
    pub fn assert_additive_inverse(&self) -> &Self {
        let zero = Number::ZERO;
        for (label, num) in &self.operands {
            if num.is_finite() {
                let result = num.clone() + (-num.clone());
                assert_eq!(
                    result, zero,
                    "[{}] Additive inverse failed for {}: {} + (-{})",
                    self.name, label, label, label
                );
            }
        }
        self
    }

    /// Test inverse: a * (1/a) == 1 (for non-zero)
    #[track_caller]
    pub fn assert_multiplicative_inverse(&self) -> &Self {
        let one = Number::ONE;
        for (label, num) in &self.operands {
            if num.is_finite() && *num != Number::ZERO {
                let result = num.clone() * (Number::ONE / num.clone());
                assert_eq!(
                    result, one,
                    "[{}] Multiplicative inverse failed for {}: {} * (1/{})",
                    self.name, label, label, label
                );
            }
        }
        self
    }
}

/// Helper macros for creating test values
#[macro_export]
macro_rules! rational {
    ($n:expr, $d:expr) => {
        Number::from_rational(Rational64::new($n, $d))
    };
}

#[macro_export]
macro_rules! decimal {
    ($n:expr, $scale:expr) => {
        Number::from_decimal(faithful_number::repr::Decimal::new($n, $scale))
    };
}
