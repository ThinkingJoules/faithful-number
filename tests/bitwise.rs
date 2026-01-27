//! Adversarial tests for bitwise operations.

use faithful_number::Number;

#[test]
fn bitwise_explicit_methods() {
    let a = Number::from(12); // 0b1100
    let b = Number::from(5); // 0b0101

    assert_eq!(a.bitand_i32(&b), Number::from(4)); // 0b0100
    assert_eq!(a.bitor_i32(&b), Number::from(13)); // 0b1101
    assert_eq!(a.bitxor_i32(&b), Number::from(9)); // 0b1001
    assert_eq!(a.bitnot_i32(), Number::from(-13)); // ~12 = -13
}

#[test]
fn shift_by_32_wraps() {
    // JS masks shift amount to 5 bits: 1 << 32 == 1 << 0 == 1
    let one = Number::from(1);
    let thirty_two = Number::from(32);

    assert_eq!(one.shl_i32(&thirty_two), Number::from(1));
}

#[test]
fn shift_by_33_wraps() {
    // 1 << 33 == 1 << 1 == 2
    let one = Number::from(1);
    let thirty_three = Number::from(33);

    assert_eq!(one.shl_i32(&thirty_three), Number::from(2));
}

#[test]
fn shift_by_negative() {
    // Negative shift amounts are treated as unsigned (masked)
    // -1 as u32 is 0xFFFFFFFF, masked to 5 bits is 31
    let one = Number::from(1);
    let neg_one = Number::from(-1);

    // 1 << 31 = 0x80000000 = -2147483648 as i32
    assert_eq!(one.shl_i32(&neg_one), Number::from(-2147483648i32));
}

#[test]
fn bitwise_with_infinity() {
    let inf = Number::infinity();
    let five = Number::from(5);

    // Infinity coerces to 0 in bitwise operations
    assert_eq!(inf.bitand_i32(&five), Number::from(0));
    assert_eq!(five.bitand_i32(&inf), Number::from(0));
}

#[test]
fn bitwise_with_nan() {
    let nan = Number::nan();
    let five = Number::from(5);

    // NaN coerces to 0 in bitwise operations
    assert_eq!(nan.bitor_i32(&five), Number::from(5)); // 0 | 5 = 5
    assert_eq!(five.bitor_i32(&nan), Number::from(5));
}

#[test]
fn bitwise_with_decimal() {
    // Decimals should truncate to integer
    let decimal = Number::from(12.7);
    let integer = Number::from(5);

    assert_eq!(decimal.bitand_i32(&integer), Number::from(4)); // 12 & 5 = 4
}

#[test]
fn bitwise_with_negative_decimal() {
    // -12.7 truncates to -12
    let neg_decimal = Number::from(-12.7);
    let integer = Number::from(5);

    // -12 in two's complement & 5
    assert_eq!(neg_decimal.bitand_i32(&integer), Number::from(4)); // -12 & 5 = 4
}

#[test]
fn bitwise_large_values_wrap() {
    // Values larger than i32 should wrap
    let large = Number::from(0x1_0000_0005i64); // > i32::MAX
    let mask = Number::from(0xFF);

    // Should wrap to just the lower 32 bits
    assert_eq!(large.bitand_i32(&mask), Number::from(5));
}
