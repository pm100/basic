10 PRINT "Starting..."
20 LET K$ = INKEY$
30 PRINT "INKEY$ returned: '"; K$; "'"
40 PRINT "Length: "; LEN(K$)
50 IF LEN(K$) > 0 THEN PRINT "ASCII: "; ASC(K$)
