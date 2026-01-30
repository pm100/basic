10 PRINT "Testing SPACE$ and STRING$"
20 PRINT "=========================="
30 PRINT ""
40 REM Test SPACE$
50 PRINT "SPACE$ function:"
60 PRINT "["
70 PRINT SPACE$(10)
80 PRINT "]"
90 LET S = "Hello" + SPACE$(5) + "World"
100 PRINT S
110 PRINT ""
120 REM Test STRING$ with character code
130 PRINT "STRING$ with ASCII code:"
140 PRINT STRING$(10, 65)
150 PRINT STRING$(5, 42)
160 PRINT ""
170 REM Test STRING$ with string
180 PRINT "STRING$ with character:"
190 PRINT STRING$(8, "X")
200 PRINT STRING$(12, "-")
210 PRINT ""
220 REM Draw a box
230 PRINT "Drawing a box:"
240 PRINT "+" + STRING$(20, "-") + "+"
250 PRINT "|" + SPACE$(20) + "|"
260 PRINT "|" + SPACE$(20) + "|"
270 PRINT "+" + STRING$(20, "-") + "+"
280 END
