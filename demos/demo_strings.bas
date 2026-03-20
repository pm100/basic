10 PRINT "Comprehensive String Operations Demo"
20 PRINT "====================================="
30 PRINT ""
40 REM Test concatenation
50 LET FIRST = "John"
60 LET LAST = "Doe"
70 LET FULL = FIRST + " " + LAST
80 PRINT "Full name: "
90 PRINT FULL
100 PRINT ""
110 REM Test string functions
120 LET MSG = "Hello, World!"
130 PRINT "Message: "
140 PRINT MSG
150 PRINT "Length: "
160 PRINT LEN(MSG)
170 PRINT "First 5 chars: "
180 PRINT LEFT$(MSG, 5)
190 PRINT "Last 6 chars: "
200 PRINT RIGHT$(MSG, 6)
210 PRINT ""
220 REM Test case conversion
230 LET UPPER = UCASE$(MSG)
240 LET LOWER = LCASE$(MSG)
250 PRINT "Uppercase: "
260 PRINT UPPER
270 PRINT "Lowercase: "
280 PRINT LOWER
290 PRINT ""
300 REM Test substring search and replace
310 LET TEXT = "The quick brown fox"
320 PRINT "Original text: "
330 PRINT TEXT
340 LET POS = INSTR$(TEXT, "quick")
350 PRINT "Position of 'quick': "
360 PRINT POS
370 MID$(TEXT, POS, 5) = "slow "
380 PRINT "After replacement: "
390 PRINT TEXT
400 END
