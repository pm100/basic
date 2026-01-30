10 REM Simple INKEY$ test - just check if empty
20 PRINT "Testing INKEY$..."
30 K$ = INKEY$
40 PRINT "Result length: "; LEN(K$)
50 IF LEN(K$) > 0 THEN PRINT "Got character: ASCII "; ASC(K$)
60 PRINT "Done"
