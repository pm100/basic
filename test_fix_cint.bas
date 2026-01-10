10 REM Test FIX and CINT functions
20 PRINT "Testing FIX (truncate towards zero):"
30 PRINT "FIX(3.7) ="; FIX(3.7)
40 PRINT "FIX(-3.7) ="; FIX(-3.7)
50 PRINT "FIX(3.2) ="; FIX(3.2)
60 PRINT "FIX(-3.2) ="; FIX(-3.2)
70 PRINT
80 PRINT "Testing CINT (round to nearest integer):"
90 PRINT "CINT(3.7) ="; CINT(3.7)
100 PRINT "CINT(-3.7) ="; CINT(-3.7)
110 PRINT "CINT(3.2) ="; CINT(3.2)
120 PRINT "CINT(-3.2) ="; CINT(-3.2)
130 PRINT "CINT(3.5) ="; CINT(3.5)
140 PRINT "CINT(4.5) ="; CINT(4.5)
150 PRINT
160 PRINT "Comparing INT, FIX, and CINT:"
170 X = -3.7
180 PRINT "X ="; X
190 PRINT "INT(X) ="; INT(X)
200 PRINT "FIX(X) ="; FIX(X)
210 PRINT "CINT(X) ="; CINT(X)
220 END
