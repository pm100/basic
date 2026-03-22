10 def xfn beep("kernel32.dll|Beep|u32,u32|i32|")
20 rem Play a simple melody using kernel32 Beep(frequency, duration_ms)
30 rem Notes: C4=262 D4=294 E4=330 F4=349 G4=392 A4=440 B4=494 C5=523
40 print "Playing melody..."
50 rem --- Mary Had a Little Lamb ---
60  fn beep(330, 300) : rem E
70  fn beep(294, 300) : rem D
80  fn beep(262, 300) : rem C
90  fn beep(294, 300) : rem D
100 fn beep(330, 300) : rem E
110 fn beep(330, 300) : rem E
120 fn beep(330, 600) : rem E (long)
130 fn beep(294, 300) : rem D
140 fn beep(294, 300) : rem D
150 fn beep(294, 600) : rem D (long)
160 fn beep(330, 300) : rem E
170 fn beep(392, 300) : rem G
180 fn beep(392, 600) : rem G (long)
190 fn beep(330, 300) : rem E
200 fn beep(294, 300) : rem D
210 fn beep(262, 300) : rem C
220 fn beep(294, 300) : rem D
230 fn beep(330, 300) : rem E
240 fn beep(330, 300) : rem E
250 fn beep(330, 300) : rem E
260 fn beep(330, 300) : rem E
270 fn beep(294, 300) : rem D
280 fn beep(294, 300) : rem D
290 fn beep(330, 300) : rem E
300 fn beep(294, 300) : rem D
310 fn beep(262, 600) : rem C (long)
320 print "Done!"
