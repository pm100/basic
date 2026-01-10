10 PRINT "Testing 2D Arrays"
20 PRINT "================="
30 PRINT ""
40 DIM M(2,2)
50 PRINT "Declared 2D array M with DIM M(2,2)"
60 PRINT "Creating a 3x3 matrix (0-2, 0-2):"
70 PRINT ""
80 REM Fill the matrix
90 LET M(0,0) = 1
100 LET M(0,1) = 2
110 LET M(0,2) = 3
120 LET M(1,0) = 4
130 LET M(1,1) = 5
140 LET M(1,2) = 6
150 LET M(2,0) = 7
160 LET M(2,1) = 8
170 LET M(2,2) = 9
180 PRINT "Matrix contents:"
190 FOR ROW = 0 TO 2
200 FOR COL = 0 TO 2
210 PRINT M(ROW,COL)
220 PRINT " "
230 NEXT COL
240 PRINT ""
250 NEXT ROW
260 PRINT ""
270 PRINT "Diagonal elements:"
280 PRINT "M(0,0) = "
290 PRINT M(0,0)
300 PRINT "M(1,1) = "
310 PRINT M(1,1)
320 PRINT "M(2,2) = "
330 PRINT M(2,2)
340 PRINT ""
350 PRINT "Test complete!"
360 END
