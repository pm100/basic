10 REM ========================================
20 REM New Features Test Program
30 REM ========================================
40 PRINT "BASIC Interpreter - New Features"
50 PRINT "================================"
60 PRINT ""
70 REM Test 1: Comments with REM and '
80 PRINT "1. Comments work (REM and apostrophe)"
90 ' This is also a comment!
100 PRINT ""
110 REM Test 2: STR$ function
120 PRINT "2. STR$ Function"
130 LET N = 42
140 LET S = STR$(N)
150 PRINT "Number 42 as string: "
160 PRINT S
170 PRINT "STR$(-10) = "
180 PRINT STR$(-10)
190 PRINT ""
200 REM Test 3: VAL function
210 PRINT "3. VAL Function"
220 LET TEXT = "987"
230 LET NUM = VAL(TEXT)
240 PRINT "String 987 as number: "
250 PRINT NUM
260 PRINT "VAL with math: "
270 PRINT VAL("100") + VAL("23")
280 PRINT ""
290 REM Test 4: Round-trip conversion
300 PRINT "4. Round-Trip STR$ and VAL"
310 LET ORIGINAL = 555
320 LET AS_STRING = STR$(ORIGINAL)
330 LET BACK_TO_NUM = VAL(AS_STRING)
340 PRINT "Original: "
350 PRINT ORIGINAL
360 PRINT "After STR$ then VAL: "
370 PRINT BACK_TO_NUM
380 PRINT ""
390 REM Test 5: Random numbers
400 PRINT "5. Random Numbers"
410 PRINT "Five random numbers:"
420 FOR I = 1 TO 5
430 PRINT RND
440 NEXT I
450 PRINT ""
460 REM Test 6: RANDOMIZE with seed
470 PRINT "6. RANDOMIZE with seed"
480 RANDOMIZE 12345
490 PRINT "After RANDOMIZE 12345:"
500 PRINT RND
510 PRINT RND
520 PRINT ""
530 REM Test 7: Using in calculations
540 PRINT "7. Functions in Calculations"
550 LET A = VAL("50")
560 LET B = VAL("30")
570 LET SUM = A + B
580 PRINT "VAL(50) + VAL(30) = "
590 PRINT SUM
600 PRINT ""
610 PRINT "All tests complete!"
620 END
