10 PRINT "Testing Error Handling"
20 PRINT ""
30 PRINT "Testing SQR with negative number:"
40 LET X = -9
50 PRINT "SQR(-9) should give error and use absolute value:"
60 PRINT SQR(X)
70 PRINT ""
80 PRINT "Testing LOG with negative number:"
90 LET X = -5
100 PRINT "LOG(-5) should give error:"
110 PRINT LOG(X)
120 PRINT ""
130 PRINT "Testing LOG with zero:"
140 LET X = 0
150 PRINT "LOG(0) should give error:"
160 PRINT LOG(X)
170 PRINT ""
180 PRINT "Testing complex expression:"
190 LET A = 2
200 LET B = 3
210 PRINT "2 ^ 3 ^ 2 = 2 ^ (3 ^ 2) = 2 ^ 9 = "
220 PRINT A ^ B ^ A
230 PRINT ""
240 PRINT "Testing nested functions:"
250 LET X = -2.5
260 PRINT "ABS(SIN(ABS(X))) = "
270 PRINT ABS(SIN(ABS(X)))
280 END
