10 REM Test error handling
20 ON ERROR GOTO 110
30 PRINT "Before error"
40 X = 1 / 0
50 PRINT "Line after error - this WILL execute"
60 PRINT "Program continues normally"
70 END
100 REM Error handler
110 PRINT "--- Error Handler ---"
120 PRINT "Error at line"; ERL; "code"; ERR
130 RESUME NEXT
140 PRINT "ERROR: Handler should not continue past RESUME"
150 END
