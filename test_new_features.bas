10 REM Comprehensive test of all new features
20 PRINT "=== Testing Multiple Statements Per Line ==="
30 A = 10 : B = 20 : C = A + B : PRINT "A + B ="; C
40 X$ = "Test" : Y$ = "Works" : PRINT X$; " "; Y$
50 PRINT
60 PRINT "=== Testing SGN Function ==="
70 PRINT "SGN(5) ="; SGN(5); " (should be 1)"
80 PRINT "SGN(0) ="; SGN(0); " (should be 0)"
90 PRINT "SGN(-3) ="; SGN(-3); " (should be -1)"
100 PRINT
110 PRINT "=== Testing LINE INPUT # ==="
120 OPEN "temp.txt" FOR OUTPUT AS #1
130 PRINT #1, "Data line 1, with commas"
140 PRINT #1, "Data line 2"
150 CLOSE #1
160 OPEN "temp.txt" FOR INPUT AS #1
170 LINE INPUT #1, L1$
180 LINE INPUT #1, L2$
190 CLOSE #1
200 PRINT "Read from file:"
210 PRINT "  Line 1: "; L1$
220 PRINT "  Line 2: "; L2$
230 PRINT
240 PRINT "=== All Tests Complete! ==="
250 END
