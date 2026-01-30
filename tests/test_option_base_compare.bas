10 PRINT "Comparing OPTION BASE 0 vs 1"
20 PRINT "============================="
30 PRINT ""
40 PRINT "Part 1: Default (OPTION BASE 0)"
50 DIM A(3)
60 LET A(0) = 10
70 LET A(1) = 20
80 LET A(2) = 30
90 LET A(3) = 40
100 PRINT "A has 4 elements: A(0) to A(3)"
110 PRINT "Sum of A(0) + A(3) = "
120 PRINT A(0) + A(3)
130 PRINT ""
140 PRINT "Part 2: OPTION BASE 1"
150 OPTION BASE 1
160 DIM B(3)
170 LET B(1) = 10
180 LET B(2) = 20
190 LET B(3) = 30
200 PRINT "B has 3 elements: B(1) to B(3)"
210 PRINT "Sum of B(1) + B(3) = "
220 PRINT B(1) + B(3)
230 PRINT ""
240 PRINT "Note: A still works with 0-based indexing"
250 PRINT "because it was created before OPTION BASE 1"
260 PRINT "A(0) = "
270 PRINT A(0)
280 PRINT ""
290 PRINT "Test complete!"
300 END
