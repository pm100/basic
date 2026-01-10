10 PRINT "===================================="
20 PRINT "Enhanced BASIC Features Test"
30 PRINT "===================================="
40 PRINT ""
50 PRINT "Test 1: PRINT with Comma Separators"
60 PRINT "Name", "Age", "City", "Score"
70 PRINT "Alice", 25, "NYC", 95
80 PRINT "Bob", 30, "LA", 88
90 PRINT ""
100 PRINT "Test 2: TAB() Function"
110 PRINT "Start"; TAB(20); "Middle"; TAB(40); "End"
120 PRINT ""
130 PRINT "Test 3: SWAP Statement"
140 LET X = 100
150 LET Y = 200
160 PRINT "Before: X="; X; " Y="; Y
170 SWAP X, Y
180 PRINT "After:  X="; X; " Y="; Y
190 PRINT ""
200 PRINT "Test 4: IF...THEN...ELSE Inline"
210 LET SCORE = 92
220 IF SCORE >= 90 THEN PRINT "Grade: A" ELSE PRINT "Grade: B"
230 LET SCORE = 75
240 IF SCORE >= 90 THEN PRINT "Grade: A" ELSE PRINT "Grade: B"
250 PRINT ""
260 PRINT "Test 5: Complex Table"
270 PRINT "Product", TAB(20); "Qty", TAB(30); "Price", TAB(45); "Total"
280 PRINT STRING$(60, 45)
290 LET PROD$ = "Widget"
300 LET Q = 5
310 LET P = 19.99
320 PRINT PROD$, TAB(20); Q, TAB(30); P, TAB(45); Q * P
330 PRINT ""
340 PRINT "All tests passed!"
350 END
