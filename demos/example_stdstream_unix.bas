10  REM Standard stream example - Linux/macOS version
20  REM Use fopen("/dev/stderr", "w") to get a FILE* for a standard stream.
30  REM /dev/stdin, /dev/stdout, /dev/stderr are always available on Linux and macOS.
40  REM On Linux use libc.so.6; on macOS use libSystem.B.dylib.
50  PRINT "Standard stream example (Linux/macOS)"
60  DEF XFN c_fopen("libc.so.6|fopen|cstr,cstr|ptr|")
70  DEF XFN c_fputs("libc.so.6|fputs|cstr,ptr|i32|")
80  DEF XFN c_fflush("libc.so.6|fflush|ptr|i32|")
90  DEF XFN c_fclose("libc.so.6|fclose|ptr|i32|")
100 REM Open stderr and stdout
110 LET fp_stderr = FN c_fopen("/dev/stderr", "w")
120 LET fp_stdout = FN c_fopen("/dev/stdout", "w")
130 REM fputs to stderr writes directly to the console
140 LET r = FN c_fputs("Hello from BASIC via fputs (stderr)", fp_stderr)
150 PRINT "fputs returned: "; r
160 LET r = FN c_fputs("Result of 6 * 7 = 42", fp_stderr)
170 PRINT "fputs returned: "; r
180 REM fflush stdout to ensure buffered output is written
190 LET r = FN c_fflush(fp_stdout)
200 PRINT "fflush stdout returned: "; r
210 LET r = FN c_fclose(fp_stderr)
220 LET r = FN c_fclose(fp_stdout)
230 PRINT "Done."
240 END
