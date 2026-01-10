10 REM Test file I/O
20 OPEN "testout.txt" FOR OUTPUT AS #1
30 PRINT #1, "Hello, File!"
40 PRINT #1, "Line 2"
50 PRINT #1, "Numbers: "; 123; SPC(5); 456
60 CLOSE #1
70 PRINT "File written successfully"
80 END
