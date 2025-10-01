use crate::Number;
use std::ops::{
    AddAssign, BitAndAssign, BitOrAssign, BitXorAssign, DivAssign, MulAssign, RemAssign, ShlAssign,
    ShrAssign, SubAssign,
};

// Assignment operators
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

// Bitwise assignment operators
impl BitAndAssign for Number {
    fn bitand_assign(&mut self, rhs: Number) {
        *self = self.clone() & rhs;
    }
}

impl BitOrAssign for Number {
    fn bitor_assign(&mut self, rhs: Number) {
        *self = self.clone() | rhs;
    }
}

impl BitXorAssign for Number {
    fn bitxor_assign(&mut self, rhs: Number) {
        *self = self.clone() ^ rhs;
    }
}

impl ShlAssign<Number> for Number {
    fn shl_assign(&mut self, rhs: Number) {
        *self = self.clone() << rhs;
    }
}

impl ShrAssign<Number> for Number {
    fn shr_assign(&mut self, rhs: Number) {
        *self = self.clone() >> rhs;
    }
}
