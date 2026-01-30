10 REM Test HEX$ and OCT$ functions
20 PRINT "Decimal", "Hex", "Octal"
30 PRINT "-------", "---", "-----"
40 FOR I = 0 TO 20
50 PRINT I, HEX$(I), OCT$(I)
60 NEXT I
70 PRINT
80 PRINT "Special values:"
90 PRINT "255 =", HEX$(255), "(hex)"
100 PRINT "256 =", HEX$(256), "(hex)"
110 PRINT "64 =", OCT$(64), "(oct)"
120 PRINT "100 =", OCT$(100), "(oct)"
130 END
