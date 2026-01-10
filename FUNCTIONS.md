# BASIC Numeric Functions - Implementation Summary

## Overview
The BASIC interpreter now supports all 10 standard numeric functions with full floating-point arithmetic (f64) and the exponentiation operator (^).

## Implemented Functions

### 1. ABS(x) - Absolute Value
Returns the absolute value of x.
```basic
10 LET X = -5.5
20 PRINT ABS(X)  ' Outputs: 5.5
```

### 2. ATN(x) - Arctangent
Returns the arctangent of x in radians.
```basic
10 LET X = 1
20 PRINT ATN(X)  ' Outputs: 0.7853981633974483 (π/4)
```

### 3. COS(x) - Cosine
Returns the cosine of x (x is in radians).
```basic
10 LET X = 0
20 PRINT COS(X)  ' Outputs: 1
```

### 4. EXP(x) - Exponential
Returns e raised to the power of x (e^x).
```basic
10 LET X = 1
20 PRINT EXP(X)  ' Outputs: 2.718281828459045
```

### 5. INT(x) - Integer Part
Returns the integer part of x (truncating toward 0).
```basic
10 LET X = 7.9
20 PRINT INT(X)  ' Outputs: 7
30 LET Y = -3.7
40 PRINT INT(Y)  ' Outputs: -3
```

### 6. LOG(x) - Natural Logarithm
Returns the natural logarithm of x.
- Error handling: Displays error for x <= 0 and returns 0
```basic
10 LET X = 2.71828
20 PRINT LOG(X)  ' Outputs: 0.999999327347282 (approximately 1)
```

### 7. RND(x) - Random Number
Returns a random number between 0 and 1.
- The argument x is required but ignored (dummy argument per BASIC spec)
```basic
10 PRINT RND(0)  ' Outputs: random value like 0.9433536281333635
20 PRINT RND(0)  ' Outputs: different random value
```

### 8. SIN(x) - Sine
Returns the sine of x (x is in radians).
```basic
10 LET X = 1.5708
20 PRINT SIN(X)  ' Outputs: ~1 (sin(π/2))
```

### 9. SQR(x) - Square Root
Returns the square root of x.
- Error handling: For negative x, displays error and returns sqrt(|x|)
```basic
10 LET X = 16
20 PRINT SQR(X)  ' Outputs: 4
30 LET Y = -9
40 PRINT SQR(Y)  ' Error message, outputs: 3
```

### 10. TAN(x) - Tangent
Returns the tangent of x (x is in radians).
```basic
10 LET X = 0.7854
20 PRINT TAN(X)  ' Outputs: ~1 (tan(π/4))
```

## Exponentiation Operator (^)

The ^ operator can be used for exponentiation as an alternative to EXP.
- Right-associative: 2^3^2 = 2^(3^2) = 2^9 = 512

```basic
10 LET X = 3
20 LET Y = 2
30 PRINT X ^ Y    ' Outputs: 9
40 PRINT 2 ^ 3 ^ 2  ' Outputs: 512 (right-associative)
```

## Floating-Point Arithmetic

All arithmetic operations now use 64-bit floating-point (f64):
- Supports decimal numbers: 3.14159, 2.5, 0.001
- Valid range: approximately 10^-308 to 10^308 (f64 range)
- Note: The specified BASIC range of 10^-256 to 10^256 is well within f64 capabilities

```basic
10 LET PI = 3.14159
20 LET R = 2.5
30 PRINT PI * R * R  ' Outputs: 19.6349375
```

## Function Nesting and Expressions

Functions can be nested and combined in complex expressions:

```basic
10 LET X = -2.5
20 PRINT ABS(SIN(ABS(X)))  ' Nested functions work correctly
30 PRINT SQR(16) + COS(0) * 5  ' Combined with arithmetic
```

## Error Handling

### LOG Function Errors
- Returns 0 and displays error message for x <= 0:
  ```
  Error: LOG of non-positive number (x)
  ```

### SQR Function Errors
- Returns sqrt(|x|) and displays error message for x < 0:
  ```
  Error: SQR of negative number (x), using absolute value
  ```

## Technical Implementation Details

### Changes Made:
1. **Value enum**: Changed from `i32` to `f64` for the Number variant
2. **Token enum**: Added `Function(String)` variant for built-in functions
3. **Tokenizer**: 
   - Recognizes function names (ABS, ATN, COS, EXP, INT, LOG, RND, SIN, SQR, TAN)
   - Supports ^ operator for exponentiation
   - Handles decimal numbers (e.g., 3.14)
4. **Expression Evaluator**:
   - Handles function calls with parentheses
   - Implements proper operator precedence: ^ > * / > + -
   - Right-associative exponentiation
   - Recursive function and parenthesis processing
5. **Interpreter**: Added RNG (Random Number Generator) for RND function
6. **Dependencies**: Added `rand = "0.8"` to Cargo.toml

## Examples

See the test files in the `src/` directory:
- `test_functions.bas` - Tests all 10 functions
- `test_errors.bas` - Tests error handling and complex expressions
- `test_interactive.bas` - Interactive example with INPUT

## Interactive Mode Usage

```
> LET X = 5
> PRINT SQR(X)
2.23606797749979
> PRINT X ^ 2
25
> PRINT SIN(1.5708)
0.9999999999932537
```
