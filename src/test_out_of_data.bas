10 PRINT "Testing out of DATA error"
20 PRINT ""
30 DATA 1, 2, 3
40 READ A, B, C
50 PRINT "Read: "
60 PRINT A
70 PRINT B
80 PRINT C
90 PRINT ""
100 PRINT "Trying to read more (should error):"
110 READ D, E
120 PRINT "D = "
130 PRINT D
140 PRINT "E = "
150 PRINT E
160 END
