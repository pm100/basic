10 PRINT "Testing Auto-initialization with OPTION BASE"
20 PRINT "============================================="
30 PRINT ""
40 PRINT "Test 1: Auto-init with default BASE 0"
50 LET X(5) = 100
60 PRINT "Set X(5) without DIM (auto-initialized 0-10)"
70 PRINT "X(0) = "
80 PRINT X(0)
90 PRINT "X(5) = "
100 PRINT X(5)
110 PRINT ""
120 PRINT "Test 2: Auto-init with OPTION BASE 1"
130 OPTION BASE 1
140 LET Y(5) = 200
150 PRINT "Set Y(5) without DIM (auto-initialized 1-10)"
160 PRINT "Y(1) = "
170 PRINT Y(1)
180 PRINT "Y(5) = "
190 PRINT Y(5)
200 PRINT ""
210 PRINT "Test complete!"
220 END
