10 PRINT "MID$ Assignment Test"
20 PRINT "===================="
30 PRINT ""
40 LET S = "I love dogs"
50 PRINT "Original: "
60 PRINT S
70 PRINT ""
80 LET POS = INSTR$(S, "love")
90 PRINT "Position of 'love': "
100 PRINT POS
110 PRINT ""
120 MID$(S, POS, 4) = "hate"
130 PRINT "After MID$ assignment: "
140 PRINT S
150 END
