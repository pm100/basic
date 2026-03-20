10 PRINT "Testing 10D Array"
20 PRINT "================="
30 PRINT ""
40 DIM X(1,1,1,1,1,1,1,1,1,1)
50 PRINT "Created 10-dimensional array"
60 PRINT ""
70 LET X(0,0,0,0,0,0,0,0,0,0) = 1234
80 LET X(1,1,1,1,1,1,1,1,1,1) = 5678
90 LET X(0,1,0,1,0,1,0,1,0,1) = 9999
100 PRINT "Set three values in 10D space"
110 PRINT ""
120 PRINT "X(0,0,0,0,0,0,0,0,0,0) = "
130 PRINT X(0,0,0,0,0,0,0,0,0,0)
140 PRINT "X(1,1,1,1,1,1,1,1,1,1) = "
150 PRINT X(1,1,1,1,1,1,1,1,1,1)
160 PRINT "X(0,1,0,1,0,1,0,1,0,1) = "
170 PRINT X(0,1,0,1,0,1,0,1,0,1)
180 PRINT ""
190 PRINT "Test complete!"
200 END
