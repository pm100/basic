1 REM SKIP
10 REM Test WRITE statement
20 A$ = "Hello"
30 B$ = "World"
40 X = 42
50 Y = 3.14
60 PRINT "Using PRINT:"
70 PRINT A$, B$, X, Y
80 PRINT
90 PRINT "Using WRITE:"
100 WRITE A$, B$, X, Y
110 PRINT
120 PRINT "WRITE to file test:"
130 OPEN "writetest.txt" FOR OUTPUT AS #1
140 WRITE #1, A$, B$, X, Y
150 WRITE #1, "Another", "Line", 123
160 CLOSE #1
170 PRINT "File written - check writetest.txt"
180 END

