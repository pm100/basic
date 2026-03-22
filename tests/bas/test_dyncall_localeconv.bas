10 REM Test: localeconv returns *{cstr,cstr} - fields accessible via struct-backed array
20 REM localeconv() takes no args; the struct return is assigned directly to lc.
30 REM Fields: decimal_point (cstr), thousands_sep (cstr)
40 DEF XFN localeconv("msvcrt.dll|localeconv||*{cstr,cstr}|")
50 LET lc = FN localeconv()
60 PRINT "decimal_point: "; lc(0)
70 PRINT "thousands_sep: ["; lc(1); "]"
80 IF lc(0) = "." THEN PRINT "locale ok"
REM EXPECT_CONTAINS: decimal_point: .
REM EXPECT_CONTAINS: thousands_sep:
REM EXPECT_CONTAINS: locale ok
