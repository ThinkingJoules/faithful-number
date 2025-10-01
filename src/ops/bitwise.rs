use crate::{Number, forward_ref_binop};
use std::ops::{BitAnd, BitOr, BitXor, Not, Shl, Shr};
// Bitwise operators (convert to i32 like JS)
impl BitAnd for Number {
    type Output = Number;
    fn bitand(self, rhs: Number) -> Number {
        // JavaScript converts numbers to 32-bit signed integers for bitwise operations
        // NegativeZero is handled in to_i32() which returns 0
        let a = self.to_i32_js_coerce();
        let b = rhs.to_i32_js_coerce();
        Number::from(a & b)
    }
}

impl BitOr for Number {
    type Output = Number;
    fn bitor(self, rhs: Number) -> Number {
        let a = self.to_i32_js_coerce();
        let b = rhs.to_i32_js_coerce();
        Number::from(a | b)
    }
}

impl BitXor for Number {
    type Output = Number;
    fn bitxor(self, rhs: Number) -> Number {
        let a = self.to_i32_js_coerce();
        let b = rhs.to_i32_js_coerce();
        Number::from(a ^ b)
    }
}

impl Not for Number {
    type Output = Number;
    fn not(self) -> Number {
        let a = self.to_i32_js_coerce();
        Number::from(!a)
    }
}

impl Shl<Number> for Number {
    type Output = Number;
    fn shl(self, rhs: Number) -> Number {
        let a = self.to_i32_js_coerce();
        let b = rhs.to_u32_js_coerce() & 0x1f; // JavaScript masks to 5 bits
        Number::from(a << b)
    }
}

impl Shr<Number> for Number {
    type Output = Number;
    fn shr(self, rhs: Number) -> Number {
        let a = self.to_i32_js_coerce();
        let b = rhs.to_u32_js_coerce() & 0x1f; // JavaScript masks to 5 bits
        Number::from(a >> b)
    }
}

// Generate reference variants for bitwise operators
forward_ref_binop!(impl BitAnd, bitand for Number);
forward_ref_binop!(impl BitOr, bitor for Number);
forward_ref_binop!(impl BitXor, bitxor for Number);
forward_ref_binop!(impl Shl, shl for Number);
forward_ref_binop!(impl Shr, shr for Number);
