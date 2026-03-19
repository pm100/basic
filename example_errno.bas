10  REM errno example - Windows (msvcrt.dll)
20  REM Add the 'errno' flag to a DEF XFN to capture the platform error after
30  REM the call. The value is stored in the built-in variable ERRNO.
40  REM On Windows this reflects GetLastError() (3=path not found, 2=file not found).
50  PRINT "errno example"
60  DEF XFN c_fopen("msvcrt.dll|fopen|cstr,cstr|ptr|errno")
70  DEF XFN c_fclose("msvcrt.dll|fclose|ptr|i32|")
80  LET fp = FN c_fopen("__no_such_file_dyncall__.txt", "r")
90  IF fp <> 0 THEN GOTO 120
100 PRINT "fopen failed, ERRNO = "; ERRNO
110 GOTO 140
120 PRINT "fopen unexpectedly succeeded"
130 LET r = FN c_fclose(fp)
140 LET fp2 = FN c_fopen("example_errno.bas", "r")
150 IF fp2 = 0 THEN GOTO 180
160 PRINT "opened example_errno.bas, ERRNO = "; ERRNO
170 LET r = FN c_fclose(fp2)
180 PRINT "Done."
190 END
