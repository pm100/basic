10 REM Test sscanf via dyncall - parse a token from a string
20 DEF XFN sscanf("msvcrt.dll|sscanf|cstr,cstr,ocstr=256|i32|fixargs=2")
30 LET buf$ = ""
40 LET n = FN sscanf("hello world", "%s", buf$)
50 PRINT "sscanf returned: "; n
60 PRINT "Parsed token: "; buf$
70 REM Parse an integer as a string token
80 LET n2 = FN sscanf("42 extra", "%s", buf$)
90 PRINT "sscanf returned: "; n2
100 PRINT "Parsed number string: "; buf$
110 END
