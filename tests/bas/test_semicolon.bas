10 REM Test semicolon support in PRINT
20 PRINT "=========================================="
30 PRINT "PRINT Semicolon Test"
40 PRINT "=========================================="
50 PRINT ""
60 REM Test 1: Multiple prints on same line
70 PRINT "Hello";
80 PRINT " ";
90 PRINT "World"
100 PRINT ""
110 REM Test 2: Numbers without newlines
120 PRINT "Counting: ";
130 PRINT 1;
140 PRINT " ";
150 PRINT 2;
160 PRINT " ";
170 PRINT 3
180 PRINT ""
190 REM Test 3: Building a line
200 PRINT "Name: ";
210 PRINT "John";
220 PRINT " ";
230 PRINT "Age: ";
240 PRINT 25
250 PRINT ""
260 REM Test 4: Creating a simple progress bar
270 PRINT "Loading: [";
280 PRINT "=";
290 PRINT "=";
300 PRINT "=";
310 PRINT "=";
320 PRINT "=";
330 PRINT "]"
340 PRINT ""
350 REM Test 5: Mix of semicolon and normal prints
360 PRINT "Start";
370 PRINT "-Middle-";
380 PRINT "End"
390 PRINT "Next line"
400 PRINT ""
410 PRINT "All semicolon tests passed!"
420 END
