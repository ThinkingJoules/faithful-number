//! Precision control for high-precision arithmetic operations.
//!
//! When the `high_precision` feature is enabled, this module provides
//! thread-local precision configuration for transcendental operations.

#[cfg(feature = "high_precision")]
use std::cell::RefCell;

#[cfg(feature = "high_precision")]
thread_local! {
    /// Default precision in bits for high-precision operations.
    /// Default is 256 bits (~71 decimal digits).
    static PRECISION: RefCell<u32> = RefCell::new(256);
}

/// Set the default precision for high-precision transcendental operations.
///
/// This setting is thread-local and affects all subsequent transcendental
/// operations (sin, cos, log, exp, etc.) in the current thread.
///
/// # Arguments
/// * `bits` - Precision in bits. Higher values give more accurate results but are slower.
///   Recommended: 100-200 bits for most applications, 300+ for high-precision work.
///
/// # Example
/// ```
/// use faithful_number::Number;
///
/// #[cfg(feature = "high_precision")]
/// {
///     // Set precision to 200 bits (~60 decimal digits)
///     Number::set_default_precision(200);
///
///     // All transcendental operations now use 200-bit precision
///     let result = Number::from(2).sqrt();
///     println!("{}", result.to_f64());
/// }
/// ```
#[cfg(feature = "high_precision")]
pub fn set_default_precision(bits: u32) {
    PRECISION.with(|p| *p.borrow_mut() = bits);
}

/// Get the current default precision in bits.
///
/// # Returns
/// The current precision setting for the current thread.
#[cfg(feature = "high_precision")]
pub fn get_default_precision() -> u32 {
    PRECISION.with(|p| *p.borrow())
}

/// No-op version when high_precision feature is disabled.
#[cfg(not(feature = "high_precision"))]
pub fn set_default_precision(_bits: u32) {
    // No-op: precision control only available with high_precision feature
}

/// Returns 0 when high_precision feature is disabled (uses f64).
#[cfg(not(feature = "high_precision"))]
pub fn get_default_precision() -> u32 {
    0 // Indicates f64 precision
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "high_precision")]
    fn test_precision_control() {
        let default = get_default_precision();
        assert_eq!(default, 100);

        set_default_precision(200);
        assert_eq!(get_default_precision(), 200);

        // Restore default
        set_default_precision(100);
    }

    #[test]
    #[cfg(not(feature = "high_precision"))]
    fn test_precision_control_disabled() {
        assert_eq!(get_default_precision(), 0);
        set_default_precision(200); // Should be no-op
        assert_eq!(get_default_precision(), 0);
    }
}
