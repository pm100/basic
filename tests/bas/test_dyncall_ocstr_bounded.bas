10 REM ocstr output is zero-filled and NUL-bounded even when the callee does not terminate it
20 DEF XFN strncpy("msvcrt.dll|strncpy|ocstr=arg2,cstr,usize|ptr|")
30 LET B = ""
40 LET R = FN strncpy(B, "hello", 2)
50 PRINT "buf: ["; B; "]"
60 LET C = ""
70 LET R = FN strncpy(C, "", 8)
80 PRINT "empty: ["; C; "]"
REM EXPECT: buf: [he]
REM EXPECT: empty: []
