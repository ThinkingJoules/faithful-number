# Number Representation Test Results

## Executive Summary

Tested 10 number types across 7 operations with 430 total test cases.

## Accuracy Summary

Breakdown of exact vs approximate results for each type:

| Type | Exact | Approximate | No Result/Skipped | Total Error | Total Time (µs) |
|------|-------|-------------|-------------------|-------------|----------------|
| FaithfulNumber | 40 | 3 | 0 | 7.17e-78 | 12.85 |
| BigRational | 36 | 2 | 5 | 4.88e-17 | 18.23 |
| RugFloat | 36 | 7 | 0 | 7.60e-152 | 15.14 |
| JSDecimal | 33 | 9 | 1 | NaN | 3.03 |
| BigDecimal | 33 | 4 | 6 | 3.75e-100 | 20.76 |
| BigInt | 29 | 6 | 8 | 1.00e15 | 1.13 |
| F64 | 28 | 15 | 0 | 6.99e34 | 0.46 |
| Decimal | 28 | 5 | 10 | 2.00e50 | 0.78 |
| Rational64 | 25 | 0 | 18 | 0.00e0 | 0.57 |
| I64 | 19 | 6 | 18 | 1.00e15 | 0.26 |


## Test Coverage Summary

All types run the same 43 test cases (7 operations × varying inputs). Results differ based on type capabilities:

| Type | Total Tests | Succeeded | Failed | Skipped |
|------|-------------|-----------|--------|---------|
| F64 | 43 | 43 | 0 | 0 |
| RugFloat | 43 | 43 | 0 | 0 |
| FaithfulNumber | 43 | 43 | 0 | 0 |
| JSDecimal | 43 | 42 | 1 | 0 |
| BigRational | 43 | 38 | 5 | 0 |
| BigDecimal | 43 | 37 | 6 | 0 |
| BigInt | 43 | 35 | 8 | 0 |
| Decimal | 43 | 33 | 10 | 0 |
| I64 | 43 | 25 | 10 | 8 |
| Rational64 | 43 | 25 | 10 | 8 |


## Test Case Analysis

Detailed analysis of each test case, showing performance and accuracy trade-offs.

### Test: `Add::decimal_precision`

**Operation:** `0.1 + 0.2 = 0.3`

**Expected:** `0.3` (FiniteDecimal)

**Category:** PrecisionCritical

#### Results by Accuracy

| Type           | Result                | Error     | ns/op  | Relative Speed | Status |
|----------------|-----------------------|-----------|--------|----------------|--------|
| Rational64     | `3/10`                | ✓ Exact   | 19.32  | 1.88x          | ✓      |
| Decimal        | `0.3`                 | ✓ Exact   | 22.19  | 2.16x          | ✓      |
| JSDecimal      | `0.3`                 | ✓ Exact   | 30.70  | 2.99x          | ✓      |
| FaithfulNumber | `0.3`                 | ✓ Exact   | 41.92  | 4.08x          | ✓      |
| BigDecimal     | `0.3`                 | ✓ Exact   | 57.98  | 5.64x          | ✓      |
| BigRational    | `3/10`                | ✓ Exact   | 348.47 | 33.90x         | ✓      |
| RugFloat       | `3.000000...00015e-1` | 5.00e-155 | 61.62  | 6.00x          | ≈      |
| F64            | `0.300000...00000004` | 1.33e-16  | 10.43  | 1.01x          | ≈      |
| I64            | `0`                   | 1.00e0    | 10.28  | 1.00x          | ≈      |
| BigInt         | `0`                   | 1.00e0    | 14.99  | 1.46x          | ≈      |

### Test: `Add::extreme_1e50`

**Operation:** `100000000000000000000000000000000000000000000000000 + 100000000000000000000000000000000000000000000000000 = 200000000000000000000000000000000000000000000000000`

**Expected:** `200000000000000000000000000000000000000000000000000` (ExactInteger)

**Category:** Boundary

#### Results by Accuracy

| Type           | Result                | Error     | ns/op  | Relative Speed | Status      |
|----------------|-----------------------|-----------|--------|----------------|-------------|
| BigInt         | `20000000...00000000` | ✓ Exact   | 28.76  | 2.81x          | ✓           |
| RugFloat       | `20000000...00000000` | ✓ Exact   | 59.19  | 5.79x          | ✓           |
| BigDecimal     | `20000000...00000000` | ✓ Exact   | 61.46  | 6.01x          | ✓           |
| FaithfulNumber | `20000000...00000000` | ✓ Exact   | 67.18  | 6.57x          | ✓           |
| BigRational    | `20000000...00000000` | ✓ Exact   | 839.36 | 82.13x         | ✓           |
| F64            | `20000000...00000000` | 7.63e-17  | 10.22  | 1.00x          | ≈           |
| JSDecimal      | `NaN`                 | NaN       | 17.44  | 1.71x          | ≈           |
| I64            | N/A                   | -         | -      | -              | ⚠ Skipped   |
| Decimal        | N/A                   | -         | -      | -              | ❌ Failed    |
| Rational64     | N/A                   | -         | -      | -              | ⚠ Skipped   |

### Test: `Add::f64_precision_limit`

**Operation:** `10000000000000000 + 1 = 10000000000000001`

**Expected:** `10000000000000001` (ExactInteger)

**Category:** PrecisionCritical

#### Results by Accuracy

| Type           | Result                | Error     | ns/op  | Relative Speed | Status |
|----------------|-----------------------|-----------|--------|----------------|--------|
| I64            | `10000000...00000001` | ✓ Exact   | 10.32  | 1.01x          | ✓      |
| Decimal        | `10000000...00000001` | ✓ Exact   | 23.41  | 2.28x          | ✓      |
| BigInt         | `10000000...00000001` | ✓ Exact   | 28.51  | 2.78x          | ✓      |
| Rational64     | `10000000...00000001` | ✓ Exact   | 29.20  | 2.85x          | ✓      |
| JSDecimal      | `10000000...00000001` | ✓ Exact   | 33.18  | 3.23x          | ✓      |
| RugFloat       | `10000000...00000001` | ✓ Exact   | 55.33  | 5.39x          | ✓      |
| FaithfulNumber | `10000000...00000001` | ✓ Exact   | 57.40  | 5.59x          | ✓      |
| BigDecimal     | `10000000...00000001` | ✓ Exact   | 58.00  | 5.65x          | ✓      |
| BigRational    | `10000000...00000001` | ✓ Exact   | 404.44 | 39.42x         | ✓      |
| F64            | `10000000...00000000` | 1.00e-16  | 10.26  | 1.00x          | ≈      |

### Test: `Add::large_1e15`

**Operation:** `1000000000000000 + 1000000000000000 = 2000000000000000`

**Expected:** `2000000000000000` (ExactInteger)

**Category:** Boundary

#### Results by Accuracy

| Type           | Result             | Error     | ns/op  | Relative Speed | Status |
|----------------|--------------------|-----------|--------|----------------|--------|
| I64            | `2000000000000000` | ✓ Exact   | 10.34  | 1.00x          | ✓      |
| F64            | `2000000000000000` | ✓ Exact   | 10.55  | 1.02x          | ✓      |
| Decimal        | `2000000000000000` | ✓ Exact   | 23.28  | 2.25x          | ✓      |
| Rational64     | `2000000000000000` | ✓ Exact   | 29.19  | 2.82x          | ✓      |
| BigInt         | `2000000000000000` | ✓ Exact   | 31.47  | 3.05x          | ✓      |
| JSDecimal      | `2000000000000000` | ✓ Exact   | 32.94  | 3.19x          | ✓      |
| FaithfulNumber | `2000000000000000` | ✓ Exact   | 56.29  | 5.45x          | ✓      |
| BigDecimal     | `2000000000000000` | ✓ Exact   | 58.23  | 5.63x          | ✓      |
| RugFloat       | `2000000000000000` | ✓ Exact   | 58.77  | 5.69x          | ✓      |
| BigRational    | `2000000000000000` | ✓ Exact   | 391.88 | 37.92x         | ✓      |

### Test: `Add::medium_1e6`

**Operation:** `1000000 + 1000000 = 2000000`

**Expected:** `2000000` (ExactInteger)

**Category:** Normal

#### Results by Accuracy

| Type           | Result    | Error     | ns/op  | Relative Speed | Status |
|----------------|-----------|-----------|--------|----------------|--------|
| F64            | `2000000` | ✓ Exact   | 10.28  | 1.00x          | ✓      |
| I64            | `2000000` | ✓ Exact   | 10.34  | 1.01x          | ✓      |
| Rational64     | `2000000` | ✓ Exact   | 20.66  | 2.01x          | ✓      |
| Decimal        | `2000000` | ✓ Exact   | 22.40  | 2.18x          | ✓      |
| FaithfulNumber | `2000000` | ✓ Exact   | 27.04  | 2.63x          | ✓      |
| BigInt         | `2000000` | ✓ Exact   | 28.95  | 2.82x          | ✓      |
| JSDecimal      | `2000000` | ✓ Exact   | 31.43  | 3.06x          | ✓      |
| BigDecimal     | `2000000` | ✓ Exact   | 58.19  | 5.66x          | ✓      |
| RugFloat       | `2000000` | ✓ Exact   | 58.97  | 5.74x          | ✓      |
| BigRational    | `2000000` | ✓ Exact   | 234.81 | 22.85x         | ✓      |

### Test: `Add::medium_1e9`

**Operation:** `1000000000 + 1000000000 = 2000000000`

**Expected:** `2000000000` (ExactInteger)

**Category:** Boundary

#### Results by Accuracy

| Type           | Result       | Error     | ns/op  | Relative Speed | Status |
|----------------|--------------|-----------|--------|----------------|--------|
| F64            | `2000000000` | ✓ Exact   | 10.26  | 1.00x          | ✓      |
| I64            | `2000000000` | ✓ Exact   | 10.33  | 1.01x          | ✓      |
| Decimal        | `2000000000` | ✓ Exact   | 22.24  | 2.17x          | ✓      |
| Rational64     | `2000000000` | ✓ Exact   | 23.46  | 2.29x          | ✓      |
| BigInt         | `2000000000` | ✓ Exact   | 29.27  | 2.85x          | ✓      |
| JSDecimal      | `2000000000` | ✓ Exact   | 30.70  | 2.99x          | ✓      |
| FaithfulNumber | `2000000000` | ✓ Exact   | 49.62  | 4.84x          | ✓      |
| BigDecimal     | `2000000000` | ✓ Exact   | 58.20  | 5.67x          | ✓      |
| RugFloat       | `2000000000` | ✓ Exact   | 58.96  | 5.75x          | ✓      |
| BigRational    | `2000000000` | ✓ Exact   | 308.49 | 30.07x         | ✓      |

### Test: `Add::near_i64_max`

**Operation:** `9223372036854775000 + 1000 = 9223372036854776000`

**Expected:** `9223372036854776000` (ExactInteger)

**Category:** Boundary

#### Results by Accuracy

| Type           | Result                | Error     | ns/op  | Relative Speed | Status     |
|----------------|-----------------------|-----------|--------|----------------|------------|
| Decimal        | `92233720...54776000` | ✓ Exact   | 23.34  | 2.26x          | ✓          |
| BigInt         | `92233720...54776000` | ✓ Exact   | 28.47  | 2.76x          | ✓          |
| JSDecimal      | `92233720...54776000` | ✓ Exact   | 32.99  | 3.20x          | ✓          |
| FaithfulNumber | `92233720...54776000` | ✓ Exact   | 55.95  | 5.43x          | ✓          |
| BigDecimal     | `92233720...54776000` | ✓ Exact   | 57.86  | 5.61x          | ✓          |
| RugFloat       | `92233720...54776000` | ✓ Exact   | 61.16  | 5.93x          | ✓          |
| BigRational    | `92233720...54776000` | ✓ Exact   | 188.75 | 18.31x         | ✓          |
| F64            | `92233720...54776000` | 2.08e-17  | 10.31  | 1.00x          | ≈          |
| I64            | N/A                   | -         | -      | -              | ❌ Failed   |
| Rational64     | N/A                   | -         | -      | -              | ❌ Failed   |

### Test: `Add::small_positive`

**Operation:** `1 + 2 = 3`

**Expected:** `3` (ExactInteger)

**Category:** Normal

#### Results by Accuracy

| Type           | Result | Error     | ns/op  | Relative Speed | Status |
|----------------|--------|-----------|--------|----------------|--------|
| F64            | `3`    | ✓ Exact   | 10.23  | 1.00x          | ✓      |
| I64            | `3`    | ✓ Exact   | 10.38  | 1.01x          | ✓      |
| Rational64     | `3`    | ✓ Exact   | 19.43  | 1.90x          | ✓      |
| Decimal        | `3`    | ✓ Exact   | 22.33  | 2.18x          | ✓      |
| FaithfulNumber | `3`    | ✓ Exact   | 27.02  | 2.64x          | ✓      |
| BigInt         | `3`    | ✓ Exact   | 28.78  | 2.81x          | ✓      |
| JSDecimal      | `3`    | ✓ Exact   | 30.64  | 3.00x          | ✓      |
| RugFloat       | `3`    | ✓ Exact   | 54.17  | 5.29x          | ✓      |
| BigDecimal     | `3`    | ✓ Exact   | 57.42  | 5.61x          | ✓      |
| BigRational    | `3`    | ✓ Exact   | 182.67 | 17.85x         | ✓      |

### Test: `Add::very_large_1e18`

**Operation:** `1000000000000000000 + 1000000000000000000 = 2000000000000000000`

**Expected:** `2000000000000000000` (ExactInteger)

**Category:** Boundary

#### Results by Accuracy

| Type           | Result                | Error     | ns/op  | Relative Speed | Status |
|----------------|-----------------------|-----------|--------|----------------|--------|
| I64            | `20000000...00000000` | ✓ Exact   | 10.19  | 1.00x          | ✓      |
| F64            | `20000000...00000000` | ✓ Exact   | 10.21  | 1.00x          | ✓      |
| Decimal        | `20000000...00000000` | ✓ Exact   | 23.27  | 2.28x          | ✓      |
| BigInt         | `20000000...00000000` | ✓ Exact   | 28.76  | 2.82x          | ✓      |
| FaithfulNumber | `20000000...00000000` | ✓ Exact   | 30.39  | 2.98x          | ✓      |
| Rational64     | `20000000...00000000` | ✓ Exact   | 32.01  | 3.14x          | ✓      |
| JSDecimal      | `20000000...00000000` | ✓ Exact   | 32.94  | 3.23x          | ✓      |
| BigDecimal     | `20000000...00000000` | ✓ Exact   | 58.25  | 5.72x          | ✓      |
| RugFloat       | `20000000...00000000` | ✓ Exact   | 58.98  | 5.79x          | ✓      |
| BigRational    | `20000000...00000000` | ✓ Exact   | 435.00 | 42.69x         | ✓      |

### Test: `Add::very_large_1e20`

**Operation:** `100000000000000000000 + 100000000000000000000 = 200000000000000000000`

**Expected:** `200000000000000000000` (ExactInteger)

**Category:** Boundary

#### Results by Accuracy

| Type           | Result                | Error     | ns/op  | Relative Speed | Status      |
|----------------|-----------------------|-----------|--------|----------------|-------------|
| F64            | `20000000...00000000` | ✓ Exact   | 10.38  | 1.00x          | ✓           |
| Decimal        | `20000000...00000000` | ✓ Exact   | 23.12  | 2.23x          | ✓           |
| BigInt         | `20000000...00000000` | ✓ Exact   | 28.66  | 2.76x          | ✓           |
| FaithfulNumber | `20000000...00000000` | ✓ Exact   | 30.22  | 2.91x          | ✓           |
| JSDecimal      | `20000000...00000000` | ✓ Exact   | 33.00  | 3.18x          | ✓           |
| BigDecimal     | `20000000...00000000` | ✓ Exact   | 58.80  | 5.66x          | ✓           |
| RugFloat       | `20000000...00000000` | ✓ Exact   | 60.03  | 5.78x          | ✓           |
| BigRational    | `20000000...00000000` | ✓ Exact   | 476.90 | 45.94x         | ✓           |
| I64            | N/A                   | -         | -      | -              | ⚠ Skipped   |
| Rational64     | N/A                   | -         | -      | -              | ⚠ Skipped   |

### Test: `Div::basic_div`

**Operation:** `12 ÷ 4 = 3`

**Expected:** `3` (ExactInteger)

**Category:** Normal

#### Results by Accuracy

| Type           | Result | Error     | ns/op  | Relative Speed | Status |
|----------------|--------|-----------|--------|----------------|--------|
| F64            | `3`    | ✓ Exact   | 10.19  | 1.00x          | ✓      |
| I64            | `3`    | ✓ Exact   | 10.52  | 1.03x          | ✓      |
| Rational64     | `3`    | ✓ Exact   | 11.74  | 1.15x          | ✓      |
| Decimal        | `3`    | ✓ Exact   | 18.34  | 1.80x          | ✓      |
| BigInt         | `3`    | ✓ Exact   | 26.30  | 2.58x          | ✓      |
| JSDecimal      | `3`    | ✓ Exact   | 33.64  | 3.30x          | ✓      |
| FaithfulNumber | `3`    | ✓ Exact   | 34.17  | 3.35x          | ✓      |
| BigDecimal     | `3`    | ✓ Exact   | 63.79  | 6.26x          | ✓      |
| RugFloat       | `3`    | ✓ Exact   | 271.69 | 26.66x         | ✓      |
| BigRational    | `3`    | ✓ Exact   | 400.63 | 39.31x         | ✓      |

### Test: `Div::large_div_large`

**Operation:** `100000000000000000000 ÷ 10000000000 = 10000000000`

**Expected:** `10000000000` (ExactInteger)

**Category:** Boundary

#### Results by Accuracy

| Type           | Result        | Error     | ns/op  | Relative Speed | Status      |
|----------------|---------------|-----------|--------|----------------|-------------|
| F64            | `10000000000` | ✓ Exact   | 10.33  | 1.00x          | ✓           |
| Decimal        | `10000000000` | ✓ Exact   | 20.53  | 1.99x          | ✓           |
| BigInt         | `10000000000` | ✓ Exact   | 34.12  | 3.30x          | ✓           |
| JSDecimal      | `10000000000` | ✓ Exact   | 40.34  | 3.90x          | ✓           |
| BigDecimal     | `10000000000` | ✓ Exact   | 64.71  | 6.26x          | ✓           |
| FaithfulNumber | `10000000000` | ✓ Exact   | 83.62  | 8.09x          | ✓           |
| RugFloat       | `10000000000` | ✓ Exact   | 265.83 | 25.73x         | ✓           |
| BigRational    | `10000000000` | ✓ Exact   | 626.03 | 60.59x         | ✓           |
| I64            | N/A           | -         | -      | -              | ⚠ Skipped   |
| Rational64     | N/A           | -         | -      | -              | ⚠ Skipped   |

### Test: `Div::large_div_small`

**Operation:** `1000000000000000000 ÷ 1000000 = 1000000000000`

**Expected:** `1000000000000` (ExactInteger)

**Category:** Normal

#### Results by Accuracy

| Type           | Result          | Error     | ns/op  | Relative Speed | Status |
|----------------|-----------------|-----------|--------|----------------|--------|
| I64            | `1000000000000` | ✓ Exact   | 10.27  | 1.00x          | ✓      |
| F64            | `1000000000000` | ✓ Exact   | 10.44  | 1.02x          | ✓      |
| Rational64     | `1000000000000` | ✓ Exact   | 14.49  | 1.41x          | ✓      |
| Decimal        | `1000000000000` | ✓ Exact   | 18.55  | 1.81x          | ✓      |
| BigInt         | `1000000000000` | ✓ Exact   | 26.07  | 2.54x          | ✓      |
| JSDecimal      | `1000000000000` | ✓ Exact   | 34.51  | 3.36x          | ✓      |
| BigDecimal     | `1000000000000` | ✓ Exact   | 64.91  | 6.32x          | ✓      |
| FaithfulNumber | `1000000000000` | ✓ Exact   | 75.98  | 7.40x          | ✓      |
| RugFloat       | `1000000000000` | ✓ Exact   | 257.00 | 25.02x         | ✓      |
| BigRational    | `1000000000000` | ✓ Exact   | 666.69 | 64.90x         | ✓      |

### Test: `Div::one_seventh`

**Operation:** `1 ÷ 7 = 1/7`

**Expected:** `1/7` (RepeatingRational)

**Category:** PrecisionCritical

#### Results by Accuracy

| Type           | Result                | Error     | ns/op   | Relative Speed | Status |
|----------------|-----------------------|-----------|---------|----------------|--------|
| Rational64     | `1/7`                 | ✓ Exact   | 11.91   | 1.16x          | ✓      |
| FaithfulNumber | `0.142857...28571429` | ✓ Exact   | 35.01   | 3.40x          | ✓      |
| BigRational    | `1/7`                 | ✓ Exact   | 415.67  | 40.36x         | ✓      |
| RugFloat       | `1.428571...42852e-1` | 3.73e-155 | 141.04  | 13.69x         | ≈      |
| BigDecimal     | `0.142857...28571429` | 3.00e-100 | 4789.27 | 464.97x        | ≈      |
| Decimal        | `0.142857...28571429` | 3.00e-28  | 31.26   | 3.04x          | ≈      |
| JSDecimal      | `0.142857...28571429` | 3.00e-28  | 52.35   | 5.08x          | ≈      |
| F64            | `0.142857...85714285` | 5.55e-17  | 10.30   | 1.00x          | ≈      |
| I64            | `0`                   | 1.00e0    | 10.41   | 1.01x          | ≈      |
| BigInt         | `0`                   | 1.00e0    | 45.33   | 4.40x          | ≈      |

### Test: `Div::one_third`

**Operation:** `1 ÷ 3 = 1/3`

**Expected:** `1/3` (RepeatingRational)

**Category:** PrecisionCritical

#### Results by Accuracy

| Type           | Result                | Error     | ns/op   | Relative Speed | Status |
|----------------|-----------------------|-----------|---------|----------------|--------|
| Rational64     | `1/3`                 | ✓ Exact   | 11.69   | 1.14x          | ✓      |
| FaithfulNumber | `0.333333...33333333` | ✓ Exact   | 35.45   | 3.47x          | ✓      |
| BigRational    | `1/3`                 | ✓ Exact   | 394.81  | 38.66x         | ✓      |
| RugFloat       | `3.333333...33346e-1` | 3.73e-155 | 140.62  | 13.77x         | ≈      |
| BigDecimal     | `0.333333...33333333` | 1.00e-100 | 4792.75 | 469.29x        | ≈      |
| Decimal        | `0.333333...33333333` | 1.00e-28  | 30.56   | 2.99x          | ≈      |
| JSDecimal      | `0.333333...33333333` | 1.00e-28  | 52.90   | 5.18x          | ≈      |
| F64            | `0.333333...33333333` | 5.55e-17  | 10.21   | 1.00x          | ≈      |
| I64            | `0`                   | 1.00e0    | 10.41   | 1.02x          | ≈      |
| BigInt         | `0`                   | 1.00e0    | 45.45   | 4.45x          | ≈      |

### Test: `Div::third_times_three`

**Operation:** `3 ÷ 3 = 1`

**Expected:** `1` (ExactInteger)

**Category:** Normal

#### Results by Accuracy

| Type           | Result | Error     | ns/op  | Relative Speed | Status |
|----------------|--------|-----------|--------|----------------|--------|
| F64            | `1`    | ✓ Exact   | 10.21  | 1.00x          | ✓      |
| Rational64     | `1`    | ✓ Exact   | 10.24  | 1.00x          | ✓      |
| I64            | `1`    | ✓ Exact   | 10.29  | 1.01x          | ✓      |
| Decimal        | `1`    | ✓ Exact   | 18.77  | 1.84x          | ✓      |
| BigInt         | `1`    | ✓ Exact   | 27.24  | 2.67x          | ✓      |
| FaithfulNumber | `1`    | ✓ Exact   | 29.84  | 2.92x          | ✓      |
| JSDecimal      | `1`    | ✓ Exact   | 33.46  | 3.28x          | ✓      |
| BigDecimal     | `1`    | ✓ Exact   | 45.94  | 4.50x          | ✓      |
| RugFloat       | `1`    | ✓ Exact   | 273.23 | 26.77x         | ✓      |
| BigRational    | `1`    | ✓ Exact   | 287.34 | 28.15x         | ✓      |

### Test: `Div::two_thirds`

**Operation:** `2 ÷ 3 = 2/3`

**Expected:** `2/3` (RepeatingRational)

**Category:** PrecisionCritical

#### Results by Accuracy

| Type           | Result                | Error     | ns/op   | Relative Speed | Status |
|----------------|-----------------------|-----------|---------|----------------|--------|
| Rational64     | `2/3`                 | ✓ Exact   | 11.68   | 1.14x          | ✓      |
| FaithfulNumber | `0.666666...66666667` | ✓ Exact   | 35.23   | 3.45x          | ✓      |
| BigRational    | `2/3`                 | ✓ Exact   | 396.29  | 38.83x         | ✓      |
| RugFloat       | `6.666666...66692e-1` | 3.73e-155 | 140.81  | 13.80x         | ≈      |
| BigDecimal     | `0.666666...66666667` | 5.00e-101 | 4834.06 | 473.64x        | ≈      |
| Decimal        | `0.666666...66666667` | 5.00e-29  | 31.08   | 3.05x          | ≈      |
| JSDecimal      | `0.666666...66666667` | 5.00e-29  | 52.65   | 5.16x          | ≈      |
| F64            | `0.666666...66666666` | 5.55e-17  | 10.21   | 1.00x          | ≈      |
| I64            | `0`                   | 1.00e0    | 10.43   | 1.02x          | ≈      |
| BigInt         | `0`                   | 1.00e0    | 45.34   | 4.44x          | ≈      |

### Test: `Div::very_large_div`

**Operation:** `100000000000000000000000000000000000000000000000000 ÷ 10000000000000000000000000 = 10000000000000000000000000`

**Expected:** `10000000000000000000000000` (ExactInteger)

**Category:** Boundary

#### Results by Accuracy

| Type           | Result                | Error     | ns/op  | Relative Speed | Status      |
|----------------|-----------------------|-----------|--------|----------------|-------------|
| BigInt         | `10000000...00000000` | ✓ Exact   | 140.20 | 13.66x         | ✓           |
| BigDecimal     | `10000000...00000000` | ✓ Exact   | 149.68 | 14.58x         | ✓           |
| RugFloat       | `10000000...00000000` | ✓ Exact   | 265.99 | 25.91x         | ✓           |
| FaithfulNumber | `10000000...00000000` | ✓ Exact   | 311.15 | 30.31x         | ✓           |
| BigRational    | `10000000...00000000` | ✓ Exact   | 911.30 | 88.77x         | ✓           |
| F64            | `10000000...00000000` | 9.06e-17  | 10.27  | 1.00x          | ≈           |
| JSDecimal      | `NaN`                 | NaN       | 17.37  | 1.69x          | ≈           |
| Decimal        | `7922.816...43950335` | 1.00e0    | 46.38  | 4.52x          | ≈           |
| I64            | N/A                   | -         | -      | -              | ⚠ Skipped   |
| Rational64     | N/A                   | -         | -      | -              | ⚠ Skipped   |

### Test: `Ln::ln_1`

**Operation:** `ln(1) = 0`

**Expected:** `0` (ExactInteger)

**Category:** Normal

#### Results by Accuracy

| Type           | Result | Error     | ns/op  | Relative Speed | Status     |
|----------------|--------|-----------|--------|----------------|------------|
| F64            | `0`    | ✓ Exact   | 11.10  | 1.00x          | ✓          |
| JSDecimal      | `0`    | ✓ Exact   | 29.52  | 2.66x          | ✓          |
| RugFloat       | `0`    | ✓ Exact   | 36.94  | 3.33x          | ✓          |
| FaithfulNumber | `0`    | ✓ Exact   | 311.97 | 28.10x         | ✓          |
| I64            | N/A    | -         | -      | -              | ❌ Failed   |
| Decimal        | N/A    | -         | -      | -              | ❌ Failed   |
| BigInt         | N/A    | -         | -      | -              | ❌ Failed   |
| Rational64     | N/A    | -         | -      | -              | ❌ Failed   |
| BigRational    | N/A    | -         | -      | -              | ❌ Failed   |
| BigDecimal     | N/A    | -         | -      | -              | ❌ Failed   |

### Test: `Ln::ln_2`

**Operation:** `ln(2) = 0.693147180559945309417232121458176568075500134360255254120680009493393621969694715605863326996418687542001481020570685733685520235758130557032670751635`

**Expected:** `0.693147180559945309417232121458176568075500134360255254120680009493393621969694715605863326996418687542001481020570685733685520235758130557032670751635` (Transcendental)

**Category:** Normal

#### Results by Accuracy

| Type           | Result                | Error     | ns/op   | Relative Speed | Status     |
|----------------|-----------------------|-----------|---------|----------------|------------|
| RugFloat       | `6.931471...75944e-1` | 1.10e-151 | 4656.62 | 381.09x        | ≈          |
| FaithfulNumber | `0.693147...19696955` | 1.13e-78  | 4067.20 | 332.86x        | ≈          |
| F64            | `0.693147...05599453` | 1.36e-17  | 12.22   | 1.00x          | ≈          |
| JSDecimal      | `0.693147...80559945` | 4.46e-16  | 147.08  | 12.04x         | ≈          |
| I64            | N/A                   | -         | -       | -              | ❌ Failed   |
| Decimal        | N/A                   | -         | -       | -              | ❌ Failed   |
| BigInt         | N/A                   | -         | -       | -              | ❌ Failed   |
| Rational64     | N/A                   | -         | -       | -              | ❌ Failed   |
| BigRational    | N/A                   | -         | -       | -              | ❌ Failed   |
| BigDecimal     | N/A                   | -         | -       | -              | ❌ Failed   |

### Test: `Ln::ln_negative`

**Operation:** `ln(-1) = NaN`

**Expected:** `NaN`

**Category:** EdgeCase

#### Results by Accuracy

| Type           | Result | Error     | ns/op  | Relative Speed | Status     |
|----------------|--------|-----------|--------|----------------|------------|
| F64            | `NaN`  | ✓ Exact   | 12.20  | 1.00x          | ✓          |
| JSDecimal      | `NaN`  | ✓ Exact   | 24.90  | 2.04x          | ✓          |
| FaithfulNumber | `NaN`  | ✓ Exact   | 200.98 | 16.47x         | ✓          |
| RugFloat       | `-NaN` | -         | 34.15  | 2.80x          | ≈          |
| I64            | N/A    | -         | -      | -              | ❌ Failed   |
| Decimal        | N/A    | -         | -      | -              | ❌ Failed   |
| BigInt         | N/A    | -         | -      | -              | ❌ Failed   |
| Rational64     | N/A    | -         | -      | -              | ❌ Failed   |
| BigRational    | N/A    | -         | -      | -              | ❌ Failed   |
| BigDecimal     | N/A    | -         | -      | -              | ❌ Failed   |

### Test: `Mul::basic_mul`

**Operation:** `3 × 4 = 12`

**Expected:** `12` (ExactInteger)

**Category:** Normal

#### Results by Accuracy

| Type           | Result | Error     | ns/op  | Relative Speed | Status |
|----------------|--------|-----------|--------|----------------|--------|
| F64            | `12`   | ✓ Exact   | 10.24  | 1.00x          | ✓      |
| I64            | `12`   | ✓ Exact   | 10.40  | 1.02x          | ✓      |
| Rational64     | `12`   | ✓ Exact   | 20.47  | 2.00x          | ✓      |
| Decimal        | `12`   | ✓ Exact   | 22.51  | 2.20x          | ✓      |
| JSDecimal      | `12`   | ✓ Exact   | 31.15  | 3.04x          | ✓      |
| FaithfulNumber | `12`   | ✓ Exact   | 31.58  | 3.08x          | ✓      |
| BigInt         | `12`   | ✓ Exact   | 32.97  | 3.22x          | ✓      |
| BigDecimal     | `12`   | ✓ Exact   | 70.34  | 6.87x          | ✓      |
| RugFloat       | `12`   | ✓ Exact   | 101.10 | 9.88x          | ✓      |
| BigRational    | `12`   | ✓ Exact   | 398.95 | 38.97x         | ✓      |

### Test: `Mul::extreme_mul_1e50`

**Operation:** `10000000000000000000000000 × 10000000000000000000000000 = 100000000000000000000000000000000000000000000000000`

**Expected:** `100000000000000000000000000000000000000000000000000` (ExactInteger)

**Category:** Boundary

#### Results by Accuracy

| Type           | Result                | Error     | ns/op   | Relative Speed | Status      |
|----------------|-----------------------|-----------|---------|----------------|-------------|
| BigInt         | `10000000...00000000` | ✓ Exact   | 39.39   | 3.85x          | ✓           |
| BigDecimal     | `10000000...00000000` | ✓ Exact   | 70.60   | 6.90x          | ✓           |
| RugFloat       | `10000000...00000000` | ✓ Exact   | 102.27  | 9.99x          | ✓           |
| FaithfulNumber | `10000000...00000000` | ✓ Exact   | 162.14  | 15.84x         | ✓           |
| BigRational    | `10000000...00000000` | ✓ Exact   | 1691.72 | 165.22x        | ✓           |
| F64            | `10000000...00000000` | 2.84e-16  | 10.24   | 1.00x          | ≈           |
| I64            | N/A                   | -         | -       | -              | ⚠ Skipped   |
| Decimal        | N/A                   | -         | -       | -              | ❌ Failed    |
| Rational64     | N/A                   | -         | -       | -              | ⚠ Skipped   |
| JSDecimal      | N/A                   | -         | -       | -              | ❌ Failed    |

### Test: `Mul::fractional_mul`

**Operation:** `1/2 × 1/4 = 0.125`

**Expected:** `0.125` (FiniteDecimal)

**Category:** PrecisionCritical

#### Results by Accuracy

| Type           | Result                | Error     | ns/op  | Relative Speed | Status |
|----------------|-----------------------|-----------|--------|----------------|--------|
| F64            | `0.125`               | ✓ Exact   | 10.31  | 1.00x          | ✓      |
| Rational64     | `1/8`                 | ✓ Exact   | 20.31  | 1.97x          | ✓      |
| Decimal        | `0.125`               | ✓ Exact   | 22.72  | 2.21x          | ✓      |
| JSDecimal      | `0.125`               | ✓ Exact   | 31.13  | 3.02x          | ✓      |
| FaithfulNumber | `0.125`               | ✓ Exact   | 47.27  | 4.59x          | ✓      |
| BigDecimal     | `0.125`               | ✓ Exact   | 70.12  | 6.81x          | ✓      |
| RugFloat       | `1.250000...00000e-1` | ✓ Exact   | 101.53 | 9.86x          | ✓      |
| BigRational    | `1/8`                 | ✓ Exact   | 375.07 | 36.41x         | ✓      |
| I64            | `0`                   | 1.00e0    | 10.30  | 1.00x          | ≈      |
| BigInt         | `0`                   | 1.00e0    | 13.55  | 1.32x          | ≈      |

### Test: `Mul::large_mul_1e15`

**Operation:** `1000000000 × 1000000 = 1000000000000000`

**Expected:** `1000000000000000` (ExactInteger)

**Category:** Boundary

#### Results by Accuracy

| Type           | Result             | Error     | ns/op  | Relative Speed | Status |
|----------------|--------------------|-----------|--------|----------------|--------|
| I64            | `1000000000000000` | ✓ Exact   | 10.26  | 1.00x          | ✓      |
| F64            | `1000000000000000` | ✓ Exact   | 10.27  | 1.00x          | ✓      |
| Decimal        | `1000000000000000` | ✓ Exact   | 22.71  | 2.21x          | ✓      |
| BigInt         | `1000000000000000` | ✓ Exact   | 27.84  | 2.71x          | ✓      |
| JSDecimal      | `1000000000000000` | ✓ Exact   | 31.15  | 3.04x          | ✓      |
| Rational64     | `1000000000000000` | ✓ Exact   | 46.02  | 4.49x          | ✓      |
| BigDecimal     | `1000000000000000` | ✓ Exact   | 64.04  | 6.24x          | ✓      |
| FaithfulNumber | `1000000000000000` | ✓ Exact   | 71.62  | 6.98x          | ✓      |
| RugFloat       | `1000000000000000` | ✓ Exact   | 94.42  | 9.20x          | ✓      |
| BigRational    | `1000000000000000` | ✓ Exact   | 801.70 | 78.14x         | ✓      |

### Test: `Mul::medium_mul`

**Operation:** `1000000 × 1000 = 1000000000`

**Expected:** `1000000000` (ExactInteger)

**Category:** Normal

#### Results by Accuracy

| Type           | Result       | Error     | ns/op  | Relative Speed | Status |
|----------------|--------------|-----------|--------|----------------|--------|
| F64            | `1000000000` | ✓ Exact   | 10.19  | 1.00x          | ✓      |
| I64            | `1000000000` | ✓ Exact   | 10.21  | 1.00x          | ✓      |
| Decimal        | `1000000000` | ✓ Exact   | 22.55  | 2.21x          | ✓      |
| BigInt         | `1000000000` | ✓ Exact   | 27.88  | 2.74x          | ✓      |
| JSDecimal      | `1000000000` | ✓ Exact   | 31.19  | 3.06x          | ✓      |
| Rational64     | `1000000000` | ✓ Exact   | 32.18  | 3.16x          | ✓      |
| FaithfulNumber | `1000000000` | ✓ Exact   | 61.30  | 6.01x          | ✓      |
| BigDecimal     | `1000000000` | ✓ Exact   | 66.09  | 6.48x          | ✓      |
| RugFloat       | `1000000000` | ✓ Exact   | 95.31  | 9.35x          | ✓      |
| BigRational    | `1000000000` | ✓ Exact   | 636.14 | 62.41x         | ✓      |

### Test: `Mul::small_times_large`

**Operation:** `0.1 × 10000000000000000 = 1000000000000000`

**Expected:** `1000000000000000` (ExactInteger)

**Category:** PrecisionCritical

#### Results by Accuracy

| Type           | Result             | Error     | ns/op  | Relative Speed | Status |
|----------------|--------------------|-----------|--------|----------------|--------|
| F64            | `1000000000000000` | ✓ Exact   | 10.35  | 1.00x          | ✓      |
| Decimal        | `1000000000000000` | ✓ Exact   | 23.05  | 2.23x          | ✓      |
| JSDecimal      | `1000000000000000` | ✓ Exact   | 32.13  | 3.10x          | ✓      |
| Rational64     | `1000000000000000` | ✓ Exact   | 48.96  | 4.73x          | ✓      |
| BigDecimal     | `1000000000000000` | ✓ Exact   | 66.60  | 6.43x          | ✓      |
| FaithfulNumber | `1000000000000000` | ✓ Exact   | 81.51  | 7.87x          | ✓      |
| RugFloat       | `1000000000000000` | ✓ Exact   | 102.55 | 9.90x          | ✓      |
| BigRational    | `1000000000000000` | ✓ Exact   | 920.48 | 88.90x         | ✓      |
| I64            | `0`                | 1.00e0    | 10.47  | 1.01x          | ≈      |
| BigInt         | `0`                | 1.00e0    | 13.56  | 1.31x          | ≈      |

### Test: `Mul::very_large_mul_1e20`

**Operation:** `10000000000 × 10000000000 = 100000000000000000000`

**Expected:** `100000000000000000000` (ExactInteger)

**Category:** Boundary

#### Results by Accuracy

| Type           | Result                | Error     | ns/op  | Relative Speed | Status     |
|----------------|-----------------------|-----------|--------|----------------|------------|
| F64            | `10000000...00000000` | ✓ Exact   | 10.21  | 1.00x          | ✓          |
| Decimal        | `10000000...00000000` | ✓ Exact   | 23.54  | 2.31x          | ✓          |
| JSDecimal      | `10000000...00000000` | ✓ Exact   | 33.60  | 3.29x          | ✓          |
| BigInt         | `10000000...00000000` | ✓ Exact   | 46.50  | 4.55x          | ✓          |
| FaithfulNumber | `10000000...00000000` | ✓ Exact   | 78.91  | 7.73x          | ✓          |
| BigDecimal     | `10000000...00000000` | ✓ Exact   | 84.87  | 8.31x          | ✓          |
| RugFloat       | `10000000...00000000` | ✓ Exact   | 101.27 | 9.92x          | ✓          |
| BigRational    | `10000000...00000000` | ✓ Exact   | 939.23 | 91.96x         | ✓          |
| I64            | N/A                   | -         | -      | -              | ❌ Failed   |
| Rational64     | N/A                   | -         | -      | -              | ❌ Failed   |

### Test: `Sin::sin_0`

**Operation:** `sin(0) = 0`

**Expected:** `0` (ExactInteger)

**Category:** Normal

#### Results by Accuracy

| Type           | Result | Error     | ns/op  | Relative Speed | Status     |
|----------------|--------|-----------|--------|----------------|------------|
| F64            | `0`    | ✓ Exact   | 16.17  | 1.00x          | ✓          |
| JSDecimal      | `0`    | ✓ Exact   | 32.32  | 2.00x          | ✓          |
| RugFloat       | `0`    | ✓ Exact   | 33.22  | 2.05x          | ✓          |
| FaithfulNumber | `0`    | ✓ Exact   | 145.09 | 8.97x          | ✓          |
| I64            | N/A    | -         | -      | -              | ❌ Failed   |
| Decimal        | N/A    | -         | -      | -              | ❌ Failed   |
| BigInt         | N/A    | -         | -      | -              | ❌ Failed   |
| Rational64     | N/A    | -         | -      | -              | ❌ Failed   |
| BigRational    | N/A    | -         | -      | -              | ❌ Failed   |
| BigDecimal     | N/A    | -         | -      | -              | ❌ Failed   |

### Test: `Sin::sin_pi_2`

**Operation:** `sin(355/226) = 0.999999999999991104608429233538678860702176608494706898668084524752545580977661156496453468892527151224291566485496439095571653268283690185377005688397596617`

**Expected:** `0.999999999999991104608429233538678860702176608494706898668084524752545580977661156496453468892527151224291566485496439095571653268283690185377005688397596617` (Transcendental)

**Category:** Normal

#### Results by Accuracy

| Type           | Result                | Error     | ns/op   | Relative Speed | Status     |
|----------------|-----------------------|-----------|---------|----------------|------------|
| RugFloat       | `9.999999...96617e-1` | ✓ Exact   | 6165.44 | 350.89x        | ✓          |
| FaithfulNumber | `0.999999...09776625` | 1.34e-78  | 4620.66 | 262.97x        | ≈          |
| F64            | `0.999999...99999911` | 4.61e-18  | 17.57   | 1.00x          | ≈          |
| JSDecimal      | `0.999999...99999991` | 1.05e-16  | 162.74  | 9.26x          | ≈          |
| I64            | N/A                   | -         | -       | -              | ❌ Failed   |
| Decimal        | N/A                   | -         | -       | -              | ❌ Failed   |
| BigInt         | N/A                   | -         | -       | -              | ❌ Failed   |
| Rational64     | N/A                   | -         | -       | -              | ❌ Failed   |
| BigRational    | N/A                   | -         | -       | -              | ❌ Failed   |
| BigDecimal     | N/A                   | -         | -       | -              | ❌ Failed   |

### Test: `Sqrt::sqrt_2`

**Operation:** `√2 = 1.41421356237309504880168872420969807856967187537694807317667973799073247846210703885038753432764157273501384623091229702492483605585073721264412149709993586`

**Expected:** `1.41421356237309504880168872420969807856967187537694807317667973799073247846210703885038753432764157273501384623091229702492483605585073721264412149709993586` (Transcendental)

**Category:** PrecisionCritical

#### Results by Accuracy

| Type           | Result                | Error     | ns/op   | Relative Speed | Status     |
|----------------|-----------------------|-----------|---------|----------------|------------|
| RugFloat       | `1.414213...09993586` | ✓ Exact   | 222.62  | 21.78x         | ✓          |
| BigDecimal     | `1.414213...27641573` | 1.87e-100 | 2186.11 | 213.91x        | ≈          |
| FaithfulNumber | `1.414213...78462102` | 3.56e-78  | 1334.67 | 130.60x        | ≈          |
| JSDecimal      | `1.414213...16887242` | 6.86e-30  | 1310.52 | 128.23x        | ≈          |
| BigRational    | `28284271...00000000` | 3.45e-17  | 443.69  | 43.41x         | ≈          |
| F64            | `1.414213...23730951` | 3.62e-17  | 10.22   | 1.00x          | ≈          |
| I64            | N/A                   | -         | -       | -              | ❌ Failed   |
| Decimal        | N/A                   | -         | -       | -              | ❌ Failed   |
| BigInt         | N/A                   | -         | -       | -              | ❌ Failed   |
| Rational64     | N/A                   | -         | -       | -              | ❌ Failed   |

### Test: `Sqrt::sqrt_4`

**Operation:** `√4 = 2`

**Expected:** `2` (PerfectRoot)

**Category:** Normal

#### Results by Accuracy

| Type           | Result | Error     | ns/op   | Relative Speed | Status     |
|----------------|--------|-----------|---------|----------------|------------|
| F64            | `2`    | ✓ Exact   | 10.22   | 1.00x          | ✓          |
| FaithfulNumber | `2`    | ✓ Exact   | 25.29   | 2.47x          | ✓          |
| BigRational    | `2`    | ✓ Exact   | 155.69  | 15.24x         | ✓          |
| JSDecimal      | `2`    | ✓ Exact   | 175.54  | 17.18x         | ✓          |
| RugFloat       | `2`    | ✓ Exact   | 273.92  | 26.81x         | ✓          |
| BigDecimal     | `2`    | ✓ Exact   | 2211.18 | 216.40x        | ✓          |
| I64            | N/A    | -         | -       | -              | ❌ Failed   |
| Decimal        | N/A    | -         | -       | -              | ❌ Failed   |
| BigInt         | N/A    | -         | -       | -              | ❌ Failed   |
| Rational64     | N/A    | -         | -       | -              | ❌ Failed   |

### Test: `Sqrt::sqrt_negative`

**Operation:** `√-1 = NaN`

**Expected:** `NaN`

**Category:** EdgeCase

#### Results by Accuracy

| Type           | Result | Error     | ns/op | Relative Speed | Status     |
|----------------|--------|-----------|-------|----------------|------------|
| F64            | `NaN`  | ✓ Exact   | 10.21 | 1.00x          | ✓          |
| FaithfulNumber | `NaN`  | ✓ Exact   | 17.68 | 1.73x          | ✓          |
| JSDecimal      | `NaN`  | ✓ Exact   | 24.63 | 2.41x          | ✓          |
| RugFloat       | `-NaN` | -         | 32.70 | 3.20x          | ≈          |
| BigRational    | `0`    | -         | 47.01 | 4.60x          | ≈          |
| I64            | N/A    | -         | -     | -              | ❌ Failed   |
| Decimal        | N/A    | -         | -     | -              | ❌ Failed   |
| BigInt         | N/A    | -         | -     | -              | ❌ Failed   |
| Rational64     | N/A    | -         | -     | -              | ❌ Failed   |
| BigDecimal     | N/A    | -         | -     | -              | ❌ Failed   |

### Test: `Sub::basic_sub`

**Operation:** `5 - 3 = 2`

**Expected:** `2` (ExactInteger)

**Category:** Normal

#### Results by Accuracy

| Type           | Result | Error     | ns/op  | Relative Speed | Status |
|----------------|--------|-----------|--------|----------------|--------|
| I64            | `2`    | ✓ Exact   | 10.23  | 1.00x          | ✓      |
| F64            | `2`    | ✓ Exact   | 10.43  | 1.02x          | ✓      |
| Rational64     | `2`    | ✓ Exact   | 18.81  | 1.84x          | ✓      |
| Decimal        | `2`    | ✓ Exact   | 21.87  | 2.14x          | ✓      |
| BigInt         | `2`    | ✓ Exact   | 26.29  | 2.57x          | ✓      |
| JSDecimal      | `2`    | ✓ Exact   | 30.74  | 3.01x          | ✓      |
| FaithfulNumber | `2`    | ✓ Exact   | 40.42  | 3.95x          | ✓      |
| BigDecimal     | `2`    | ✓ Exact   | 47.07  | 4.60x          | ✓      |
| RugFloat       | `2`    | ✓ Exact   | 59.32  | 5.80x          | ✓      |
| BigRational    | `2`    | ✓ Exact   | 174.82 | 17.10x         | ✓      |

### Test: `Sub::catastrophic_cancellation`

**Operation:** `10000000000000001 - 10000000000000000 = 1`

**Expected:** `1` (ExactInteger)

**Category:** PrecisionCritical

#### Results by Accuracy

| Type           | Result | Error     | ns/op | Relative Speed | Status |
|----------------|--------|-----------|-------|----------------|--------|
| I64            | `1`    | ✓ Exact   | 10.25 | 1.00x          | ✓      |
| Rational64     | `1`    | ✓ Exact   | 17.64 | 1.72x          | ✓      |
| Decimal        | `1`    | ✓ Exact   | 22.84 | 2.23x          | ✓      |
| BigInt         | `1`    | ✓ Exact   | 26.31 | 2.57x          | ✓      |
| JSDecimal      | `1`    | ✓ Exact   | 33.33 | 3.25x          | ✓      |
| FaithfulNumber | `1`    | ✓ Exact   | 35.80 | 3.49x          | ✓      |
| BigDecimal     | `1`    | ✓ Exact   | 46.98 | 4.58x          | ✓      |
| RugFloat       | `1`    | ✓ Exact   | 65.07 | 6.35x          | ✓      |
| BigRational    | `1`    | ✓ Exact   | 81.42 | 7.94x          | ✓      |
| F64            | `0`    | 1.00e0    | 10.34 | 1.01x          | ≈      |

### Test: `Sub::extreme_1e50`

**Operation:** `300000000000000000000000000000000000000000000000000 - 100000000000000000000000000000000000000000000000000 = 200000000000000000000000000000000000000000000000000`

**Expected:** `200000000000000000000000000000000000000000000000000` (ExactInteger)

**Category:** Boundary

#### Results by Accuracy

| Type           | Result                | Error     | ns/op  | Relative Speed | Status      |
|----------------|-----------------------|-----------|--------|----------------|-------------|
| BigInt         | `20000000...00000000` | ✓ Exact   | 26.94  | 2.62x          | ✓           |
| BigDecimal     | `20000000...00000000` | ✓ Exact   | 50.94  | 4.96x          | ✓           |
| RugFloat       | `20000000...00000000` | ✓ Exact   | 60.26  | 5.87x          | ✓           |
| FaithfulNumber | `20000000...00000000` | ✓ Exact   | 90.44  | 8.81x          | ✓           |
| BigRational    | `20000000...00000000` | ✓ Exact   | 829.12 | 80.74x         | ✓           |
| F64            | `19999999...00000000` | 1.31e-16  | 10.27  | 1.00x          | ≈           |
| JSDecimal      | `NaN`                 | NaN       | 17.38  | 1.69x          | ≈           |
| Decimal        | `0`                   | 1.00e0    | 22.86  | 2.23x          | ≈           |
| I64            | N/A                   | -         | -      | -              | ⚠ Skipped   |
| Rational64     | N/A                   | -         | -      | -              | ⚠ Skipped   |

### Test: `Sub::large_1e15`

**Operation:** `3000000000000000 - 1000000000000000 = 2000000000000000`

**Expected:** `2000000000000000` (ExactInteger)

**Category:** Boundary

#### Results by Accuracy

| Type           | Result             | Error     | ns/op  | Relative Speed | Status |
|----------------|--------------------|-----------|--------|----------------|--------|
| I64            | `2000000000000000` | ✓ Exact   | 10.25  | 1.00x          | ✓      |
| F64            | `2000000000000000` | ✓ Exact   | 10.43  | 1.02x          | ✓      |
| Decimal        | `2000000000000000` | ✓ Exact   | 22.83  | 2.23x          | ✓      |
| BigInt         | `2000000000000000` | ✓ Exact   | 26.20  | 2.56x          | ✓      |
| Rational64     | `2000000000000000` | ✓ Exact   | 28.84  | 2.81x          | ✓      |
| JSDecimal      | `2000000000000000` | ✓ Exact   | 33.03  | 3.22x          | ✓      |
| BigDecimal     | `2000000000000000` | ✓ Exact   | 47.67  | 4.65x          | ✓      |
| FaithfulNumber | `2000000000000000` | ✓ Exact   | 55.52  | 5.42x          | ✓      |
| RugFloat       | `2000000000000000` | ✓ Exact   | 66.26  | 6.47x          | ✓      |
| BigRational    | `2000000000000000` | ✓ Exact   | 392.20 | 38.28x         | ✓      |

### Test: `Sub::medium_1e6`

**Operation:** `3000000 - 1000000 = 2000000`

**Expected:** `2000000` (ExactInteger)

**Category:** Normal

#### Results by Accuracy

| Type           | Result    | Error     | ns/op  | Relative Speed | Status |
|----------------|-----------|-----------|--------|----------------|--------|
| I64            | `2000000` | ✓ Exact   | 10.24  | 1.00x          | ✓      |
| F64            | `2000000` | ✓ Exact   | 10.39  | 1.01x          | ✓      |
| Rational64     | `2000000` | ✓ Exact   | 20.34  | 1.99x          | ✓      |
| Decimal        | `2000000` | ✓ Exact   | 22.15  | 2.16x          | ✓      |
| BigInt         | `2000000` | ✓ Exact   | 25.99  | 2.54x          | ✓      |
| JSDecimal      | `2000000` | ✓ Exact   | 30.65  | 2.99x          | ✓      |
| FaithfulNumber | `2000000` | ✓ Exact   | 43.20  | 4.22x          | ✓      |
| BigDecimal     | `2000000` | ✓ Exact   | 46.90  | 4.58x          | ✓      |
| RugFloat       | `2000000` | ✓ Exact   | 65.78  | 6.42x          | ✓      |
| BigRational    | `2000000` | ✓ Exact   | 235.87 | 23.03x         | ✓      |

### Test: `Sub::medium_1e9`

**Operation:** `3000000000 - 1000000000 = 2000000000`

**Expected:** `2000000000` (ExactInteger)

**Category:** Boundary

#### Results by Accuracy

| Type           | Result       | Error     | ns/op  | Relative Speed | Status |
|----------------|--------------|-----------|--------|----------------|--------|
| I64            | `2000000000` | ✓ Exact   | 10.27  | 1.00x          | ✓      |
| F64            | `2000000000` | ✓ Exact   | 10.33  | 1.01x          | ✓      |
| Decimal        | `2000000000` | ✓ Exact   | 21.90  | 2.13x          | ✓      |
| Rational64     | `2000000000` | ✓ Exact   | 23.41  | 2.28x          | ✓      |
| BigInt         | `2000000000` | ✓ Exact   | 26.03  | 2.53x          | ✓      |
| JSDecimal      | `2000000000` | ✓ Exact   | 30.79  | 3.00x          | ✓      |
| BigDecimal     | `2000000000` | ✓ Exact   | 46.97  | 4.57x          | ✓      |
| FaithfulNumber | `2000000000` | ✓ Exact   | 51.53  | 5.02x          | ✓      |
| RugFloat       | `2000000000` | ✓ Exact   | 65.89  | 6.41x          | ✓      |
| BigRational    | `2000000000` | ✓ Exact   | 311.92 | 30.36x         | ✓      |

### Test: `Sub::near_i64_max`

**Operation:** `9223372036854776000 - 1000 = 9223372036854775000`

**Expected:** `9223372036854775000` (ExactInteger)

**Category:** Boundary

#### Results by Accuracy

| Type           | Result                | Error     | ns/op  | Relative Speed | Status      |
|----------------|-----------------------|-----------|--------|----------------|-------------|
| Decimal        | `92233720...54775000` | ✓ Exact   | 22.87  | 2.23x          | ✓           |
| BigInt         | `92233720...54775000` | ✓ Exact   | 26.60  | 2.59x          | ✓           |
| JSDecimal      | `92233720...54775000` | ✓ Exact   | 33.29  | 3.24x          | ✓           |
| BigDecimal     | `92233720...54775000` | ✓ Exact   | 47.44  | 4.61x          | ✓           |
| RugFloat       | `92233720...54775000` | ✓ Exact   | 66.13  | 6.43x          | ✓           |
| FaithfulNumber | `92233720...54775000` | ✓ Exact   | 72.76  | 7.08x          | ✓           |
| BigRational    | `92233720...54775000` | ✓ Exact   | 819.13 | 79.68x         | ✓           |
| F64            | `92233720...54775000` | 2.34e-17  | 10.28  | 1.00x          | ≈           |
| I64            | N/A                   | -         | -      | -              | ⚠ Skipped   |
| Rational64     | N/A                   | -         | -      | -              | ⚠ Skipped   |

### Test: `Sub::negative_result`

**Operation:** `3 - 5 = -2`

**Expected:** `-2` (ExactInteger)

**Category:** Normal

#### Results by Accuracy

| Type           | Result | Error     | ns/op  | Relative Speed | Status |
|----------------|--------|-----------|--------|----------------|--------|
| I64            | `-2`   | ✓ Exact   | 10.20  | 1.00x          | ✓      |
| F64            | `-2`   | ✓ Exact   | 10.26  | 1.01x          | ✓      |
| Rational64     | `-2`   | ✓ Exact   | 18.80  | 1.84x          | ✓      |
| Decimal        | `-2`   | ✓ Exact   | 21.96  | 2.15x          | ✓      |
| BigInt         | `-2`   | ✓ Exact   | 26.36  | 2.58x          | ✓      |
| JSDecimal      | `-2`   | ✓ Exact   | 30.69  | 3.01x          | ✓      |
| FaithfulNumber | `-2`   | ✓ Exact   | 40.56  | 3.97x          | ✓      |
| BigDecimal     | `-2`   | ✓ Exact   | 46.99  | 4.61x          | ✓      |
| RugFloat       | `-2`   | ✓ Exact   | 62.67  | 6.14x          | ✓      |
| BigRational    | `-2`   | ✓ Exact   | 168.11 | 16.48x         | ✓      |

### Test: `Sub::very_large_1e18`

**Operation:** `3000000000000000000 - 1000000000000000000 = 2000000000000000000`

**Expected:** `2000000000000000000` (ExactInteger)

**Category:** Boundary

#### Results by Accuracy

| Type           | Result                | Error     | ns/op  | Relative Speed | Status |
|----------------|-----------------------|-----------|--------|----------------|--------|
| I64            | `20000000...00000000` | ✓ Exact   | 10.27  | 1.00x          | ✓      |
| F64            | `20000000...00000000` | ✓ Exact   | 10.27  | 1.00x          | ✓      |
| Decimal        | `20000000...00000000` | ✓ Exact   | 22.76  | 2.22x          | ✓      |
| BigInt         | `20000000...00000000` | ✓ Exact   | 26.16  | 2.55x          | ✓      |
| Rational64     | `20000000...00000000` | ✓ Exact   | 31.76  | 3.09x          | ✓      |
| JSDecimal      | `20000000...00000000` | ✓ Exact   | 33.12  | 3.22x          | ✓      |
| FaithfulNumber | `20000000...00000000` | ✓ Exact   | 41.66  | 4.06x          | ✓      |
| BigDecimal     | `20000000...00000000` | ✓ Exact   | 47.46  | 4.62x          | ✓      |
| RugFloat       | `20000000...00000000` | ✓ Exact   | 66.06  | 6.43x          | ✓      |
| BigRational    | `20000000...00000000` | ✓ Exact   | 438.32 | 42.68x         | ✓      |

### Test: `Sub::very_large_1e20`

**Operation:** `300000000000000000000 - 100000000000000000000 = 200000000000000000000`

**Expected:** `200000000000000000000` (ExactInteger)

**Category:** Boundary

#### Results by Accuracy

| Type           | Result                | Error     | ns/op  | Relative Speed | Status      |
|----------------|-----------------------|-----------|--------|----------------|-------------|
| F64            | `20000000...00000000` | ✓ Exact   | 10.26  | 1.00x          | ✓           |
| Decimal        | `20000000...00000000` | ✓ Exact   | 23.06  | 2.25x          | ✓           |
| BigInt         | `20000000...00000000` | ✓ Exact   | 26.55  | 2.59x          | ✓           |
| JSDecimal      | `20000000...00000000` | ✓ Exact   | 33.90  | 3.30x          | ✓           |
| FaithfulNumber | `20000000...00000000` | ✓ Exact   | 41.57  | 4.05x          | ✓           |
| BigDecimal     | `20000000...00000000` | ✓ Exact   | 49.74  | 4.85x          | ✓           |
| RugFloat       | `20000000...00000000` | ✓ Exact   | 66.24  | 6.46x          | ✓           |
| BigRational    | `20000000...00000000` | ✓ Exact   | 459.96 | 44.83x         | ✓           |
| I64            | N/A                   | -         | -      | -              | ⚠ Skipped   |
| Rational64     | N/A                   | -         | -      | -              | ⚠ Skipped   |


