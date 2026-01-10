10 PRINT "OPTION BASE Comprehensive Demo"
20 PRINT "==============================="
30 PRINT ""
40 PRINT "Demonstration of array indexing with OPTION BASE"
50 PRINT ""
60 PRINT "Test 1: Default behavior (0-based)"
70 PRINT "-----------------------------------"
80 DIM A(2)
90 LET A(0) = 10
100 LET A(1) = 20
110 LET A(2) = 30
120 PRINT "Array A declared as DIM A(2)"
130 PRINT "Has 3 elements: indices 0, 1, 2"
140 FOR I = 0 TO 2
150 PRINT "A("
160 PRINT I
170 PRINT ") = "
180 PRINT A(I)
190 NEXT I
200 PRINT ""
210 PRINT "Test 2: OPTION BASE 1"
220 PRINT "-----------------------------------"
230 OPTION BASE 1
240 DIM B(2)
250 LET B(1) = 100
260 LET B(2) = 200
270 PRINT "Array B declared as DIM B(2) with OPTION BASE 1"
280 PRINT "Has 2 elements: indices 1, 2"
290 FOR I = 1 TO 2
300 PRINT "B("
310 PRINT I
320 PRINT ") = "
330 PRINT B(I)
340 NEXT I
350 PRINT ""
360 PRINT "Test 3: Multidimensional with OPTION BASE 1"
370 PRINT "-----------------------------------"
380 DIM MATRIX(3,3)
390 LET MATRIX(1,1) = 11
400 LET MATRIX(1,2) = 12
410 LET MATRIX(1,3) = 13
420 LET MATRIX(2,1) = 21
430 LET MATRIX(2,2) = 22
440 LET MATRIX(2,3) = 23
450 LET MATRIX(3,1) = 31
460 LET MATRIX(3,2) = 32
470 LET MATRIX(3,3) = 33
480 PRINT "3x3 Matrix (indices 1-3 for each dimension):"
490 FOR ROW = 1 TO 3
500 FOR COL = 1 TO 3
510 PRINT MATRIX(ROW,COL)
520 PRINT " "
530 NEXT COL
540 PRINT ""
550 NEXT ROW
560 PRINT ""
570 PRINT "Key Points:"
580 PRINT "- Default OPTION BASE is 0"
590 PRINT "- DIM A(N) with BASE 0: indices 0 to N (N+1 elements)"
600 PRINT "- DIM A(N) with BASE 1: indices 1 to N (N elements)"
610 PRINT "- Arrays created before OPTION BASE retain their base"
620 PRINT ""
630 PRINT "Demo complete!"
640 END
