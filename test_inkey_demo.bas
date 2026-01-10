10 REM INKEY$ Demo - Simple key detector
20 PRINT "Press ESC to exit, any other key to see its code"
30 PRINT
40 K$ = INKEY$
50 IF K$ = "" THEN GOTO 40
55 LET A = ASC(K$)
60 IF A = 27 THEN GOTO 100
70 IF NOT A = 0 THEN PRINT "Key: "; K$; " ASCII: "; A
80 GOTO 40
100 PRINT "ESC pressed - exiting"
110 END