//! Bitwise operations for Number.
//!
//! By default, bitwise operator traits (BitAnd, BitOr, etc.) are NOT implemented.
//! This prevents accidental implicit coercion from floating-point to i32.
//!
//! To enable operator syntax (`a & b`), use the `js_bitwise` feature.
//!
//! Explicit methods are always available regardless of feature flags.

use crate::Number;

// ============================================================================
// Explicit methods - always available
// ============================================================================

impl Number {
    /// Bitwise AND with explicit i32 conversion.
    ///
    /// Converts both operands to i32 (truncating, wrapping) before the operation.
    pub fn bitand_i32(&self, rhs: &Number) -> Number {
        let a = self.to_i32_js_coerce();
        let b = rhs.to_i32_js_coerce();
        Number::from(a & b)
    }

    /// Bitwise OR with explicit i32 conversion.
    pub fn bitor_i32(&self, rhs: &Number) -> Number {
        let a = self.to_i32_js_coerce();
        let b = rhs.to_i32_js_coerce();
        Number::from(a | b)
    }

    /// Bitwise XOR with explicit i32 conversion.
    pub fn bitxor_i32(&self, rhs: &Number) -> Number {
        let a = self.to_i32_js_coerce();
        let b = rhs.to_i32_js_coerce();
        Number::from(a ^ b)
    }

    /// Bitwise NOT with explicit i32 conversion.
    pub fn bitnot_i32(&self) -> Number {
        let a = self.to_i32_js_coerce();
        Number::from(!a)
    }

    /// Left shift with explicit i32 conversion.
    ///
    /// Shift amount is masked to 5 bits (0-31) per JavaScript semantics.
    pub fn shl_i32(&self, rhs: &Number) -> Number {
        let a = self.to_i32_js_coerce();
        let b = rhs.to_u32_js_coerce() & 0x1f;
        Number::from(a << b)
    }

    /// Right shift (signed) with explicit i32 conversion.
    ///
    /// Shift amount is masked to 5 bits (0-31) per JavaScript semantics.
    pub fn shr_i32(&self, rhs: &Number) -> Number {
        let a = self.to_i32_js_coerce();
        let b = rhs.to_u32_js_coerce() & 0x1f;
        Number::from(a >> b)
    }
}

// ============================================================================
// Operator trait impls - only with js_bitwise feature
// ============================================================================

#[cfg(feature = "js_bitwise")]
mod trait_impls {
    use super::*;
    use crate::forward_ref_binop;
    use std::ops::{BitAnd, BitOr, BitXor, Not, Shl, Shr};

    impl BitAnd for Number {
        type Output = Number;
        fn bitand(self, rhs: Number) -> Number {
            self.bitand_i32(&rhs)
        }
    }

    impl BitOr for Number {
        type Output = Number;
        fn bitor(self, rhs: Number) -> Number {
            self.bitor_i32(&rhs)
        }
    }

    impl BitXor for Number {
        type Output = Number;
        fn bitxor(self, rhs: Number) -> Number {
            self.bitxor_i32(&rhs)
        }
    }

    impl Not for Number {
        type Output = Number;
        fn not(self) -> Number {
            self.bitnot_i32()
        }
    }

    impl Shl<Number> for Number {
        type Output = Number;
        fn shl(self, rhs: Number) -> Number {
            self.shl_i32(&rhs)
        }
    }

    impl Shr<Number> for Number {
        type Output = Number;
        fn shr(self, rhs: Number) -> Number {
            self.shr_i32(&rhs)
        }
    }

    // Generate reference variants for bitwise operators
    forward_ref_binop!(impl BitAnd, bitand for Number);
    forward_ref_binop!(impl BitOr, bitor for Number);
    forward_ref_binop!(impl BitXor, bitxor for Number);
    forward_ref_binop!(impl Shl, shl for Number);
    forward_ref_binop!(impl Shr, shr for Number);
}
