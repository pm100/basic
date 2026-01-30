10 PRINT "Practical CHR$/ASC Examples"
20 PRINT "==========================="
30 PRINT ""
40 REM Example 1: Simple encryption/decryption
50 PRINT "Example 1: Caesar Cipher"
60 PRINT "------------------------"
70 LET MESSAGE = "ABC"
80 PRINT "Original: "
90 PRINT MESSAGE
100 REM Encrypt by shifting 3 positions
110 LET C1 = ASC(LEFT$(MESSAGE, 1)) + 3
120 LET C2 = ASC(MID$(MESSAGE, 2, 1)) + 3
130 LET C3 = ASC(MID$(MESSAGE, 3, 1)) + 3
140 LET ENCRYPTED = CHR$(C1) + CHR$(C2) + CHR$(C3)
150 PRINT "Encrypted: "
160 PRINT ENCRYPTED
170 REM Decrypt by shifting back
180 LET D1 = ASC(LEFT$(ENCRYPTED, 1)) - 3
190 LET D2 = ASC(MID$(ENCRYPTED, 2, 1)) - 3
200 LET D3 = ASC(MID$(ENCRYPTED, 3, 1)) - 3
210 LET DECRYPTED = CHR$(D1) + CHR$(D2) + CHR$(D3)
220 PRINT "Decrypted: "
230 PRINT DECRYPTED
240 PRINT ""
250 REM Example 2: Check if character is uppercase
260 PRINT "Example 2: Character classification"
270 PRINT "------------------------------------"
280 LET TEST = "A"
290 LET CODE = ASC(TEST)
300 PRINT "Character: "
310 PRINT TEST
320 PRINT "ASCII code: "
330 PRINT CODE
340 IF CODE >= 65 THEN 360
350 GOTO 380
360 IF CODE <= 90 THEN 390
370 GOTO 380
380 PRINT "Not uppercase"
390 PRINT "Is uppercase letter!"
400 PRINT ""
410 REM Example 3: Generate alphabet
420 PRINT "Example 3: Generate alphabet"
430 PRINT "----------------------------"
440 LET ALPHABET = ""
450 FOR I = 65 TO 90
460 LET ALPHABET = ALPHABET + CHR$(I)
470 NEXT I
480 PRINT ALPHABET
490 PRINT ""
500 PRINT "Examples complete!"
510 END
