1 REM SKIP
10 PRINT "Testing INKEY$ - Press keys, ESC to exit"
20 K$ = INKEY$
30 IF K$ <> "" THEN PRINT "Got key: "; K$; " ASCII: "; ASC(K$)
40 IF ASC(K$) = 27 THEN END
50 GOTO 20

