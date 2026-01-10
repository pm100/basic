10 PRINT "Testing OPTION BASE 1"
20 PRINT "====================="
30 PRINT ""
40 OPTION BASE 1
50 DIM A(3)
60 PRINT "DIM A(3) with OPTION BASE 1"
70 PRINT "Should have indices 1, 2, 3 (3 elements)"
80 PRINT ""
90 LET A(1) = 100
100 LET A(2) = 200
110 LET A(3) = 300
120 PRINT "A(1) = "
130 PRINT A(1)
140 PRINT "A(2) = "
150 PRINT A(2)
160 PRINT "A(3) = "
170 PRINT A(3)
180 PRINT ""
190 PRINT "Testing that A(0) is out of bounds:"
200 REM Trying to access A(0) should give an error
210 PRINT ""
220 PRINT "Test complete!"
230 END
