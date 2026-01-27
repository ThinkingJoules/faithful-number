use crate::Number;
use std::ops::{AddAssign, DivAssign, MulAssign, RemAssign, SubAssign};

// Arithmetic assignment operators - always available
impl AddAssign for Number {
    fn add_assign(&mut self, rhs: Number) {
        *self = self.clone() + rhs;
    }
}

impl SubAssign for Number {
    fn sub_assign(&mut self, rhs: Number) {
        *self = self.clone() - rhs;
    }
}

impl MulAssign for Number {
    fn mul_assign(&mut self, rhs: Number) {
        *self = self.clone() * rhs;
    }
}

impl DivAssign for Number {
    fn div_assign(&mut self, rhs: Number) {
        *self = self.clone() / rhs;
    }
}

impl RemAssign for Number {
    fn rem_assign(&mut self, rhs: Number) {
        *self = self.clone() % rhs;
    }
}

// Bitwise assignment operators - only with js_bitwise feature
#[cfg(feature = "js_bitwise")]
mod bitwise_assign {
    use super::*;
    use std::ops::{BitAndAssign, BitOrAssign, BitXorAssign, ShlAssign, ShrAssign};

    impl BitAndAssign for Number {
        fn bitand_assign(&mut self, rhs: Number) {
            *self = self.bitand_i32(&rhs);
        }
    }

    impl BitOrAssign for Number {
        fn bitor_assign(&mut self, rhs: Number) {
            *self = self.bitor_i32(&rhs);
        }
    }

    impl BitXorAssign for Number {
        fn bitxor_assign(&mut self, rhs: Number) {
            *self = self.bitxor_i32(&rhs);
        }
    }

    impl ShlAssign<Number> for Number {
        fn shl_assign(&mut self, rhs: Number) {
            *self = self.shl_i32(&rhs);
        }
    }

    impl ShrAssign<Number> for Number {
        fn shr_assign(&mut self, rhs: Number) {
            *self = self.shr_i32(&rhs);
        }
    }
}
