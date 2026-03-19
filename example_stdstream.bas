10  REM Standard stream example - write to stderr/stdout via C library (ucrtbase.dll)
20  REM On Windows, call __acrt_iob_func(n) to get the FILE* for a standard stream:
30  REM   0=stdin, 1=stdout, 2=stderr
40  REM Then pass the returned pointer to fputs/fflush etc. from the same DLL.
50  PRINT "Standard stream example"
60  DEF XFN c_iob("ucrtbase.dll|__acrt_iob_func|u32|ptr|coerce")
70  DEF XFN c_fputs("ucrtbase.dll|fputs|cstr,ptr|i32|")
80  DEF XFN c_fflush("ucrtbase.dll|fflush|ptr|i32|")
90  REM Get FILE* for stderr (index 2) and stdout (index 1)
100 LET fp_stderr = FN c_iob(2)
110 LET fp_stdout = FN c_iob(1)
120 REM fputs to stderr writes directly to the console
130 LET r = FN c_fputs("Hello from BASIC via fputs (stderr)", fp_stderr)
140 PRINT "fputs returned: "; r
150 LET r = FN c_fputs("Result of 6 * 7 = 42", fp_stderr)
160 PRINT "fputs returned: "; r
170 REM fflush stdout to ensure buffered output is written
180 LET r = FN c_fflush(fp_stdout)
190 PRINT "fflush stdout returned: "; r
200 PRINT "Done."
210 END
