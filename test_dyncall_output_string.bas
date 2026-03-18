10 REM Test dyncall with output string argument
20 DEF XFN GetTempPathA("kernel32.dll|GetTempPathA|u32,ocstr=arg0|u32|")
30 LET buffer = ""
40 LET pathlen = FN GetTempPathA(260, buffer)
50 PRINT "Temp path length: "; pathlen
60 PRINT "Temp path: "; buffer
