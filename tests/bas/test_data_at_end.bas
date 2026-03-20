10 PRINT "Testing DATA statements at end of program"
20 PRINT "=========================================="
30 PRINT ""
40 PRINT "Reading employee data (DATA is at line 200+):"
50 PRINT ""
60 FOR I = 1 TO 3
70 READ EMPNAME, EMPID, SALARY
80 PRINT "Employee: "
90 PRINT EMPNAME
100 PRINT "ID: "
110 PRINT EMPID
120 PRINT "Salary: $"
130 PRINT SALARY
140 PRINT ""
150 NEXT I
160 PRINT "All data read successfully!"
170 PRINT ""
180 PRINT "This proves DATA statements work anywhere in the program."
190 END
200 REM === DATA SECTION AT END ===
210 DATA "Alice Johnson", 1001, 75000
220 DATA "Bob Smith", 1002, 68000
230 DATA "Carol White", 1003, 82000
