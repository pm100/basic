10 REM Test INKEY$ function
20 PRINT "Press any key (or wait 5 seconds)..."
30 COUNT = 0
40 K$ = INKEY$
50 IF K$ = "" THEN COUNT = COUNT + 1: IF COUNT < 50 THEN GOTO 40
60 IF K$ = "" THEN PRINT "No key pressed": GOTO 80
70 PRINT "You pressed: "; K$; " (ASCII "; ASC(K$); ")"
80 END
