10 REM errno is reset before each call: a success must not report the previous failure
20 DEF XFN fopen("msvcrt.dll|fopen|cstr,cstr|ptr|errno")
30 LET F = FN fopen("/no/such/dir/x", "r")
40 IF ERRNO <> 0 THEN PRINT "failed: errno set"
50 DEF XFN myabs("msvcrt.dll|abs|i32|i32|errno")
60 LET A = FN myabs(0-5)
70 PRINT "abs: "; A; " errno: "; ERRNO
REM EXPECT: failed: errno set
REM EXPECT: abs: 5 errno: 0
