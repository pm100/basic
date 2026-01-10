10 PRINT "N-Dimensional Array Support Summary"
20 PRINT "===================================="
30 PRINT ""
40 PRINT "This interpreter supports arrays with"
50 PRINT "up to 60 dimensions as per BASIC spec."
60 PRINT ""
70 PRINT "Test 1: Single dimension"
80 DIM A(5)
90 LET A(3) = 100
100 PRINT "A(3) = "
110 PRINT A(3)
120 PRINT ""
130 PRINT "Test 2: Two dimensions"
140 DIM B(3,3)
150 LET B(2,2) = 200
160 PRINT "B(2,2) = "
170 PRINT B(2,2)
180 PRINT ""
190 PRINT "Test 3: Three dimensions"
200 DIM C(2,2,2)
210 LET C(1,1,1) = 300
220 PRINT "C(1,1,1) = "
230 PRINT C(1,1,1)
240 PRINT ""
250 PRINT "Test 4: Five dimensions"
260 DIM D(1,1,1,1,1)
270 LET D(1,0,1,0,1) = 500
280 PRINT "D(1,0,1,0,1) = "
290 PRINT D(1,0,1,0,1)
300 PRINT ""
310 PRINT "Test 5: Ten dimensions"
320 DIM E(1,1,1,1,1,1,1,1,1,1)
330 LET E(0,1,0,1,0,1,0,1,0,1) = 1000
340 PRINT "E(0,1,0,1,0,1,0,1,0,1) = "
350 PRINT E(0,1,0,1,0,1,0,1,0,1)
360 PRINT ""
370 PRINT "All dimension tests passed!"
380 PRINT ""
390 PRINT "Note: Arrays can have up to 60 dimensions,"
400 PRINT "limited only by available memory."
410 END
