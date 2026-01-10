10 REM Test LINE INPUT # (file version)
20 OPEN "testlines.txt" FOR OUTPUT AS #1
30 PRINT #1, "First line with, commas"
40 PRINT #1, "Second line: more data"
50 PRINT #1, "Third line"
60 CLOSE #1
70 PRINT "File written"
80 PRINT
90 OPEN "testlines.txt" FOR INPUT AS #1
100 LINE INPUT #1, A$
110 PRINT "Line 1: "; A$
120 LINE INPUT #1, B$
130 PRINT "Line 2: "; B$
140 LINE INPUT #1, C$
150 PRINT "Line 3: "; C$
160 CLOSE #1
170 END
