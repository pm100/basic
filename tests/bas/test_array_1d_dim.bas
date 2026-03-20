10 PRINT "Testing 1D Arrays with DIM Statement"
20 PRINT "====================================="
30 PRINT ""
40 DIM B(5)
50 PRINT "Declared array B with DIM B(5) - indices 0 to 5"
60 PRINT ""
70 FOR I = 0 TO 5
80 LET B(I) = I * 10
90 NEXT I
100 PRINT "Filled array with FOR loop:"
110 FOR I = 0 TO 5
120 PRINT "B("
130 PRINT I
140 PRINT ") = "
150 PRINT B(I)
160 NEXT I
170 PRINT ""
180 PRINT "Computing sum:"
190 LET SUM = 0
200 FOR I = 0 TO 5
210 LET SUM = SUM + B(I)
220 NEXT I
230 PRINT "Sum of B(0) through B(5) = "
240 PRINT SUM
250 PRINT ""
260 PRINT "Test complete!"
270 END
