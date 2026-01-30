10 REM ========================================
20 REM Test Medium Complexity Features
30 REM ========================================
40 PRINT "Advanced BASIC Features Test"
50 PRINT "============================"
60 PRINT ""
70 REM ----------------------------------------
80 REM Test 1: DEF FN - User-Defined Functions
90 REM ----------------------------------------
100 PRINT "1. DEF FN - User-Defined Functions"
110 DEF FN DOUBLE(X) = X * 2
120 DEF FN SQUARE(N) = N * N
130 DEF FN AREA(R) = 3.14159 * R * R
140 PRINT "FN DOUBLE(5) = "; FN DOUBLE(5)
150 PRINT "FN SQUARE(7) = "; FN SQUARE(7)
160 PRINT "FN AREA(10) = "; FN AREA(10)
170 PRINT ""
180 REM ----------------------------------------
190 REM Test 2: EXIT FOR - Early Loop Termination
200 REM ----------------------------------------
210 PRINT "2. EXIT FOR Statement"
220 PRINT "Counting 1 to 10, exit at 5:"
230 FOR I = 1 TO 10
240   PRINT I;
250   IF I = 5 THEN EXIT FOR
260 NEXT I
270 PRINT ""
280 PRINT "Loop exited at I = "; I
290 PRINT ""
300 REM ----------------------------------------
310 REM Test 3: EXIT WHILE - Early Loop Exit
320 REM ----------------------------------------
330 PRINT "3. EXIT WHILE Statement"
340 PRINT "Counting while < 100, exit at 7:"
350 LET N = 1
360 WHILE N < 100
370   PRINT N;
380   LET N = N + 1
390   IF N > 7 THEN EXIT WHILE
400 WEND
410 PRINT ""
420 PRINT "Loop exited at N = "; N
430 PRINT ""
440 REM ----------------------------------------
450 REM Test 4: SELECT CASE - Multi-way Branching
460 REM ----------------------------------------
470 PRINT "4. SELECT CASE Statement"
480 LET DAY = 3
490 PRINT "Day "; DAY; " is: ";
500 SELECT CASE DAY
510 CASE 1
520   PRINT "Monday"
530 CASE 2
540   PRINT "Tuesday"
550 CASE 3
560   PRINT "Wednesday"
570 CASE 4, 5
580   PRINT "Thursday or Friday"
590 CASE ELSE
600   PRINT "Weekend"
610 END SELECT
620 PRINT ""
630 REM ----------------------------------------
640 REM Test 5: SELECT CASE with Strings
650 REM ----------------------------------------
660 PRINT "5. SELECT CASE with String Values"
670 LET GRADE$ = "B"
680 PRINT "Grade "; GRADE$; " means: ";
690 SELECT CASE GRADE$
700 CASE "A"
710   PRINT "Excellent"
720 CASE "B"
730   PRINT "Good"
740 CASE "C"
750   PRINT "Average"
760 CASE ELSE
770   PRINT "Needs Improvement"
780 END SELECT
790 PRINT ""
800 REM ----------------------------------------
810 REM Test 6: Complex Example
820 REM ----------------------------------------
830 PRINT "6. Complex Combination"
840 DEF FN CELSIUS(F) = (F - 32) * 5 / 9
850 FOR TEMP = 32 TO 212 STEP 36
860   LET C = FN CELSIUS(TEMP)
870   PRINT TEMP; "F = "; C; "C";
880   IF C >= 100 THEN EXIT FOR
890   PRINT " (below boiling)"
900 NEXT TEMP
910 PRINT ""
920 PRINT ""
930 PRINT "All advanced features tested!"
940 END
