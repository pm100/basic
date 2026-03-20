1 REM SKIP
10 PRINT "Testing 60D Array with Small Bounds"
20 PRINT "===================================="
30 PRINT ""
40 REM Each dimension has bound 0-1 (2 values per dimension)
50 REM Total elements = 2^60 which is too large!
60 PRINT "Note: While BASIC spec allows 60 dimensions,"
70 PRINT "practical memory limits apply."
80 PRINT ""
90 PRINT "Testing smaller realistic example:"
100 PRINT "Creating 20-dimensional array (0-1 each)..."
110 DIM HYPER(1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1)
120 PRINT "Successfully created 20D array"
130 PRINT "Total elements: 2^20 = 1,048,576"
140 PRINT ""
150 PRINT "Setting two values:"
160 LET HYPER(0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0) = 1000
170 LET HYPER(1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1) = 9999
180 PRINT ""
190 PRINT "Origin = "
200 PRINT HYPER(0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0)
210 PRINT "Max corner = "
220 PRINT HYPER(1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1)
230 PRINT ""
240 PRINT "Test complete!"
250 END

