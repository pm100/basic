10 REM Comprehensive test of all new features
20 PRINT "=== Testing SPC(n) ==="
30 PRINT "A"; SPC(10); "B"; SPC(5); "C"
40 PRINT
50 PRINT "=== Testing LTRIM$, RTRIM$, TRIM$ ==="
60 PRINT "Original: ["; "  spaces  "; "]"
70 PRINT "LTRIM$:   ["; LTRIM$("  spaces  "); "]"
80 PRINT "RTRIM$:   ["; RTRIM$("  spaces  "); "]"
90 PRINT "TRIM$:    ["; TRIM$("  spaces  "); "]"
100 PRINT
110 PRINT "=== Testing LINE INPUT ==="
120 A$ = "Hello, World"
130 PRINT "Would ask for: LINE INPUT (skipping for automated test)"
140 PRINT
150 PRINT "=== Testing File I/O ==="
160 OPEN "testdata.txt" FOR OUTPUT AS #1
170 PRINT #1, "First line"
180 PRINT #1, "Second"; SPC(3); "line"
190 PRINT #1, "Numbers:", 123, 456, 789
200 CLOSE #1
210 PRINT "File written successfully"
220 PRINT
230 PRINT "=== All tests complete! ==="
240 END
