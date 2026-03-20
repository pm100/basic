   10 def xfn msgbox("user32.dll|MessageBoxA|ptr,cstr,cstr,u32|i32|coerce")
   20 print "Showing a Yes/No dialog..."
   30 let result = fn msgbox(0, "Do you like dyncall?", "dyncall BASIC demo", 4)
   40 if result = 6 then print "You clicked Yes!"
   50 if result = 7 then print "You clicked No!"
   60 print "MessageBoxA returned "; result