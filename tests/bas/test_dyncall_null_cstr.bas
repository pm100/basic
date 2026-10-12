10 REM A NULL const char* return is nil (0), and an unset env var is not an empty string
20 DEF XFN getenv("msvcrt.dll|getenv|cstr|cstr|")
30 LET V = FN getenv("DYNCALL_SURELY_UNSET_VAR_12345")
40 PRINT "unset: "; V
50 LET W = FN getenv("PATH")
60 IF LEN(W) > 0 THEN PRINT "path set"
REM EXPECT: unset: 0
REM EXPECT: path set
