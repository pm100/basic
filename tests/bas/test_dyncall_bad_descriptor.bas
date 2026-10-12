10 REM Invalid descriptors report an error and do not crash the interpreter
20 DEF XFN bad1("msvcrt.dll|strcpy|ocstr,cstr|ptr|")
30 PRINT "after missing length"
40 DEF XFN bad2("msvcrt.dll|printf|cstr,f32|i32|fixargs=1")
50 PRINT "after promoted variadic"
60 DEF XFN bad3("no_such_lib_xyz.dll|f||void|")
70 PRINT "after missing library"
80 DEF XFN bad4("msvcrt.dll|memset|ocstr=arg7,i32,u64|ptr|")
90 PRINT "after bad length arg"
100 DEF XFN ok("msvcrt.dll|abs|i32|i32|")
110 PRINT "abs: "; FN ok(0-9)
REM EXPECT: after missing length
REM EXPECT: after promoted variadic
REM EXPECT: after missing library
REM EXPECT: after bad length arg
REM EXPECT: abs: 9
