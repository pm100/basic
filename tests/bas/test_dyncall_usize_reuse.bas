10 REM usize token, and reusing one definition many times (FuncDef is shared and reusable)
20 DEF XFN strlen("msvcrt.dll|strlen|cstr|usize|")
30 PRINT "len: "; FN strlen("hello")
40 LET TOTAL = 0
50 FOR I = 1 TO 200
60 LET TOTAL = TOTAL + FN strlen("abc")
70 NEXT I
80 PRINT "total: "; TOTAL
REM EXPECT: len: 5
REM EXPECT: total: 600
