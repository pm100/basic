1 REM SKIP
10 PRINT "Testing CLS (Clear Screen) Statement"
20 PRINT "====================================="
30 PRINT ""
40 PRINT "This screen will be cleared in 2 seconds..."
50 PRINT ""
60 FOR I = 1 TO 10
70 PRINT "Line "
80 PRINT I
90 NEXT I
100 PRINT ""
110 PRINT "Press Enter to clear screen..."
120 INPUT DUMMY
130 CLS
140 PRINT "Screen cleared!"
150 PRINT ""
160 PRINT "Now displaying a clean menu:"
170 PRINT ""
180 PRINT "  MAIN MENU"
190 PRINT "  ========="
200 PRINT ""
210 PRINT "  1. Option One"
220 PRINT "  2. Option Two"
230 PRINT "  3. Option Three"
240 PRINT "  4. Exit"
250 PRINT ""
260 PRINT "CLS test complete!"
270 END

