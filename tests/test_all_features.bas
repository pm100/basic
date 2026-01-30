10 REM ========================================
20 REM Comprehensive Test of New Features
30 REM ========================================
40 PRINT "BASIC Interpreter - Feature Test"
50 PRINT "================================="
60 PRINT ""
70 REM ----------------------------------------
80 REM Test 1: Logical Operators (AND, OR, NOT)
90 REM ----------------------------------------
100 PRINT "1. Logical Operators"
110 LET X = 10
120 LET Y = 20
130 IF X > 5 AND Y < 30 THEN 160
140 PRINT "FAIL: AND operator"
150 GOTO 170
160 PRINT "PASS: AND operator"
170 IF X < 5 OR Y > 15 THEN 200
180 PRINT "FAIL: OR operator"
190 GOTO 210
200 PRINT "PASS: OR operator"
210 IF NOT X > 100 THEN 240
220 PRINT "FAIL: NOT operator"
230 GOTO 250
240 PRINT "PASS: NOT operator"
250 PRINT ""
260 REM ----------------------------------------
270 REM Test 2: MOD Operator
280 REM ----------------------------------------
290 PRINT "2. MOD Operator"
300 LET A = 17 MOD 5
310 PRINT "17 MOD 5 = "
320 PRINT A
330 LET B = 100
340 IF B MOD 2 = 0 THEN 370
350 PRINT "FAIL: 100 is not even"
360 GOTO 380
370 PRINT "PASS: 100 is even"
380 PRINT ""
390 REM ----------------------------------------
400 REM Test 3: SPACE$ Function
410 REM ----------------------------------------
420 PRINT "3. SPACE$ Function"
430 LET MSG = "Hello" + SPACE$(3) + "World"
440 PRINT MSG
450 PRINT ""
460 REM ----------------------------------------
470 REM Test 4: STRING$ Function
480 REM ----------------------------------------
490 PRINT "4. STRING$ Function"
500 PRINT STRING$(15, 61)
510 PRINT STRING$(10, "*")
520 PRINT ""
530 REM ----------------------------------------
540 REM Test 5: ON...GOTO
550 REM ----------------------------------------
560 PRINT "5. ON...GOTO"
570 LET CHOICE = 2
580 ON CHOICE GOTO 700, 800, 900
590 PRINT "FAIL: ON...GOTO"
600 GOTO 1000
700 PRINT "Option 1"
710 GOTO 1000
800 PRINT "PASS: ON...GOTO (option 2)"
810 GOTO 1000
900 PRINT "Option 3"
910 GOTO 1000
1000 PRINT ""
1010 REM ----------------------------------------
1020 REM Test 6: ON...GOSUB
1030 REM ----------------------------------------
1040 PRINT "6. ON...GOSUB"
1050 LET N = 1
1060 ON N GOSUB 1200, 1300
1070 PRINT ""
1080 PRINT "All features tested successfully!"
1090 END
1200 PRINT "PASS: ON...GOSUB (subroutine 1)"
1210 RETURN
1300 PRINT "Subroutine 2"
1310 RETURN
