10 PRINT "====================================="
20 PRINT "Comprehensive String Operations Test"
30 PRINT "====================================="
40 PRINT ""
50 REM Test 1: String concatenation
60 PRINT "Test 1: Concatenation"
70 PRINT "---------------------"
80 LET GREETING = "Hello" + ", " + "World!"
90 PRINT GREETING
100 PRINT ""
110 REM Test 2: LEN function
120 PRINT "Test 2: LEN()"
130 PRINT "-------------"
140 LET TEXT = "BASIC Programming"
150 PRINT "Text: "
160 PRINT TEXT
170 PRINT "Length: "
180 PRINT LEN(TEXT)
190 PRINT ""
200 REM Test 3: LEFT$ and RIGHT$
210 PRINT "Test 3: LEFT$ and RIGHT$"
220 PRINT "------------------------"
230 PRINT "First 5: "
240 PRINT LEFT$(TEXT, 5)
250 PRINT "Last 11: "
260 PRINT RIGHT$(TEXT, 11)
270 PRINT ""
280 REM Test 4: MID$ extraction
290 PRINT "Test 4: MID$ extraction"
300 PRINT "-----------------------"
310 PRINT "Characters 7-17: "
320 PRINT MID$(TEXT, 7, 11)
330 PRINT ""
340 REM Test 5: UCASE$ and LCASE$
350 PRINT "Test 5: Case conversion"
360 PRINT "-----------------------"
370 LET MIXED = "HeLLo WoRLd"
380 PRINT "Original: "
390 PRINT MIXED
400 PRINT "Upper: "
410 PRINT UCASE$(MIXED)
420 PRINT "Lower: "
430 PRINT LCASE$(MIXED)
440 PRINT ""
450 REM Test 6: INSTR$ search
460 PRINT "Test 6: INSTR$ search"
470 PRINT "---------------------"
480 LET SENTENCE = "The cat in the hat"
490 PRINT "Sentence: "
500 PRINT SENTENCE
510 LET POS1 = INSTR$(SENTENCE, "cat")
520 LET POS2 = INSTR$(SENTENCE, "hat")
530 PRINT "Position of 'cat': "
540 PRINT POS1
550 PRINT "Position of 'hat': "
560 PRINT POS2
570 PRINT ""
580 REM Test 7: MID$ assignment
590 PRINT "Test 7: MID$ assignment"
600 PRINT "-----------------------"
610 LET MESSAGE = "I like apples"
620 PRINT "Before: "
630 PRINT MESSAGE
640 LET WORD_POS = INSTR$(MESSAGE, "like")
650 MID$(MESSAGE, WORD_POS, 4) = "love"
660 PRINT "After: "
670 PRINT MESSAGE
680 PRINT ""
690 REM Test 8: Complex example
700 PRINT "Test 8: Name processing"
710 PRINT "-----------------------"
720 LET FULLNAME = "john doe"
730 PRINT "Original: "
740 PRINT FULLNAME
750 LET FIRST = LEFT$(FULLNAME, 4)
760 LET LAST_START = INSTR$(FULLNAME, " ") + 1
770 LET LAST = MID$(FULLNAME, LAST_START, 3)
780 LET FIRST_UPPER = UCASE$(LEFT$(FIRST, 1)) + MID$(FIRST, 2, 3)
790 LET LAST_UPPER = UCASE$(LEFT$(LAST, 1)) + MID$(LAST, 2, 2)
800 LET FORMATTED = FIRST_UPPER + " " + LAST_UPPER
810 PRINT "Formatted: "
820 PRINT FORMATTED
830 PRINT ""
840 PRINT "All tests complete!"
850 END
