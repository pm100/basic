10 REM Test dyncall with string output
20 DEF XFN getenv("msvcrt.dll|getenv|cstr|cstr|")
30 LET result$ = FN getenv("PATH")
40 PRINT "PATH environment variable:"
50 PRINT result$
REM expect_contains: PATH
