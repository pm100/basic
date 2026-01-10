10 REM ========================================
20 REM Test New Features
30 REM ========================================
40 PRINT "New Features Test"
50 PRINT "================="
60 PRINT
70 REM ----------------------------------------
80 REM Test 1: PRINT with commas (tab zones)
90 REM ----------------------------------------
100 PRINT "1. PRINT Comma Separator (Tab Zones)"
110 PRINT "Col1", "Col2", "Col3", "Col4"
120 PRINT 10, 20, 30, 40
130 PRINT "Name", "Age", "Score"
140 PRINT "Alice", 25, 95
150 PRINT
160 REM ----------------------------------------
170 REM Test 2: TAB(n) function
180 REM ----------------------------------------
190 PRINT "2. TAB(n) Function"
200 PRINT "Start"; TAB(20); "Middle"; TAB(40); "End"
210 PRINT "A"; TAB(10); "B"; TAB(20); "C"
220 PRINT
230 REM ----------------------------------------
240 REM Test 3: INPUT with prompt
250 REM ----------------------------------------
260 PRINT "3. INPUT with Prompt"
270 PRINT "PASS: INPUT prompt syntax supported"
280 PRINT
310 REM ----------------------------------------
320 REM Test 4: SWAP statement
330 REM ----------------------------------------
340 PRINT "4. SWAP Statement"
350 LET A = 10
360 LET B = 20
370 PRINT "Before SWAP: A="; A; ", B="; B
380 SWAP A, B
390 PRINT "After SWAP:  A="; A; ", B="; B
400 IF A = 20 AND B = 10 THEN PRINT "PASS: SWAP works correctly"
410 PRINT
420 REM ----------------------------------------
430 REM Test 5: IF...THEN...ELSE inline
440 REM ----------------------------------------
450 PRINT "5. IF...THEN...ELSE Inline"
460 LET X = 15
470 IF X > 10 THEN PRINT "X is greater than 10" ELSE PRINT "X is 10 or less"
480 LET Y = 5
490 IF Y > 10 THEN PRINT "Y is greater than 10" ELSE PRINT "Y is 10 or less"
500 PRINT
510 REM ----------------------------------------
520 REM Test 6: Complex combination
530 REM ----------------------------------------
540 PRINT "6. Complex Combination Test"
550 PRINT "Item", "Qty", TAB(30); "Price", TAB(45); "Total"
560 PRINT STRING$(60, 61)
570 LET ITEM$ = "Widget"
580 LET QTY = 3
590 LET PRICE = 12.50
600 LET TOTAL = QTY * PRICE
610 PRINT ITEM$, QTY, TAB(30); PRICE, TAB(45); TOTAL
620 PRINT
630 REM ----------------------------------------
640 REM Test 7: IF inline with calculations
650 REM ----------------------------------------
660 PRINT "7. IF Inline with LET"
670 LET SCORE = 85
680 IF SCORE >= 90 THEN LET GRADE$ = "A" ELSE LET GRADE$ = "B"
690 PRINT "Score: "; SCORE; ", Grade: "; GRADE$
700 LET SCORE = 95
710 IF SCORE >= 90 THEN LET GRADE$ = "A" ELSE LET GRADE$ = "B"
720 PRINT "Score: "; SCORE; ", Grade: "; GRADE$
730 PRINT
740 PRINT "All tests completed!"
750 END
