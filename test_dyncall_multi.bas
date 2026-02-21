10 REM Test dyncall with multiple arguments
20 REM Example: strcmp function to compare two strings
30 DEF XFN strcmp("msvcrt.dll|strcmp|cstr,cstr|i32|")
40 LET result1 = FN strcmp("hello", "hello")
50 PRINT "strcmp('hello', 'hello') = "; result1
60 LET result2 = FN strcmp("apple", "banana")
70 PRINT "strcmp('apple', 'banana') = "; result2
80 LET result3 = FN strcmp("zebra", "apple")
90 PRINT "strcmp('zebra', 'apple') = "; result3
