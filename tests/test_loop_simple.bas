10 DIM A(2,2,2)
20 PRINT "Filling array with loop"
30 FOR I = 0 TO 2
40 LET A(I,0,0) = I * 100
50 NEXT I
60 PRINT "Reading values:"
70 PRINT A(0,0,0)
80 PRINT A(1,0,0)
90 PRINT A(2,0,0)
100 END
