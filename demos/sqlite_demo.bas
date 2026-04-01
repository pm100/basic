10 REM ============================================
20 REM  SQLite Demo - Contact Book
30 REM  Uses dyncall FFI to call sqlite3 C API
40 REM ============================================
50 REM
100 REM --- Define SQLite3 external functions ---
110 DEF XFN dbopen("sqlite3.dll|sqlite3_open|cstr,*i64|i32|")
120 DEF XFN dbclose("sqlite3.dll|sqlite3_close|ptr|i32|")
130 DEF XFN dbexec("sqlite3.dll|sqlite3_exec|ptr,cstr,ptr,ptr,ptr|i32|")
140 DEF XFN dbprep("sqlite3.dll|sqlite3_prepare_v2|ptr,cstr,i32,*i64,ptr|i32|")
150 DEF XFN dbstep("sqlite3.dll|sqlite3_step|ptr|i32|")
160 DEF XFN dbcoltxt("sqlite3.dll|sqlite3_column_text|ptr,i32|cstr|")
170 DEF XFN dbcolint("sqlite3.dll|sqlite3_column_int|ptr,i32|i32|")
180 DEF XFN dbfinal("sqlite3.dll|sqlite3_finalize|ptr|i32|")
190 DEF XFN dberr("sqlite3.dll|sqlite3_errmsg|ptr|cstr|")
200 REM
210 REM --- SQLite constants ---
220 LET SQLITE_OK = 0
230 LET SQLITE_ROW = 100
240 LET SQLITE_DONE = 101
250 REM
300 REM ============================================
310 REM  Open database
320 REM ============================================
330 LET DB = 0
340 LET RC = FN dbopen("contacts.db", DB)
350 IF RC <> SQLITE_OK THEN GOTO 9000
360 PRINT "Opened database: contacts.db"
370 PRINT
400 REM ============================================
410 REM  Create contacts table
420 REM ============================================
430 LET S$ = "CREATE TABLE IF NOT EXISTS contacts ("
440 LET S$ = S$ + "id INTEGER PRIMARY KEY AUTOINCREMENT, "
450 LET S$ = S$ + "name TEXT NOT NULL, "
460 LET S$ = S$ + "phone TEXT, "
470 LET S$ = S$ + "email TEXT)"
480 LET RC = FN dbexec(DB, S$, 0, 0, 0)
490 IF RC <> SQLITE_OK THEN GOTO 9100
500 PRINT "Table 'contacts' ready."
510 PRINT
600 REM ============================================
610 REM  Insert sample contacts
620 REM ============================================
630 LET RC = FN dbexec(DB, "DELETE FROM contacts", 0, 0, 0)
640 LET RC = FN dbexec(DB, "INSERT INTO contacts (name, phone, email) VALUES ('Alice Smith', '555-0101', 'alice@example.com')", 0, 0, 0)
650 LET RC = FN dbexec(DB, "INSERT INTO contacts (name, phone, email) VALUES ('Bob Jones', '555-0102', 'bob@example.com')", 0, 0, 0)
660 LET RC = FN dbexec(DB, "INSERT INTO contacts (name, phone, email) VALUES ('Carol White', '555-0103', 'carol@example.com')", 0, 0, 0)
670 LET RC = FN dbexec(DB, "INSERT INTO contacts (name, phone, email) VALUES ('Dave Brown', '555-0104', 'dave@example.com')", 0, 0, 0)
680 LET RC = FN dbexec(DB, "INSERT INTO contacts (name, phone, email) VALUES ('Eve Green', '555-0105', 'eve@example.com')", 0, 0, 0)
690 PRINT "Inserted 5 contacts."
700 PRINT
800 REM ============================================
810 REM  Query: show all contacts
820 REM ============================================
830 PRINT "=== All Contacts ==="
840 PRINT "ID   Name             Phone        Email"
850 PRINT "---  ---------------  -----------  --------------------"
860 LET STMT = 0
870 LET RC = FN dbprep(DB, "SELECT id, name, phone, email FROM contacts ORDER BY name", -1, STMT, 0)
880 IF RC <> SQLITE_OK THEN GOTO 9200
890 LET RC = FN dbstep(STMT)
900 IF RC <> SQLITE_ROW THEN GOTO 940
905 LET CID = FN dbcolint(STMT, 0)
906 LET CNAME$ = FN dbcoltxt(STMT, 1)
907 LET CPHONE$ = FN dbcoltxt(STMT, 2)
908 LET CEMAIL$ = FN dbcoltxt(STMT, 3)
910 PRINT CID; TAB(5); CNAME$; TAB(22); CPHONE$; TAB(34); CEMAIL$
920 GOTO 890
940 LET RC = FN dbfinal(STMT)
950 PRINT
1000 REM ============================================
1010 REM  Update: change Bob's phone number
1020 REM ============================================
1030 LET RC = FN dbexec(DB, "UPDATE contacts SET phone = '555-9999' WHERE name = 'Bob Jones'", 0, 0, 0)
1040 IF RC <> SQLITE_OK THEN GOTO 9100
1050 PRINT "Updated Bob's phone to 555-9999."
1060 PRINT
1100 REM ============================================
1110 REM  Delete: remove Dave
1120 REM ============================================
1130 LET RC = FN dbexec(DB, "DELETE FROM contacts WHERE name = 'Dave Brown'", 0, 0, 0)
1140 IF RC <> SQLITE_OK THEN GOTO 9100
1150 PRINT "Deleted Dave Brown."
1160 PRINT
1200 REM ============================================
1210 REM  Query: show updated contacts
1220 REM ============================================
1230 PRINT "=== After Update & Delete ==="
1240 PRINT "ID   Name             Phone        Email"
1250 PRINT "---  ---------------  -----------  --------------------"
1260 LET STMT = 0
1270 LET RC = FN dbprep(DB, "SELECT id, name, phone, email FROM contacts ORDER BY name", -1, STMT, 0)
1280 IF RC <> SQLITE_OK THEN GOTO 9200
1290 LET RC = FN dbstep(STMT)
1300 IF RC <> SQLITE_ROW THEN GOTO 1340
1305 LET CID = FN dbcolint(STMT, 0)
1306 LET CNAME$ = FN dbcoltxt(STMT, 1)
1307 LET CPHONE$ = FN dbcoltxt(STMT, 2)
1308 LET CEMAIL$ = FN dbcoltxt(STMT, 3)
1310 PRINT CID; TAB(5); CNAME$; TAB(22); CPHONE$; TAB(34); CEMAIL$
1320 GOTO 1290
1340 LET RC = FN dbfinal(STMT)
1350 PRINT
1400 REM ============================================
1410 REM  Query: search by name (LIKE pattern)
1420 REM ============================================
1430 PRINT "=== Search: names containing 'e' ==="
1440 LET STMT = 0
1450 LET RC = FN dbprep(DB, "SELECT name, email FROM contacts WHERE name LIKE '%e%'", -1, STMT, 0)
1460 IF RC <> SQLITE_OK THEN GOTO 9200
1470 LET RC = FN dbstep(STMT)
1480 IF RC <> SQLITE_ROW THEN GOTO 1520
1485 LET CNAME$ = FN dbcoltxt(STMT, 0)
1486 LET CEMAIL$ = FN dbcoltxt(STMT, 1)
1490 PRINT "  "; CNAME$; " - "; CEMAIL$
1500 GOTO 1470
1520 LET RC = FN dbfinal(STMT)
1530 PRINT
1600 REM ============================================
1610 REM  Aggregate: count contacts
1620 REM ============================================
1630 LET STMT = 0
1640 LET RC = FN dbprep(DB, "SELECT COUNT(*) FROM contacts", -1, STMT, 0)
1650 IF RC <> SQLITE_OK THEN GOTO 9200
1660 LET RC = FN dbstep(STMT)
1670 IF RC = SQLITE_ROW THEN PRINT "Total contacts: "; FN dbcolint(STMT, 0)
1680 LET RC = FN dbfinal(STMT)
1690 PRINT
1800 REM ============================================
1810 REM  Close database
1820 REM ============================================
1830 LET RC = FN dbclose(DB)
1840 PRINT "Database closed. Done!"
1850 END
1860 REM
9000 REM --- Error: open failed ---
9010 PRINT "Error opening database"
9020 END
9100 REM --- Error: exec failed ---
9110 PRINT "SQL error: "; FN dberr(DB)
9120 LET RC = FN dbclose(DB)
9130 END
9200 REM --- Error: prepare failed ---
9210 PRINT "Prepare error: "; FN dberr(DB)
9220 LET RC = FN dbclose(DB)
9230 END
