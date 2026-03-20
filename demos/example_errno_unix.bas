10  REM errno example - Linux/macOS
20  REM Add the 'errno' flag to a DEF XFN to capture the C errno after the call.
30  REM The value is stored in the built-in variable ERRNO automatically.
40  REM ENOENT (file not found) = 2 on Linux/macOS.
50  PRINT "errno example"
60  DEF XFN c_fopen("libc.so.6|fopen|cstr,cstr|ptr|errno")
70  DEF XFN c_fclose("libc.so.6|fclose|ptr|i32|")
80  LET fp = FN c_fopen("__no_such_file_dyncall__.txt", "r")
90  IF fp <> 0 THEN GOTO 120
100 PRINT "fopen failed, ERRNO = "; ERRNO
110 GOTO 140
120 PRINT "fopen unexpectedly succeeded"
130 LET r = FN c_fclose(fp)
140 LET fp2 = FN c_fopen("example_errno_unix.bas", "r")
150 IF fp2 = 0 THEN GOTO 180
160 PRINT "opened example_errno_unix.bas, ERRNO = "; ERRNO
170 LET r = FN c_fclose(fp2)
180 PRINT "Done."
190 END
