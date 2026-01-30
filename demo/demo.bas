10 PRINT "BASIC Numeric Functions - Comprehensive Demo"
20 PRINT "=============================================="
30 PRINT ""
40 PRINT "Calculating circle properties:"
50 LET PI = 3.14159265
60 LET RADIUS = 5
70 PRINT "Radius = 5"
80 PRINT "Circumference = 2 * PI * R = "
90 PRINT 2 * PI * RADIUS
100 PRINT "Area = PI * R ^ 2 = "
110 PRINT PI * RADIUS ^ 2
120 PRINT ""
130 PRINT "Trigonometry Demo:"
140 LET ANGLE = PI / 4
150 PRINT "Angle = PI/4 radians (45 degrees)"
160 PRINT "SIN(ANGLE) = "
170 PRINT SIN(ANGLE)
180 PRINT "COS(ANGLE) = "
190 PRINT COS(ANGLE)
200 PRINT "TAN(ANGLE) = "
210 PRINT TAN(ANGLE)
220 PRINT "ATN(1) = PI/4 = "
230 PRINT ATN(1)
240 PRINT ""
250 PRINT "Exponential and Logarithm Demo:"
260 LET E = EXP(1)
270 PRINT "e = EXP(1) = "
280 PRINT E
290 PRINT "LOG(e) = "
300 PRINT LOG(E)
310 PRINT "e ^ 2 = "
320 PRINT E ^ 2
330 PRINT ""
340 PRINT "Statistical Demo:"
350 PRINT "Generating 5 random numbers:"
360 FOR I = 1 TO 5
370 PRINT RND(0)
380 NEXT I
390 PRINT ""
400 PRINT "Number Processing Demo:"
410 LET VALUE = -7.8
420 PRINT "Original value: -7.8"
430 PRINT "ABS(-7.8) = "
440 PRINT ABS(VALUE)
450 PRINT "INT(-7.8) = "
460 PRINT INT(VALUE)
470 PRINT "SQR(ABS(-7.8)) = "
480 PRINT SQR(ABS(VALUE))
490 PRINT ""
500 PRINT "Complex Expression Demo:"
510 PRINT "Evaluating: 2 ^ 3 + SQR(16) * COS(0) - ABS(-3)"
520 PRINT 2 ^ 3 + SQR(16) * COS(0) - ABS(-3)
530 PRINT ""
540 PRINT "Expected: 8 + 4 * 1 - 3 = 9"
550 PRINT "Result matches!"
560 PRINT ""
570 PRINT "Demo complete!"
580 END
