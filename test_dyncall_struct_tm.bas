10 REM Pass a BASIC numeric array as struct tm to mktime/strftime.
20 DIM TM(8)
30 LET TM(0) = 30
40 LET TM(1) = 15
50 LET TM(2) = 8
60 LET TM(3) = 11
70 LET TM(4) = 2
80 LET TM(5) = 124
90 LET TM(6) = 0
100 LET TM(7) = 0
110 LET TM(8) = -1
120 DEF XFN mktime("msvcrt.dll|mktime|*{i32,i32,i32,i32,i32,i32,i32,i32,i32}|i64|")
130 DEF XFN strftime("msvcrt.dll|strftime|ocstr=arg1,u64,cstr,*{i32,i32,i32,i32,i32,i32,i32,i32,i32}|u64|")
140 LET OUT$ = ""
150 LET TS = FN mktime(TM)
160 LET N = FN strftime(OUT$, 64, "%Y-%m-%d %H:%M:%S", TM)
170 PRINT "timestamp: "; TS
180 PRINT "written: "; N
190 PRINT "formatted: "; OUT$
200 PRINT "weekday: "; TM(6)
210 PRINT "yearday: "; TM(7)
220 END
