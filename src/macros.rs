// Macro definitions for the library
// Contains: js_dec!, impl_from_primitives!, forward_ref_binop!

// Macro to generate reference variants for binary operators
#[macro_export]
macro_rules! forward_ref_binop {
    (impl $trait:ident, $method:ident for $type:ty) => {
        impl $trait<&$type> for $type {
            type Output = $type;
            fn $method(self, rhs: &$type) -> $type {
                $trait::$method(self, rhs.clone())
            }
        }

        impl $trait<$type> for &$type {
            type Output = $type;
            fn $method(self, rhs: $type) -> $type {
                $trait::$method(self.clone(), rhs)
            }
        }

        impl $trait<&$type> for &$type {
            type Output = $type;
            fn $method(self, rhs: &$type) -> $type {
                $trait::$method(self.clone(), rhs.clone())
            }
        }
    };
}

// Macro to generate From implementations for primitive types
// These convert to Decimal by default (could be optimized to use Rational for small integers)
#[macro_export]
macro_rules! impl_from_primitives {
    ($($t:ty),*) => {
        $(
            impl From<$t> for Number {
                fn from(n: $t) -> Number {
                    Number::from_decimal(Decimal::from(n))
                }
            }
        )*
    };
}

#[macro_export]
macro_rules! impl_from_primitives_inner {
    ($($t:ty),*) => {
        $(
            impl From<$t> for NumericValue {
                fn from(n: $t) -> NumericValue {
                    NumericValue::Decimal(Decimal::from(n))
                }
            }
        )*
    };
}
// Convenience macro for creating Number literals
#[macro_export]
macro_rules! num {
    (NaN) => {
        Number::NAN
    };
    (Infinity) => {
        Number::POSITIVE_INFINITY
    };
    (-Infinity) => {
        Number::NEGATIVE_INFINITY
    };
    (-0) => {
        Number::NEGATIVE_ZERO
    };
    ($n:expr) => {
        Number::from($n)
    };
}
