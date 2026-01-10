10 PRINT "Employee Database Example"
20 PRINT "========================="
30 PRINT ""
40 DATA "Alice Johnson", 50000, 5
50 DATA "Bob Smith", 65000, 10
60 DATA "Charlie Brown", 48000, 3
70 DATA "Diana Prince", 72000, 8
80 PRINT "Employee Report:"
90 PRINT ""
100 FOR I = 1 TO 4
110 READ NAME, SALARY, YEARS
120 PRINT "Name: "
130 PRINT NAME
140 PRINT "Salary: $"
150 PRINT SALARY
160 PRINT "Years of service: "
170 PRINT YEARS
180 LET BONUS = SALARY * 0.05 * YEARS
190 PRINT "Bonus: $"
200 PRINT BONUS
210 PRINT ""
220 NEXT I
230 PRINT "End of report."
240 END
