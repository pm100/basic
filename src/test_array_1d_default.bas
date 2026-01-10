10 PRINT "Testing 1D Arrays (Default Dimensions)"
20 PRINT "========================================"
30 PRINT ""
40 PRINT "Setting array values without DIM (uses 0-10):"
50 LET A(0) = 100
60 LET A(1) = 200
70 LET A(2) = 300
80 LET A(5) = 500
90 LET A(10) = 1000
100 PRINT ""
110 PRINT "Reading array values:"
120 PRINT "A(0) = "
130 PRINT A(0)
140 PRINT "A(1) = "
150 PRINT A(1)
160 PRINT "A(2) = "
170 PRINT A(2)
180 PRINT "A(5) = "
190 PRINT A(5)
200 PRINT "A(10) = "
210 PRINT A(10)
220 PRINT ""
230 PRINT "Using arrays in expressions:"
240 LET SUM = A(0) + A(1) + A(2)
250 PRINT "A(0) + A(1) + A(2) = "
260 PRINT SUM
270 PRINT ""
280 PRINT "Test complete!"
290 END
