10 PRINT "Testing OPTION BASE with 2D Arrays"
20 PRINT "==================================="
30 PRINT ""
40 OPTION BASE 1
50 DIM M(2,2)
60 PRINT "DIM M(2,2) with OPTION BASE 1"
70 PRINT "Should have indices (1,1) to (2,2)"
80 PRINT ""
90 LET M(1,1) = 11
100 LET M(1,2) = 12
110 LET M(2,1) = 21
120 LET M(2,2) = 22
130 PRINT "M(1,1) = "
140 PRINT M(1,1)
150 PRINT "M(1,2) = "
160 PRINT M(1,2)
170 PRINT "M(2,1) = "
180 PRINT M(2,1)
190 PRINT "M(2,2) = "
200 PRINT M(2,2)
210 PRINT ""
220 PRINT "Test complete!"
230 END
