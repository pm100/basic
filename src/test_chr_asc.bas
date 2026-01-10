10 PRINT "CHR$ and ASC Functions Test"
20 PRINT "============================"
30 PRINT ""
40 REM Test CHR$ - convert numbers to characters
50 PRINT "Test 1: CHR$ function"
60 PRINT "---------------------"
70 PRINT "CHR$(65) = "
80 PRINT CHR$(65)
90 PRINT "CHR$(97) = "
100 PRINT CHR$(97)
110 PRINT "CHR$(48) = "
120 PRINT CHR$(48)
130 PRINT "CHR$(32) = "
140 PRINT CHR$(32)
150 PRINT "(space)"
160 PRINT ""
170 REM Test ASC - convert characters to numbers
180 PRINT "Test 2: ASC function"
190 PRINT "--------------------"
200 PRINT "ASC(A) = "
210 PRINT ASC("A")
220 PRINT "ASC(a) = "
230 PRINT ASC("a")
240 PRINT "ASC(0) = "
250 PRINT ASC("0")
260 PRINT "ASC(space) = "
270 PRINT ASC(" ")
280 PRINT ""
290 REM Test ASC with multi-character strings
300 PRINT "Test 3: ASC with multi-char strings"
310 PRINT "------------------------------------"
320 LET S = "Hello"
330 PRINT "String: "
340 PRINT S
350 PRINT "ASC(S) = "
360 PRINT ASC(S)
370 PRINT "(only first char)"
380 PRINT ""
390 REM Test round-trip conversion
400 PRINT "Test 4: Round-trip conversion"
410 PRINT "-----------------------------"
420 LET N = 66
430 PRINT "Original number: "
440 PRINT N
450 LET C = CHR$(N)
460 PRINT "CHR$(N) = "
470 PRINT C
480 LET N2 = ASC(C)
490 PRINT "ASC(C) back to: "
500 PRINT N2
510 PRINT ""
520 REM Test building strings from ASCII codes
530 PRINT "Test 5: Building a word"
540 PRINT "-----------------------"
550 LET WORD = CHR$(72) + CHR$(101) + CHR$(108) + CHR$(108) + CHR$(111)
560 PRINT "Built from ASCII: "
570 PRINT WORD
580 PRINT ""
590 REM Test ASCII table display
600 PRINT "Test 6: ASCII table sample"
610 PRINT "--------------------------"
620 PRINT "Code Char"
630 FOR I = 65 TO 75
640 PRINT I
650 PRINT "    "
660 PRINT CHR$(I)
670 NEXT I
680 PRINT ""
690 PRINT "All tests complete!"
700 END
