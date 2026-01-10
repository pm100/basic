# READ/DATA/RESTORE Statements - Implementation Summary

## Overview

Successfully implemented READ, DATA, and RESTORE statements for the BASIC interpreter. These statements allow programs to store constant data and read it sequentially.

## New Statements

### 1. DATA Statement
Stores constant values (numbers or strings) that can be read by READ statements.

**Syntax:**
```basic
DATA value1, value2, value3, ...
```

**Examples:**
```basic
10 DATA 10, 20, 30, 40, 50
20 DATA "Alice", "Bob", "Charlie"
30 DATA 3.14159, 2.71828, "PI", "E"
```

**Features:**
- Can contain numbers (integer or floating-point)
- Can contain strings (enclosed in quotes)
- Values are separated by commas
- Multiple DATA statements in a program are treated as one continuous list
- DATA statements can appear anywhere in the program
- All DATA values are collected in order when the program starts

### 2. READ Statement
Reads values from DATA statements and assigns them to variables.

**Syntax:**
```basic
READ var1, var2, var3, ...
```

**Examples:**
```basic
10 DATA 100, 200, 300
20 READ A, B, C
```

**Features:**
- Reads values sequentially from the DATA pool
- Can read multiple variables in one statement
- Automatically advances the data pointer after each value
- Type is determined by the DATA value (number or string)
- Error message if attempting to read past end of data

### 3. RESTORE Statement
Resets the data pointer to the beginning of the DATA pool.

**Syntax:**
```basic
RESTORE
```

**Examples:**
```basic
10 DATA 1, 2, 3
20 READ A, B, C
30 RESTORE
40 READ X, Y, Z    ' X=1, Y=2, Z=3 (same data read again)
```

**Features:**
- Resets pointer to the first DATA value
- Allows re-reading the same data multiple times
- Useful in loops and complex calculations

## Implementation Details

### Changes Made

1. **Token Enum Updates:**
   - Added `Comma` token type for parsing comma-separated lists
   - Added DATA, READ, and RESTORE to the keywords array

2. **Statement Enum Updates:**
   - Added `Data { values: Vec<Token> }` variant
   - Added `Read { vars: Vec<String> }` variant
   - Added `Restore` variant

3. **Interpreter Struct Updates:**
   - Added `data_values: Vec<Value>` - stores all DATA values
   - Added `data_pointer: usize` - tracks current position in data

4. **Tokenizer:**
   - Recognizes comma (`,`) as a separator token
   - Handles DATA, READ, and RESTORE keywords

5. **Parser:**
   - Parses DATA statements with comma-separated values
   - Parses READ statements with comma-separated variable names
   - Parses RESTORE statements

6. **Execution:**
   - **Program initialization:** Collects all DATA values before execution
   - **READ execution:** Assigns data values to variables and advances pointer
   - **RESTORE execution:** Resets data pointer to 0
   - **DATA execution:** No-op during execution (processed at initialization)

### Error Handling

- **Out of DATA:** When READ attempts to read past the end of available DATA values, displays error message and stops reading remaining variables in that READ statement

## Examples

### Basic Usage
```basic
10 DATA 10, 20, 30
20 READ A, B, C
30 PRINT A
40 PRINT B
50 PRINT C
```
Output:
```
10
20
30
```

### Mixed Types
```basic
10 DATA "Alice", 25, "Bob", 30
20 READ NAME1, AGE1, NAME2, AGE2
30 PRINT NAME1
40 PRINT AGE1
50 PRINT NAME2
60 PRINT AGE2
```
Output:
```
Alice
25
Bob
30
```

### Using RESTORE
```basic
10 DATA 100, 200, 300
20 READ A, B, C
30 PRINT "First read: "
40 PRINT A, B, C
50 RESTORE
60 READ X, Y, Z
70 PRINT "Second read: "
80 PRINT X, Y, Z
```

### Loop with READ
```basic
10 DATA 85, 92, 78, 95, 88
20 FOR I = 1 TO 5
30 READ SCORE
40 PRINT SCORE
50 NEXT I
```

### Multiple DATA Statements
```basic
10 DATA 1, 2, 3
20 DATA 4, 5, 6
30 DATA 7, 8, 9
40 FOR I = 1 TO 9
50 READ N
60 PRINT N
70 NEXT I
```
Output: Prints 1 through 9

## Test Files Created

1. **test_read_data.bas** - Basic READ/DATA functionality
2. **test_restore.bas** - RESTORE statement testing
3. **test_out_of_data.bas** - Error handling when reading past end
4. **demo_employees.bas** - Employee database example
5. **demo_statistics.bas** - Statistics calculation with RESTORE

## Use Cases

1. **Database Records:** Store and process tabular data
2. **Configuration:** Store program constants
3. **Test Data:** Easily change test inputs
4. **Lookup Tables:** Store reference data
5. **Statistical Analysis:** Process datasets multiple times

## Technical Notes

- DATA values are stored in order of line numbers, not order of appearance in code
- The data pointer is global to the entire program
- RESTORE always resets to the very first DATA value
- Empty DATA statements are valid but contribute no values
- Commas are required separators (unlike some BASIC dialects that allow spaces)
- String values in DATA must be quoted

## Compatibility

This implementation follows classic BASIC behavior:
- ✅ Sequential data reading
- ✅ RESTORE resets to beginning
- ✅ Mixed numeric and string data
- ✅ Multiple DATA statements
- ✅ Error on out-of-data condition

## Completion Status

✅ All READ/DATA/RESTORE features implemented
✅ Comma tokenization working
✅ Data collection at program start
✅ Sequential reading with pointer management
✅ RESTORE functionality
✅ Error handling for out-of-data
✅ Works with both file and interactive modes
✅ Comprehensive test coverage
