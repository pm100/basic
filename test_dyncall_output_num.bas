10 REM Test dyncall with output numeric (pointer) arguments
20 REM sscanf with *i32: parse an integer directly into a numeric variable
30 DEF XFN sscanf("msvcrt.dll|sscanf|cstr,cstr,*i32|i32|fixargs=2")
40 LET x = 0
50 LET n = FN sscanf("42", "%d", x)
60 PRINT "sscanf returned: "; n
70 PRINT "Parsed integer: "; x
80 REM Parse a second value
90 LET y = 0
100 LET n2 = FN sscanf("123", "%d", y)
110 PRINT "sscanf returned: "; n2
120 PRINT "Parsed integer: "; y
130 END
