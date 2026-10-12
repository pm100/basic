10 REM Bad output-buffer lengths are errors, not panics or huge allocations
20 DEF XFN strncpy("msvcrt.dll|strncpy|ocstr=arg2,cstr,i32|ptr|")
30 LET B = ""
40 LET R = FN strncpy(B, "hello", 0-1)
50 PRINT "negative: "; R; " ["; B; "]"
60 DEF XFN GetTempPathA("kernel32.dll|GetTempPathA|u32,ocstr=arg0|u32|")
70 LET T = ""
80 LET N = FN GetTempPathA(2000000000, T)
90 PRINT "huge: "; N; " ["; T; "]"
100 PRINT "still running"
REM EXPECT: negative: 0 []
REM EXPECT: huge: 0 []
REM EXPECT: still running
