10 PRINT "Testing OPTION BASE 0 (Default)"
20 PRINT "================================"
30 PRINT ""
40 OPTION BASE 0
50 DIM A(3)
60 PRINT "DIM A(3) with OPTION BASE 0"
70 PRINT "Should have indices 0, 1, 2, 3 (4 elements)"
80 PRINT ""
90 LET A(0) = 100
100 LET A(1) = 200
110 LET A(2) = 300
120 LET A(3) = 400
130 PRINT "A(0) = "
140 PRINT A(0)
150 PRINT "A(1) = "
160 PRINT A(1)
170 PRINT "A(2) = "
180 PRINT A(2)
190 PRINT "A(3) = "
200 PRINT A(3)
210 PRINT ""
220 PRINT "Test complete!"
230 END
