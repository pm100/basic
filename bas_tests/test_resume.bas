10 REM Test RESUME (retry same line)
20 COUNT = 0
30 ON ERROR GOTO 110
40 PRINT "Attempting division..."
50 X = 10 / COUNT
60 PRINT "Success! X ="; X
70 END
100 REM Error handler
110 PRINT "Error at line"; ERL
120 COUNT = 5
130 PRINT "COUNT set to"; COUNT
140 PRINT "Using RESUME to retry"
150 RESUME
