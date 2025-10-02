# Number Representation Test Results

## Executive Summary

Tested 10 number types across 7 operations with 430 total test cases.

## Accuracy Summary

Breakdown of exact vs approximate results for each type:

| Type | Exact | Approximate | No Result/Skipped | Total Error | Total Time (µs) |
|------|-------|-------------|-------------------|-------------|----------------|
| FaithfulNumber | 40 | 3 | 0 | 7.17e-78 | 11.69 |
| BigRational | 36 | 2 | 5 | 4.88e-17 | 16.45 |
| RugFloat | 36 | 7 | 0 | 7.60e-152 | 13.54 |
| JSDecimal | 33 | 9 | 1 | NaN | 2.50 |
| BigDecimal | 33 | 4 | 6 | 3.75e-100 | 19.59 |
| BigInt | 29 | 6 | 8 | 1.00e15 | 1.01 |
| F64 | 28 | 15 | 0 | 6.99e34 | 0.43 |
| Decimal | 28 | 5 | 10 | 2.00e50 | 0.72 |
| Rational64 | 25 | 0 | 18 | 0.00e0 | 0.53 |
| I64 | 19 | 6 | 18 | 1.00e15 | 0.24 |


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
| Rational64     | `3/10`                | ✓ Exact   | 17.91  | 1.89x          | ✓      |
| Decimal        | `0.3`                 | ✓ Exact   | 20.52  | 2.17x          | ✓      |
| JSDecimal      | `0.3`                 | ✓ Exact   | 23.19  | 2.45x          | ✓      |
| FaithfulNumber | `0.3`                 | ✓ Exact   | 38.45  | 4.06x          | ✓      |
| BigDecimal     | `0.3`                 | ✓ Exact   | 46.47  | 4.90x          | ✓      |
| BigRational    | `3/10`                | ✓ Exact   | 323.24 | 34.11x         | ✓      |
| RugFloat       | `3.000000...00015e-1` | 5.00e-155 | 56.68  | 5.98x          | ≈      |
| F64            | `0.300000...00000004` | 1.33e-16  | 9.51   | 1.00x          | ≈      |
| I64            | `0`                   | 1.00e0    | 9.48   | 1.00x          | ≈      |
| BigInt         | `0`                   | 1.00e0    | 11.94  | 1.26x          | ≈      |

### Test: `Add::extreme_1e50`

**Operation:** `100000000000000000000000000000000000000000000000000 + 100000000000000000000000000000000000000000000000000 = 200000000000000000000000000000000000000000000000000`

**Expected:** `200000000000000000000000000000000000000000000000000` (ExactInteger)

**Category:** Boundary

#### Results by Accuracy

| Type           | Result                | Error     | ns/op  | Relative Speed | Status      |
|----------------|-----------------------|-----------|--------|----------------|-------------|
| BigInt         | `20000000...00000000` | ✓ Exact   | 24.32  | 2.51x          | ✓           |
| BigDecimal     | `20000000...00000000` | ✓ Exact   | 48.14  | 4.96x          | ✓           |
| RugFloat       | `20000000...00000000` | ✓ Exact   | 55.32  | 5.70x          | ✓           |
| FaithfulNumber | `20000000...00000000` | ✓ Exact   | 64.84  | 6.68x          | ✓           |
| BigRational    | `20000000...00000000` | ✓ Exact   | 769.48 | 79.27x         | ✓           |
| F64            | `20000000...00000000` | 7.63e-17  | 9.71   | 1.00x          | ≈           |
| JSDecimal      | `NaN`                 | NaN       | 16.31  | 1.68x          | ≈           |
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
| I64            | `10000000...00000001` | ✓ Exact   | 9.54   | 1.00x          | ✓      |
| Decimal        | `10000000...00000001` | ✓ Exact   | 21.39  | 2.24x          | ✓      |
| JSDecimal      | `10000000...00000001` | ✓ Exact   | 24.12  | 2.53x          | ✓      |
| BigInt         | `10000000...00000001` | ✓ Exact   | 24.28  | 2.55x          | ✓      |
| Rational64     | `10000000...00000001` | ✓ Exact   | 28.45  | 2.98x          | ✓      |
| BigDecimal     | `10000000...00000001` | ✓ Exact   | 45.90  | 4.81x          | ✓      |
| RugFloat       | `10000000...00000001` | ✓ Exact   | 51.52  | 5.40x          | ✓      |
| FaithfulNumber | `10000000...00000001` | ✓ Exact   | 54.76  | 5.74x          | ✓      |
| BigRational    | `10000000...00000001` | ✓ Exact   | 362.85 | 38.05x         | ✓      |
| F64            | `10000000...00000000` | 1.00e-16  | 9.58   | 1.00x          | ≈      |

### Test: `Add::large_1e15`

**Operation:** `1000000000000000 + 1000000000000000 = 2000000000000000`

**Expected:** `2000000000000000` (ExactInteger)

**Category:** Boundary

#### Results by Accuracy

| Type           | Result             | Error     | ns/op  | Relative Speed | Status |
|----------------|--------------------|-----------|--------|----------------|--------|
| F64            | `2000000000000000` | ✓ Exact   | 9.66   | 1.00x          | ✓      |
| I64            | `2000000000000000` | ✓ Exact   | 9.69   | 1.00x          | ✓      |
| Decimal        | `2000000000000000` | ✓ Exact   | 21.65  | 2.24x          | ✓      |
| JSDecimal      | `2000000000000000` | ✓ Exact   | 24.63  | 2.55x          | ✓      |
| BigInt         | `2000000000000000` | ✓ Exact   | 27.34  | 2.83x          | ✓      |
| Rational64     | `2000000000000000` | ✓ Exact   | 28.54  | 2.95x          | ✓      |
| BigDecimal     | `2000000000000000` | ✓ Exact   | 46.41  | 4.80x          | ✓      |
| FaithfulNumber | `2000000000000000` | ✓ Exact   | 52.17  | 5.40x          | ✓      |
| RugFloat       | `2000000000000000` | ✓ Exact   | 55.25  | 5.72x          | ✓      |
| BigRational    | `2000000000000000` | ✓ Exact   | 359.02 | 37.16x         | ✓      |

### Test: `Add::medium_1e6`

**Operation:** `1000000 + 1000000 = 2000000`

**Expected:** `2000000` (ExactInteger)

**Category:** Normal

#### Results by Accuracy

| Type           | Result    | Error     | ns/op  | Relative Speed | Status |
|----------------|-----------|-----------|--------|----------------|--------|
| I64            | `2000000` | ✓ Exact   | 9.66   | 1.00x          | ✓      |
| F64            | `2000000` | ✓ Exact   | 9.67   | 1.00x          | ✓      |
| Rational64     | `2000000` | ✓ Exact   | 19.27  | 1.99x          | ✓      |
| Decimal        | `2000000` | ✓ Exact   | 20.75  | 2.15x          | ✓      |
| JSDecimal      | `2000000` | ✓ Exact   | 23.51  | 2.43x          | ✓      |
| BigInt         | `2000000` | ✓ Exact   | 24.35  | 2.52x          | ✓      |
| FaithfulNumber | `2000000` | ✓ Exact   | 25.52  | 2.64x          | ✓      |
| BigDecimal     | `2000000` | ✓ Exact   | 47.45  | 4.91x          | ✓      |
| RugFloat       | `2000000` | ✓ Exact   | 54.52  | 5.64x          | ✓      |
| BigRational    | `2000000` | ✓ Exact   | 219.48 | 22.71x         | ✓      |

### Test: `Add::medium_1e9`

**Operation:** `1000000000 + 1000000000 = 2000000000`

**Expected:** `2000000000` (ExactInteger)

**Category:** Boundary

#### Results by Accuracy

| Type           | Result       | Error     | ns/op  | Relative Speed | Status |
|----------------|--------------|-----------|--------|----------------|--------|
| F64            | `2000000000` | ✓ Exact   | 9.65   | 1.00x          | ✓      |
| I64            | `2000000000` | ✓ Exact   | 9.71   | 1.01x          | ✓      |
| Decimal        | `2000000000` | ✓ Exact   | 21.45  | 2.22x          | ✓      |
| Rational64     | `2000000000` | ✓ Exact   | 22.96  | 2.38x          | ✓      |
| JSDecimal      | `2000000000` | ✓ Exact   | 23.49  | 2.43x          | ✓      |
| BigInt         | `2000000000` | ✓ Exact   | 23.88  | 2.47x          | ✓      |
| FaithfulNumber | `2000000000` | ✓ Exact   | 45.54  | 4.72x          | ✓      |
| BigDecimal     | `2000000000` | ✓ Exact   | 51.81  | 5.37x          | ✓      |
| RugFloat       | `2000000000` | ✓ Exact   | 55.22  | 5.72x          | ✓      |
| BigRational    | `2000000000` | ✓ Exact   | 296.94 | 30.78x         | ✓      |

### Test: `Add::near_i64_max`

**Operation:** `9223372036854775000 + 1000 = 9223372036854776000`

**Expected:** `9223372036854776000` (ExactInteger)

**Category:** Boundary

#### Results by Accuracy

| Type           | Result                | Error     | ns/op  | Relative Speed | Status     |
|----------------|-----------------------|-----------|--------|----------------|------------|
| Decimal        | `92233720...54776000` | ✓ Exact   | 21.62  | 2.20x          | ✓          |
| BigInt         | `92233720...54776000` | ✓ Exact   | 23.95  | 2.44x          | ✓          |
| JSDecimal      | `92233720...54776000` | ✓ Exact   | 24.37  | 2.48x          | ✓          |
| BigDecimal     | `92233720...54776000` | ✓ Exact   | 45.69  | 4.65x          | ✓          |
| FaithfulNumber | `92233720...54776000` | ✓ Exact   | 52.84  | 5.38x          | ✓          |
| RugFloat       | `92233720...54776000` | ✓ Exact   | 56.92  | 5.80x          | ✓          |
| BigRational    | `92233720...54776000` | ✓ Exact   | 178.44 | 18.18x         | ✓          |
| F64            | `92233720...54776000` | 2.08e-17  | 9.82   | 1.00x          | ≈          |
| I64            | N/A                   | -         | -      | -              | ❌ Failed   |
| Rational64     | N/A                   | -         | -      | -              | ❌ Failed   |

### Test: `Add::small_positive`

**Operation:** `1 + 2 = 3`

**Expected:** `3` (ExactInteger)

**Category:** Normal

#### Results by Accuracy

| Type           | Result | Error     | ns/op  | Relative Speed | Status |
|----------------|--------|-----------|--------|----------------|--------|
| F64            | `3`    | ✓ Exact   | 9.75   | 1.00x          | ✓      |
| I64            | `3`    | ✓ Exact   | 9.81   | 1.01x          | ✓      |
| Rational64     | `3`    | ✓ Exact   | 17.84  | 1.83x          | ✓      |
| Decimal        | `3`    | ✓ Exact   | 21.06  | 2.16x          | ✓      |
| JSDecimal      | `3`    | ✓ Exact   | 23.38  | 2.40x          | ✓      |
| FaithfulNumber | `3`    | ✓ Exact   | 25.01  | 2.57x          | ✓      |
| BigInt         | `3`    | ✓ Exact   | 25.13  | 2.58x          | ✓      |
| BigDecimal     | `3`    | ✓ Exact   | 46.37  | 4.76x          | ✓      |
| RugFloat       | `3`    | ✓ Exact   | 51.37  | 5.27x          | ✓      |
| BigRational    | `3`    | ✓ Exact   | 168.49 | 17.28x         | ✓      |

### Test: `Add::very_large_1e18`

**Operation:** `1000000000000000000 + 1000000000000000000 = 2000000000000000000`

**Expected:** `2000000000000000000` (ExactInteger)

**Category:** Boundary

#### Results by Accuracy

| Type           | Result                | Error     | ns/op  | Relative Speed | Status |
|----------------|-----------------------|-----------|--------|----------------|--------|
| F64            | `20000000...00000000` | ✓ Exact   | 9.58   | 1.00x          | ✓      |
| I64            | `20000000...00000000` | ✓ Exact   | 9.80   | 1.02x          | ✓      |
| Decimal        | `20000000...00000000` | ✓ Exact   | 21.76  | 2.27x          | ✓      |
| BigInt         | `20000000...00000000` | ✓ Exact   | 24.19  | 2.52x          | ✓      |
| JSDecimal      | `20000000...00000000` | ✓ Exact   | 24.69  | 2.58x          | ✓      |
| FaithfulNumber | `20000000...00000000` | ✓ Exact   | 28.72  | 3.00x          | ✓      |
| Rational64     | `20000000...00000000` | ✓ Exact   | 30.88  | 3.22x          | ✓      |
| BigDecimal     | `20000000...00000000` | ✓ Exact   | 46.67  | 4.87x          | ✓      |
| RugFloat       | `20000000...00000000` | ✓ Exact   | 55.17  | 5.76x          | ✓      |
| BigRational    | `20000000...00000000` | ✓ Exact   | 400.74 | 41.81x         | ✓      |

### Test: `Add::very_large_1e20`

**Operation:** `100000000000000000000 + 100000000000000000000 = 200000000000000000000`

**Expected:** `200000000000000000000` (ExactInteger)

**Category:** Boundary

#### Results by Accuracy

| Type           | Result                | Error     | ns/op  | Relative Speed | Status      |
|----------------|-----------------------|-----------|--------|----------------|-------------|
| F64            | `20000000...00000000` | ✓ Exact   | 9.73   | 1.00x          | ✓           |
| Decimal        | `20000000...00000000` | ✓ Exact   | 21.90  | 2.25x          | ✓           |
| BigInt         | `20000000...00000000` | ✓ Exact   | 24.29  | 2.50x          | ✓           |
| JSDecimal      | `20000000...00000000` | ✓ Exact   | 24.43  | 2.51x          | ✓           |
| FaithfulNumber | `20000000...00000000` | ✓ Exact   | 28.54  | 2.93x          | ✓           |
| BigDecimal     | `20000000...00000000` | ✓ Exact   | 48.36  | 4.97x          | ✓           |
| RugFloat       | `20000000...00000000` | ✓ Exact   | 55.19  | 5.67x          | ✓           |
| BigRational    | `20000000...00000000` | ✓ Exact   | 421.14 | 43.28x         | ✓           |
| I64            | N/A                   | -         | -      | -              | ⚠ Skipped   |
| Rational64     | N/A                   | -         | -      | -              | ⚠ Skipped   |

### Test: `Div::basic_div`

**Operation:** `12 ÷ 4 = 3`

**Expected:** `3` (ExactInteger)

**Category:** Normal

#### Results by Accuracy

| Type           | Result | Error     | ns/op  | Relative Speed | Status |
|----------------|--------|-----------|--------|----------------|--------|
| I64            | `3`    | ✓ Exact   | 9.47   | 1.00x          | ✓      |
| F64            | `3`    | ✓ Exact   | 9.51   | 1.00x          | ✓      |
| Rational64     | `3`    | ✓ Exact   | 10.88  | 1.15x          | ✓      |
| Decimal        | `3`    | ✓ Exact   | 16.99  | 1.79x          | ✓      |
| BigInt         | `3`    | ✓ Exact   | 24.12  | 2.55x          | ✓      |
| JSDecimal      | `3`    | ✓ Exact   | 24.29  | 2.57x          | ✓      |
| FaithfulNumber | `3`    | ✓ Exact   | 26.93  | 2.84x          | ✓      |
| BigDecimal     | `3`    | ✓ Exact   | 57.96  | 6.12x          | ✓      |
| RugFloat       | `3`    | ✓ Exact   | 244.52 | 25.83x         | ✓      |
| BigRational    | `3`    | ✓ Exact   | 363.99 | 38.44x         | ✓      |

### Test: `Div::large_div_large`

**Operation:** `100000000000000000000 ÷ 10000000000 = 10000000000`

**Expected:** `10000000000` (ExactInteger)

**Category:** Boundary

#### Results by Accuracy

| Type           | Result        | Error     | ns/op  | Relative Speed | Status      |
|----------------|---------------|-----------|--------|----------------|-------------|
| F64            | `10000000000` | ✓ Exact   | 9.45   | 1.00x          | ✓           |
| Decimal        | `10000000000` | ✓ Exact   | 18.85  | 1.99x          | ✓           |
| JSDecimal      | `10000000000` | ✓ Exact   | 26.33  | 2.79x          | ✓           |
| BigInt         | `10000000000` | ✓ Exact   | 30.36  | 3.21x          | ✓           |
| BigDecimal     | `10000000000` | ✓ Exact   | 58.81  | 6.22x          | ✓           |
| FaithfulNumber | `10000000000` | ✓ Exact   | 75.65  | 8.00x          | ✓           |
| RugFloat       | `10000000000` | ✓ Exact   | 244.59 | 25.88x         | ✓           |
| BigRational    | `10000000000` | ✓ Exact   | 566.64 | 59.95x         | ✓           |
| I64            | N/A           | -         | -      | -              | ⚠ Skipped   |
| Rational64     | N/A           | -         | -      | -              | ⚠ Skipped   |

### Test: `Div::large_div_small`

**Operation:** `1000000000000000000 ÷ 1000000 = 1000000000000`

**Expected:** `1000000000000` (ExactInteger)

**Category:** Normal

#### Results by Accuracy

| Type           | Result          | Error     | ns/op  | Relative Speed | Status |
|----------------|-----------------|-----------|--------|----------------|--------|
| F64            | `1000000000000` | ✓ Exact   | 9.47   | 1.00x          | ✓      |
| I64            | `1000000000000` | ✓ Exact   | 9.49   | 1.00x          | ✓      |
| Rational64     | `1000000000000` | ✓ Exact   | 13.82  | 1.46x          | ✓      |
| Decimal        | `1000000000000` | ✓ Exact   | 17.25  | 1.82x          | ✓      |
| BigInt         | `1000000000000` | ✓ Exact   | 24.02  | 2.54x          | ✓      |
| JSDecimal      | `1000000000000` | ✓ Exact   | 24.56  | 2.59x          | ✓      |
| BigDecimal     | `1000000000000` | ✓ Exact   | 58.17  | 6.14x          | ✓      |
| FaithfulNumber | `1000000000000` | ✓ Exact   | 69.42  | 7.33x          | ✓      |
| RugFloat       | `1000000000000` | ✓ Exact   | 240.10 | 25.34x         | ✓      |
| BigRational    | `1000000000000` | ✓ Exact   | 605.65 | 63.92x         | ✓      |

### Test: `Div::one_seventh`

**Operation:** `1 ÷ 7 = 1/7`

**Expected:** `1/7` (RepeatingRational)

**Category:** PrecisionCritical

#### Results by Accuracy

| Type           | Result                | Error     | ns/op   | Relative Speed | Status |
|----------------|-----------------------|-----------|---------|----------------|--------|
| Rational64     | `1/7`                 | ✓ Exact   | 11.14   | 1.17x          | ✓      |
| FaithfulNumber | `0.142857...28571429` | ✓ Exact   | 29.18   | 3.07x          | ✓      |
| BigRational    | `1/7`                 | ✓ Exact   | 375.49  | 39.48x         | ✓      |
| RugFloat       | `1.428571...42852e-1` | 3.73e-155 | 130.59  | 13.73x         | ≈      |
| BigDecimal     | `0.142857...28571429` | 3.00e-100 | 4553.33 | 478.75x        | ≈      |
| Decimal        | `0.142857...28571429` | 3.00e-28  | 28.85   | 3.03x          | ≈      |
| JSDecimal      | `0.142857...28571429` | 3.00e-28  | 38.69   | 4.07x          | ≈      |
| F64            | `0.142857...85714285` | 5.55e-17  | 9.55    | 1.00x          | ≈      |
| I64            | `0`                   | 1.00e0    | 9.51    | 1.00x          | ≈      |
| BigInt         | `0`                   | 1.00e0    | 41.54   | 4.37x          | ≈      |

### Test: `Div::one_third`

**Operation:** `1 ÷ 3 = 1/3`

**Expected:** `1/3` (RepeatingRational)

**Category:** PrecisionCritical

#### Results by Accuracy

| Type           | Result                | Error     | ns/op   | Relative Speed | Status |
|----------------|-----------------------|-----------|---------|----------------|--------|
| Rational64     | `1/3`                 | ✓ Exact   | 10.89   | 1.14x          | ✓      |
| FaithfulNumber | `0.333333...33333333` | ✓ Exact   | 28.16   | 2.96x          | ✓      |
| BigRational    | `1/3`                 | ✓ Exact   | 353.72  | 37.20x         | ✓      |
| RugFloat       | `3.333333...33346e-1` | 3.73e-155 | 130.87  | 13.76x         | ≈      |
| BigDecimal     | `0.333333...33333333` | 1.00e-100 | 4566.82 | 480.22x        | ≈      |
| Decimal        | `0.333333...33333333` | 1.00e-28  | 28.52   | 3.00x          | ≈      |
| JSDecimal      | `0.333333...33333333` | 1.00e-28  | 38.43   | 4.04x          | ≈      |
| F64            | `0.333333...33333333` | 5.55e-17  | 9.51    | 1.00x          | ≈      |
| I64            | `0`                   | 1.00e0    | 9.51    | 1.00x          | ≈      |
| BigInt         | `0`                   | 1.00e0    | 41.38   | 4.35x          | ≈      |

### Test: `Div::third_times_three`

**Operation:** `3 ÷ 3 = 1`

**Expected:** `1` (ExactInteger)

**Category:** Normal

#### Results by Accuracy

| Type           | Result | Error     | ns/op  | Relative Speed | Status |
|----------------|--------|-----------|--------|----------------|--------|
| F64            | `1`    | ✓ Exact   | 9.50   | 1.00x          | ✓      |
| Rational64     | `1`    | ✓ Exact   | 9.54   | 1.00x          | ✓      |
| I64            | `1`    | ✓ Exact   | 9.55   | 1.01x          | ✓      |
| Decimal        | `1`    | ✓ Exact   | 16.94  | 1.78x          | ✓      |
| BigInt         | `1`    | ✓ Exact   | 24.00  | 2.53x          | ✓      |
| JSDecimal      | `1`    | ✓ Exact   | 24.22  | 2.55x          | ✓      |
| FaithfulNumber | `1`    | ✓ Exact   | 25.84  | 2.72x          | ✓      |
| BigDecimal     | `1`    | ✓ Exact   | 40.88  | 4.30x          | ✓      |
| RugFloat       | `1`    | ✓ Exact   | 250.67 | 26.40x         | ✓      |
| BigRational    | `1`    | ✓ Exact   | 265.99 | 28.01x         | ✓      |

### Test: `Div::two_thirds`

**Operation:** `2 ÷ 3 = 2/3`

**Expected:** `2/3` (RepeatingRational)

**Category:** PrecisionCritical

#### Results by Accuracy

| Type           | Result                | Error     | ns/op   | Relative Speed | Status |
|----------------|-----------------------|-----------|---------|----------------|--------|
| Rational64     | `2/3`                 | ✓ Exact   | 10.87   | 1.14x          | ✓      |
| FaithfulNumber | `0.666666...66666667` | ✓ Exact   | 28.18   | 2.97x          | ✓      |
| BigRational    | `2/3`                 | ✓ Exact   | 356.07  | 37.50x         | ✓      |
| RugFloat       | `6.666666...66692e-1` | 3.73e-155 | 131.20  | 13.82x         | ≈      |
| BigDecimal     | `0.666666...66666667` | 5.00e-101 | 4521.83 | 476.25x        | ≈      |
| Decimal        | `0.666666...66666667` | 5.00e-29  | 28.65   | 3.02x          | ≈      |
| JSDecimal      | `0.666666...66666667` | 5.00e-29  | 38.66   | 4.07x          | ≈      |
| F64            | `0.666666...66666666` | 5.55e-17  | 9.51    | 1.00x          | ≈      |
| I64            | `0`                   | 1.00e0    | 9.49    | 1.00x          | ≈      |
| BigInt         | `0`                   | 1.00e0    | 41.40   | 4.36x          | ≈      |

### Test: `Div::very_large_div`

**Operation:** `100000000000000000000000000000000000000000000000000 ÷ 10000000000000000000000000 = 10000000000000000000000000`

**Expected:** `10000000000000000000000000` (ExactInteger)

**Category:** Boundary

#### Results by Accuracy

| Type           | Result                | Error     | ns/op  | Relative Speed | Status      |
|----------------|-----------------------|-----------|--------|----------------|-------------|
| BigInt         | `10000000...00000000` | ✓ Exact   | 126.91 | 13.41x         | ✓           |
| BigDecimal     | `10000000...00000000` | ✓ Exact   | 138.98 | 14.69x         | ✓           |
| FaithfulNumber | `10000000...00000000` | ✓ Exact   | 195.55 | 20.67x         | ✓           |
| RugFloat       | `10000000...00000000` | ✓ Exact   | 246.27 | 26.03x         | ✓           |
| BigRational    | `10000000...00000000` | ✓ Exact   | 865.89 | 91.53x         | ✓           |
| F64            | `10000000...00000000` | 9.06e-17  | 9.46   | 1.00x          | ≈           |
| JSDecimal      | `NaN`                 | NaN       | 16.17  | 1.71x          | ≈           |
| Decimal        | `7922.816...43950335` | 1.00e0    | 43.38  | 4.58x          | ≈           |
| I64            | N/A                   | -         | -      | -              | ⚠ Skipped   |
| Rational64     | N/A                   | -         | -      | -              | ⚠ Skipped   |

### Test: `Ln::ln_1`

**Operation:** `ln(1) = 0`

**Expected:** `0` (ExactInteger)

**Category:** Normal

#### Results by Accuracy

| Type           | Result | Error     | ns/op  | Relative Speed | Status     |
|----------------|--------|-----------|--------|----------------|------------|
| F64            | `0`    | ✓ Exact   | 10.26  | 1.00x          | ✓          |
| JSDecimal      | `0`    | ✓ Exact   | 16.83  | 1.64x          | ✓          |
| RugFloat       | `0`    | ✓ Exact   | 34.85  | 3.40x          | ✓          |
| FaithfulNumber | `0`    | ✓ Exact   | 292.26 | 28.50x         | ✓          |
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
| RugFloat       | `6.931471...75944e-1` | 1.10e-151 | 4103.00 | 363.41x        | ≈          |
| FaithfulNumber | `0.693147...19696955` | 1.13e-78  | 3692.10 | 327.02x        | ≈          |
| F64            | `0.693147...05599453` | 1.36e-17  | 11.29   | 1.00x          | ≈          |
| JSDecimal      | `0.693147...80559945` | 4.46e-16  | 119.57  | 10.59x         | ≈          |
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
| F64            | `NaN`  | ✓ Exact   | 11.06  | 1.00x          | ✓          |
| JSDecimal      | `NaN`  | ✓ Exact   | 17.28  | 1.56x          | ✓          |
| FaithfulNumber | `NaN`  | ✓ Exact   | 181.66 | 16.43x         | ✓          |
| RugFloat       | `-NaN` | -         | 32.06  | 2.90x          | ≈          |
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
| F64            | `12`   | ✓ Exact   | 9.52   | 1.00x          | ✓      |
| I64            | `12`   | ✓ Exact   | 9.56   | 1.00x          | ✓      |
| Rational64     | `12`   | ✓ Exact   | 18.70  | 1.96x          | ✓      |
| Decimal        | `12`   | ✓ Exact   | 20.42  | 2.15x          | ✓      |
| JSDecimal      | `12`   | ✓ Exact   | 23.47  | 2.47x          | ✓      |
| FaithfulNumber | `12`   | ✓ Exact   | 26.89  | 2.83x          | ✓      |
| BigInt         | `12`   | ✓ Exact   | 30.81  | 3.24x          | ✓      |
| BigDecimal     | `12`   | ✓ Exact   | 65.84  | 6.92x          | ✓      |
| RugFloat       | `12`   | ✓ Exact   | 95.72  | 10.06x         | ✓      |
| BigRational    | `12`   | ✓ Exact   | 358.03 | 37.63x         | ✓      |

### Test: `Mul::extreme_mul_1e50`

**Operation:** `10000000000000000000000000 × 10000000000000000000000000 = 100000000000000000000000000000000000000000000000000`

**Expected:** `100000000000000000000000000000000000000000000000000` (ExactInteger)

**Category:** Boundary

#### Results by Accuracy

| Type           | Result                | Error     | ns/op   | Relative Speed | Status      |
|----------------|-----------------------|-----------|---------|----------------|-------------|
| BigInt         | `10000000...00000000` | ✓ Exact   | 36.02   | 3.77x          | ✓           |
| BigDecimal     | `10000000...00000000` | ✓ Exact   | 65.93   | 6.91x          | ✓           |
| RugFloat       | `10000000...00000000` | ✓ Exact   | 96.37   | 10.10x         | ✓           |
| FaithfulNumber | `10000000...00000000` | ✓ Exact   | 163.04  | 17.09x         | ✓           |
| BigRational    | `10000000...00000000` | ✓ Exact   | 1524.70 | 159.78x        | ✓           |
| F64            | `10000000...00000000` | 2.84e-16  | 9.54    | 1.00x          | ≈           |
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
| F64            | `0.125`               | ✓ Exact   | 9.54   | 1.01x          | ✓      |
| Rational64     | `1/8`                 | ✓ Exact   | 18.47  | 1.96x          | ✓      |
| Decimal        | `0.125`               | ✓ Exact   | 20.55  | 2.18x          | ✓      |
| JSDecimal      | `0.125`               | ✓ Exact   | 23.38  | 2.48x          | ✓      |
| FaithfulNumber | `0.125`               | ✓ Exact   | 40.87  | 4.33x          | ✓      |
| BigDecimal     | `0.125`               | ✓ Exact   | 62.97  | 6.67x          | ✓      |
| RugFloat       | `1.250000...00000e-1` | ✓ Exact   | 94.82  | 10.04x         | ✓      |
| BigRational    | `1/8`                 | ✓ Exact   | 350.13 | 37.07x         | ✓      |
| I64            | `0`                   | 1.00e0    | 9.45   | 1.00x          | ≈      |
| BigInt         | `0`                   | 1.00e0    | 12.69  | 1.34x          | ≈      |

### Test: `Mul::large_mul_1e15`

**Operation:** `1000000000 × 1000000 = 1000000000000000`

**Expected:** `1000000000000000` (ExactInteger)

**Category:** Boundary

#### Results by Accuracy

| Type           | Result             | Error     | ns/op  | Relative Speed | Status |
|----------------|--------------------|-----------|--------|----------------|--------|
| I64            | `1000000000000000` | ✓ Exact   | 9.45   | 1.00x          | ✓      |
| F64            | `1000000000000000` | ✓ Exact   | 9.52   | 1.01x          | ✓      |
| Decimal        | `1000000000000000` | ✓ Exact   | 20.51  | 2.17x          | ✓      |
| JSDecimal      | `1000000000000000` | ✓ Exact   | 23.51  | 2.49x          | ✓      |
| BigInt         | `1000000000000000` | ✓ Exact   | 25.39  | 2.69x          | ✓      |
| Rational64     | `1000000000000000` | ✓ Exact   | 42.11  | 4.46x          | ✓      |
| BigDecimal     | `1000000000000000` | ✓ Exact   | 60.03  | 6.35x          | ✓      |
| FaithfulNumber | `1000000000000000` | ✓ Exact   | 63.85  | 6.76x          | ✓      |
| RugFloat       | `1000000000000000` | ✓ Exact   | 90.31  | 9.56x          | ✓      |
| BigRational    | `1000000000000000` | ✓ Exact   | 719.35 | 76.11x         | ✓      |

### Test: `Mul::medium_mul`

**Operation:** `1000000 × 1000 = 1000000000`

**Expected:** `1000000000` (ExactInteger)

**Category:** Normal

#### Results by Accuracy

| Type           | Result       | Error     | ns/op  | Relative Speed | Status |
|----------------|--------------|-----------|--------|----------------|--------|
| F64            | `1000000000` | ✓ Exact   | 9.49   | 1.00x          | ✓      |
| I64            | `1000000000` | ✓ Exact   | 9.51   | 1.00x          | ✓      |
| Decimal        | `1000000000` | ✓ Exact   | 20.32  | 2.14x          | ✓      |
| JSDecimal      | `1000000000` | ✓ Exact   | 23.46  | 2.47x          | ✓      |
| BigInt         | `1000000000` | ✓ Exact   | 25.46  | 2.68x          | ✓      |
| Rational64     | `1000000000` | ✓ Exact   | 28.53  | 3.01x          | ✓      |
| FaithfulNumber | `1000000000` | ✓ Exact   | 53.43  | 5.63x          | ✓      |
| BigDecimal     | `1000000000` | ✓ Exact   | 60.17  | 6.34x          | ✓      |
| RugFloat       | `1000000000` | ✓ Exact   | 90.18  | 9.51x          | ✓      |
| BigRational    | `1000000000` | ✓ Exact   | 569.32 | 60.01x         | ✓      |

### Test: `Mul::small_times_large`

**Operation:** `0.1 × 10000000000000000 = 1000000000000000`

**Expected:** `1000000000000000` (ExactInteger)

**Category:** PrecisionCritical

#### Results by Accuracy

| Type           | Result             | Error     | ns/op  | Relative Speed | Status |
|----------------|--------------------|-----------|--------|----------------|--------|
| F64            | `1000000000000000` | ✓ Exact   | 9.53   | 1.01x          | ✓      |
| Decimal        | `1000000000000000` | ✓ Exact   | 21.25  | 2.25x          | ✓      |
| JSDecimal      | `1000000000000000` | ✓ Exact   | 23.75  | 2.51x          | ✓      |
| Rational64     | `1000000000000000` | ✓ Exact   | 44.07  | 4.66x          | ✓      |
| BigDecimal     | `1000000000000000` | ✓ Exact   | 61.68  | 6.52x          | ✓      |
| FaithfulNumber | `1000000000000000` | ✓ Exact   | 70.87  | 7.49x          | ✓      |
| RugFloat       | `1000000000000000` | ✓ Exact   | 94.13  | 9.95x          | ✓      |
| BigRational    | `1000000000000000` | ✓ Exact   | 744.75 | 78.76x         | ✓      |
| I64            | `0`                | 1.00e0    | 9.46   | 1.00x          | ≈      |
| BigInt         | `0`                | 1.00e0    | 12.73  | 1.35x          | ≈      |

### Test: `Mul::very_large_mul_1e20`

**Operation:** `10000000000 × 10000000000 = 100000000000000000000`

**Expected:** `100000000000000000000` (ExactInteger)

**Category:** Boundary

#### Results by Accuracy

| Type           | Result                | Error     | ns/op  | Relative Speed | Status     |
|----------------|-----------------------|-----------|--------|----------------|------------|
| F64            | `10000000...00000000` | ✓ Exact   | 9.44   | 1.00x          | ✓          |
| Decimal        | `10000000...00000000` | ✓ Exact   | 21.47  | 2.28x          | ✓          |
| JSDecimal      | `10000000...00000000` | ✓ Exact   | 24.21  | 2.57x          | ✓          |
| BigInt         | `10000000...00000000` | ✓ Exact   | 44.80  | 4.75x          | ✓          |
| FaithfulNumber | `10000000...00000000` | ✓ Exact   | 73.46  | 7.78x          | ✓          |
| BigDecimal     | `10000000...00000000` | ✓ Exact   | 81.31  | 8.62x          | ✓          |
| RugFloat       | `10000000...00000000` | ✓ Exact   | 94.79  | 10.05x         | ✓          |
| BigRational    | `10000000...00000000` | ✓ Exact   | 828.06 | 87.76x         | ✓          |
| I64            | N/A                   | -         | -      | -              | ❌ Failed   |
| Rational64     | N/A                   | -         | -      | -              | ❌ Failed   |

### Test: `Sin::sin_0`

**Operation:** `sin(0) = 0`

**Expected:** `0` (ExactInteger)

**Category:** Normal

#### Results by Accuracy

| Type           | Result | Error     | ns/op  | Relative Speed | Status     |
|----------------|--------|-----------|--------|----------------|------------|
| F64            | `0`    | ✓ Exact   | 15.31  | 1.00x          | ✓          |
| JSDecimal      | `0`    | ✓ Exact   | 20.44  | 1.34x          | ✓          |
| RugFloat       | `0`    | ✓ Exact   | 30.96  | 2.02x          | ✓          |
| FaithfulNumber | `0`    | ✓ Exact   | 133.99 | 8.75x          | ✓          |
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
| RugFloat       | `9.999999...96617e-1` | ✓ Exact   | 5462.55 | 333.02x        | ✓          |
| FaithfulNumber | `0.999999...09776625` | 1.34e-78  | 4249.19 | 259.05x        | ≈          |
| F64            | `0.999999...99999911` | 4.61e-18  | 16.40   | 1.00x          | ≈          |
| JSDecimal      | `0.999999...99999991` | 1.05e-16  | 139.52  | 8.51x          | ≈          |
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
| RugFloat       | `1.414213...09993586` | ✓ Exact   | 206.64  | 21.65x         | ✓          |
| BigDecimal     | `1.414213...27641573` | 1.87e-100 | 2092.99 | 219.28x        | ≈          |
| FaithfulNumber | `1.414213...78462102` | 3.56e-78  | 1232.87 | 129.17x        | ≈          |
| JSDecimal      | `1.414213...16887242` | 6.86e-30  | 1187.14 | 124.38x        | ≈          |
| BigRational    | `28284271...00000000` | 3.45e-17  | 390.09  | 40.87x         | ≈          |
| F64            | `1.414213...23730951` | 3.62e-17  | 9.54    | 1.00x          | ≈          |
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
| F64            | `2`    | ✓ Exact   | 9.52    | 1.00x          | ✓          |
| FaithfulNumber | `2`    | ✓ Exact   | 20.57   | 2.16x          | ✓          |
| BigRational    | `2`    | ✓ Exact   | 143.73  | 15.10x         | ✓          |
| JSDecimal      | `2`    | ✓ Exact   | 152.82  | 16.05x         | ✓          |
| RugFloat       | `2`    | ✓ Exact   | 231.49  | 24.32x         | ✓          |
| BigDecimal     | `2`    | ✓ Exact   | 2121.41 | 222.85x        | ✓          |
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
| F64            | `NaN`  | ✓ Exact   | 9.53  | 1.00x          | ✓          |
| JSDecimal      | `NaN`  | ✓ Exact   | 17.46 | 1.83x          | ✓          |
| FaithfulNumber | `NaN`  | ✓ Exact   | 17.91 | 1.88x          | ✓          |
| RugFloat       | `-NaN` | -         | 31.05 | 3.26x          | ≈          |
| BigRational    | `0`    | -         | 41.31 | 4.34x          | ≈          |
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
| I64            | `2`    | ✓ Exact   | 9.51   | 1.00x          | ✓      |
| F64            | `2`    | ✓ Exact   | 9.52   | 1.00x          | ✓      |
| Rational64     | `2`    | ✓ Exact   | 17.51  | 1.84x          | ✓      |
| Decimal        | `2`    | ✓ Exact   | 20.16  | 2.12x          | ✓      |
| JSDecimal      | `2`    | ✓ Exact   | 22.91  | 2.41x          | ✓      |
| BigInt         | `2`    | ✓ Exact   | 23.04  | 2.42x          | ✓      |
| FaithfulNumber | `2`    | ✓ Exact   | 37.69  | 3.96x          | ✓      |
| BigDecimal     | `2`    | ✓ Exact   | 44.04  | 4.63x          | ✓      |
| RugFloat       | `2`    | ✓ Exact   | 53.97  | 5.67x          | ✓      |
| BigRational    | `2`    | ✓ Exact   | 156.32 | 16.43x         | ✓      |

### Test: `Sub::catastrophic_cancellation`

**Operation:** `10000000000000001 - 10000000000000000 = 1`

**Expected:** `1` (ExactInteger)

**Category:** PrecisionCritical

#### Results by Accuracy

| Type           | Result | Error     | ns/op | Relative Speed | Status |
|----------------|--------|-----------|-------|----------------|--------|
| I64            | `1`    | ✓ Exact   | 9.52  | 1.00x          | ✓      |
| Rational64     | `1`    | ✓ Exact   | 16.22 | 1.70x          | ✓      |
| Decimal        | `1`    | ✓ Exact   | 21.14 | 2.22x          | ✓      |
| BigInt         | `1`    | ✓ Exact   | 22.90 | 2.40x          | ✓      |
| JSDecimal      | `1`    | ✓ Exact   | 23.83 | 2.50x          | ✓      |
| FaithfulNumber | `1`    | ✓ Exact   | 34.04 | 3.57x          | ✓      |
| BigDecimal     | `1`    | ✓ Exact   | 43.81 | 4.60x          | ✓      |
| RugFloat       | `1`    | ✓ Exact   | 56.23 | 5.91x          | ✓      |
| BigRational    | `1`    | ✓ Exact   | 76.45 | 8.03x          | ✓      |
| F64            | `0`    | 1.00e0    | 9.55  | 1.00x          | ≈      |

### Test: `Sub::extreme_1e50`

**Operation:** `300000000000000000000000000000000000000000000000000 - 100000000000000000000000000000000000000000000000000 = 200000000000000000000000000000000000000000000000000`

**Expected:** `200000000000000000000000000000000000000000000000000` (ExactInteger)

**Category:** Boundary

#### Results by Accuracy

| Type           | Result                | Error     | ns/op  | Relative Speed | Status      |
|----------------|-----------------------|-----------|--------|----------------|-------------|
| BigInt         | `20000000...00000000` | ✓ Exact   | 23.88  | 2.51x          | ✓           |
| BigDecimal     | `20000000...00000000` | ✓ Exact   | 46.83  | 4.92x          | ✓           |
| RugFloat       | `20000000...00000000` | ✓ Exact   | 55.14  | 5.79x          | ✓           |
| FaithfulNumber | `20000000...00000000` | ✓ Exact   | 87.13  | 9.15x          | ✓           |
| BigRational    | `20000000...00000000` | ✓ Exact   | 764.45 | 80.30x         | ✓           |
| F64            | `19999999...00000000` | 1.31e-16  | 9.52   | 1.00x          | ≈           |
| JSDecimal      | `NaN`                 | NaN       | 16.06  | 1.69x          | ≈           |
| Decimal        | `0`                   | 1.00e0    | 21.12  | 2.22x          | ≈           |
| I64            | N/A                   | -         | -      | -              | ⚠ Skipped   |
| Rational64     | N/A                   | -         | -      | -              | ⚠ Skipped   |

### Test: `Sub::large_1e15`

**Operation:** `3000000000000000 - 1000000000000000 = 2000000000000000`

**Expected:** `2000000000000000` (ExactInteger)

**Category:** Boundary

#### Results by Accuracy

| Type           | Result             | Error     | ns/op  | Relative Speed | Status |
|----------------|--------------------|-----------|--------|----------------|--------|
| I64            | `2000000000000000` | ✓ Exact   | 9.53   | 1.00x          | ✓      |
| F64            | `2000000000000000` | ✓ Exact   | 9.60   | 1.01x          | ✓      |
| Decimal        | `2000000000000000` | ✓ Exact   | 21.06  | 2.21x          | ✓      |
| BigInt         | `2000000000000000` | ✓ Exact   | 22.98  | 2.41x          | ✓      |
| JSDecimal      | `2000000000000000` | ✓ Exact   | 23.86  | 2.50x          | ✓      |
| Rational64     | `2000000000000000` | ✓ Exact   | 27.41  | 2.88x          | ✓      |
| BigDecimal     | `2000000000000000` | ✓ Exact   | 43.69  | 4.58x          | ✓      |
| FaithfulNumber | `2000000000000000` | ✓ Exact   | 53.59  | 5.62x          | ✓      |
| RugFloat       | `2000000000000000` | ✓ Exact   | 60.82  | 6.38x          | ✓      |
| BigRational    | `2000000000000000` | ✓ Exact   | 351.84 | 36.92x         | ✓      |

### Test: `Sub::medium_1e6`

**Operation:** `3000000 - 1000000 = 2000000`

**Expected:** `2000000` (ExactInteger)

**Category:** Normal

#### Results by Accuracy

| Type           | Result    | Error     | ns/op  | Relative Speed | Status |
|----------------|-----------|-----------|--------|----------------|--------|
| F64            | `2000000` | ✓ Exact   | 9.51   | 1.00x          | ✓      |
| I64            | `2000000` | ✓ Exact   | 9.59   | 1.01x          | ✓      |
| Rational64     | `2000000` | ✓ Exact   | 18.61  | 1.96x          | ✓      |
| Decimal        | `2000000` | ✓ Exact   | 20.20  | 2.12x          | ✓      |
| BigInt         | `2000000` | ✓ Exact   | 22.97  | 2.41x          | ✓      |
| JSDecimal      | `2000000` | ✓ Exact   | 23.02  | 2.42x          | ✓      |
| FaithfulNumber | `2000000` | ✓ Exact   | 40.34  | 4.24x          | ✓      |
| BigDecimal     | `2000000` | ✓ Exact   | 43.51  | 4.57x          | ✓      |
| RugFloat       | `2000000` | ✓ Exact   | 60.73  | 6.38x          | ✓      |
| BigRational    | `2000000` | ✓ Exact   | 215.12 | 22.61x         | ✓      |

### Test: `Sub::medium_1e9`

**Operation:** `3000000000 - 1000000000 = 2000000000`

**Expected:** `2000000000` (ExactInteger)

**Category:** Boundary

#### Results by Accuracy

| Type           | Result       | Error     | ns/op  | Relative Speed | Status |
|----------------|--------------|-----------|--------|----------------|--------|
| I64            | `2000000000` | ✓ Exact   | 9.52   | 1.00x          | ✓      |
| F64            | `2000000000` | ✓ Exact   | 9.57   | 1.01x          | ✓      |
| Decimal        | `2000000000` | ✓ Exact   | 20.20  | 2.12x          | ✓      |
| Rational64     | `2000000000` | ✓ Exact   | 21.72  | 2.28x          | ✓      |
| JSDecimal      | `2000000000` | ✓ Exact   | 22.84  | 2.40x          | ✓      |
| BigInt         | `2000000000` | ✓ Exact   | 23.27  | 2.45x          | ✓      |
| BigDecimal     | `2000000000` | ✓ Exact   | 44.09  | 4.63x          | ✓      |
| FaithfulNumber | `2000000000` | ✓ Exact   | 46.73  | 4.91x          | ✓      |
| RugFloat       | `2000000000` | ✓ Exact   | 60.94  | 6.40x          | ✓      |
| BigRational    | `2000000000` | ✓ Exact   | 282.60 | 29.70x         | ✓      |

### Test: `Sub::near_i64_max`

**Operation:** `9223372036854776000 - 1000 = 9223372036854775000`

**Expected:** `9223372036854775000` (ExactInteger)

**Category:** Boundary

#### Results by Accuracy

| Type           | Result                | Error     | ns/op  | Relative Speed | Status      |
|----------------|-----------------------|-----------|--------|----------------|-------------|
| Decimal        | `92233720...54775000` | ✓ Exact   | 21.13  | 2.22x          | ✓           |
| BigInt         | `92233720...54775000` | ✓ Exact   | 23.06  | 2.42x          | ✓           |
| JSDecimal      | `92233720...54775000` | ✓ Exact   | 23.87  | 2.50x          | ✓           |
| BigDecimal     | `92233720...54775000` | ✓ Exact   | 43.35  | 4.55x          | ✓           |
| RugFloat       | `92233720...54775000` | ✓ Exact   | 61.37  | 6.44x          | ✓           |
| FaithfulNumber | `92233720...54775000` | ✓ Exact   | 65.97  | 6.92x          | ✓           |
| BigRational    | `92233720...54775000` | ✓ Exact   | 716.35 | 75.17x         | ✓           |
| F64            | `92233720...54775000` | 2.34e-17  | 9.53   | 1.00x          | ≈           |
| I64            | N/A                   | -         | -      | -              | ⚠ Skipped   |
| Rational64     | N/A                   | -         | -      | -              | ⚠ Skipped   |

### Test: `Sub::negative_result`

**Operation:** `3 - 5 = -2`

**Expected:** `-2` (ExactInteger)

**Category:** Normal

#### Results by Accuracy

| Type           | Result | Error     | ns/op  | Relative Speed | Status |
|----------------|--------|-----------|--------|----------------|--------|
| F64            | `-2`   | ✓ Exact   | 9.54   | 1.00x          | ✓      |
| I64            | `-2`   | ✓ Exact   | 9.55   | 1.00x          | ✓      |
| Rational64     | `-2`   | ✓ Exact   | 17.36  | 1.82x          | ✓      |
| Decimal        | `-2`   | ✓ Exact   | 20.46  | 2.15x          | ✓      |
| JSDecimal      | `-2`   | ✓ Exact   | 22.81  | 2.39x          | ✓      |
| BigInt         | `-2`   | ✓ Exact   | 23.19  | 2.43x          | ✓      |
| FaithfulNumber | `-2`   | ✓ Exact   | 38.04  | 3.99x          | ✓      |
| BigDecimal     | `-2`   | ✓ Exact   | 44.00  | 4.61x          | ✓      |
| RugFloat       | `-2`   | ✓ Exact   | 56.97  | 5.97x          | ✓      |
| BigRational    | `-2`   | ✓ Exact   | 153.30 | 16.08x         | ✓      |

### Test: `Sub::very_large_1e18`

**Operation:** `3000000000000000000 - 1000000000000000000 = 2000000000000000000`

**Expected:** `2000000000000000000` (ExactInteger)

**Category:** Boundary

#### Results by Accuracy

| Type           | Result                | Error     | ns/op  | Relative Speed | Status |
|----------------|-----------------------|-----------|--------|----------------|--------|
| F64            | `20000000...00000000` | ✓ Exact   | 9.51   | 1.00x          | ✓      |
| I64            | `20000000...00000000` | ✓ Exact   | 9.53   | 1.00x          | ✓      |
| Decimal        | `20000000...00000000` | ✓ Exact   | 21.20  | 2.23x          | ✓      |
| BigInt         | `20000000...00000000` | ✓ Exact   | 23.24  | 2.44x          | ✓      |
| JSDecimal      | `20000000...00000000` | ✓ Exact   | 23.88  | 2.51x          | ✓      |
| Rational64     | `20000000...00000000` | ✓ Exact   | 30.32  | 3.19x          | ✓      |
| FaithfulNumber | `20000000...00000000` | ✓ Exact   | 37.19  | 3.91x          | ✓      |
| BigDecimal     | `20000000...00000000` | ✓ Exact   | 43.61  | 4.59x          | ✓      |
| RugFloat       | `20000000...00000000` | ✓ Exact   | 60.67  | 6.38x          | ✓      |
| BigRational    | `20000000...00000000` | ✓ Exact   | 390.45 | 41.05x         | ✓      |

### Test: `Sub::very_large_1e20`

**Operation:** `300000000000000000000 - 100000000000000000000 = 200000000000000000000`

**Expected:** `200000000000000000000` (ExactInteger)

**Category:** Boundary

#### Results by Accuracy

| Type           | Result                | Error     | ns/op  | Relative Speed | Status      |
|----------------|-----------------------|-----------|--------|----------------|-------------|
| F64            | `20000000...00000000` | ✓ Exact   | 9.57   | 1.00x          | ✓           |
| Decimal        | `20000000...00000000` | ✓ Exact   | 21.31  | 2.23x          | ✓           |
| BigInt         | `20000000...00000000` | ✓ Exact   | 23.27  | 2.43x          | ✓           |
| JSDecimal      | `20000000...00000000` | ✓ Exact   | 23.90  | 2.50x          | ✓           |
| FaithfulNumber | `20000000...00000000` | ✓ Exact   | 37.07  | 3.88x          | ✓           |
| BigDecimal     | `20000000...00000000` | ✓ Exact   | 45.88  | 4.80x          | ✓           |
| RugFloat       | `20000000...00000000` | ✓ Exact   | 61.22  | 6.40x          | ✓           |
| BigRational    | `20000000...00000000` | ✓ Exact   | 415.91 | 43.47x         | ✓           |
| I64            | N/A                   | -         | -      | -              | ⚠ Skipped   |
| Rational64     | N/A                   | -         | -      | -              | ⚠ Skipped   |


