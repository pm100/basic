10 REM =====================================================
20 REM  Animals at Play - SDL2 Slideshow
30 REM  5 slides, auto-advance every 4 seconds
40 REM  Any key skips to next slide; Q or Escape exits
50 REM =====================================================

100 REM --- Register SDL2 functions via dyncall ---
110 DEF XFN sdlinit("SDL2.dll|SDL_Init|u32|i32|")
120 DEF XFN createwindow("SDL2.dll|SDL_CreateWindow|cstr,i32,i32,i32,i32,u32|ptr|")
130 DEF XFN createrenderer("SDL2.dll|SDL_CreateRenderer|ptr,i32,u32|ptr|")
140 DEF XFN imgload("SDL2_image.dll|IMG_Load|cstr|ptr|")
150 DEF XFN maketexture("SDL2.dll|SDL_CreateTextureFromSurface|ptr,ptr|ptr|")
160 DEF XFN freesurface("SDL2.dll|SDL_FreeSurface|ptr|void|")
170 DEF XFN renderclear("SDL2.dll|SDL_RenderClear|ptr|i32|")
180 DEF XFN rendercopy("SDL2.dll|SDL_RenderCopy|ptr,ptr,ptr,ptr|i32|")
190 DEF XFN renderpresent("SDL2.dll|SDL_RenderPresent|ptr|void|")
200 DEF XFN destroytex("SDL2.dll|SDL_DestroyTexture|ptr|void|")
210 DEF XFN pollevent("SDL2.dll|SDL_PollEvent|*{i32,i32,i32,i32,i32,i32,i32,i32,i32,i32,i32,i32,i32,i32}|i32|")
220 DEF XFN sdldelay("SDL2.dll|SDL_Delay|u32|void|")
225 DEF XFN getticks("SDL2.dll|SDL_GetTicks||u32|")
230 DEF XFN destroyrenderer("SDL2.dll|SDL_DestroyRenderer|ptr|void|")
240 DEF XFN destroywindow("SDL2.dll|SDL_DestroyWindow|ptr|void|")
250 DEF XFN sdlquit("SDL2.dll|SDL_Quit||void|")
260 DEF XFN mci("winmm.dll|mciSendStringA|cstr,ptr,u32,ptr|u32|")

300 REM --- SDL event type constants ---
310 LET QUIT_EVT = 256
320 LET KEY_EVT = 768

400 REM --- Initialise SDL (SDL_INIT_VIDEO = 32) ---
410 LET RC = FN sdlinit(32)
420 IF RC <> 0 THEN PRINT "SDL_Init failed": GOTO 9000

500 REM --- Create window 800x600, centred (SDL_WINDOWPOS_CENTERED = 0x2FFF0000 = 805240832, SDL_WINDOW_SHOWN = 4) ---
510 LET WIN = FN createwindow("Animals at Play", 805240832, 805240832, 800, 600, 4)
520 IF WIN = 0 THEN PRINT "SDL_CreateWindow failed": GOTO 8900

600 REM --- Create accelerated renderer (flag SDL_RENDERER_ACCELERATED = 2) ---
610 LET REN = FN createrenderer(WIN, -1, 2)
620 IF REN = 0 THEN PRINT "SDL_CreateRenderer failed": GOTO 8800

700 REM --- Image file paths ---
710 DIM IMG$(4)
720 LET IMG$(0) = "demos/animals/dog_play.jpg"
730 LET IMG$(1) = "demos/animals/cat_play.jpg"
740 LET IMG$(2) = "demos/animals/puppy_play.jpg"
750 LET IMG$(3) = "demos/animals/kitten_play.jpg"
760 LET IMG$(4) = "demos/animals/otter_play.jpg"

800 REM --- Caption labels ---
810 DIM CAP$(4)
820 LET CAP$(0) = "A dog having the time of his life!"
830 LET CAP$(1) = "A cat on the prowl."
840 LET CAP$(2) = "Puppies pile-up!"
850 LET CAP$(3) = "Kittens at play."
860 LET CAP$(4) = "Otter just wants to have fun."

900 REM --- Transition sounds (Windows Media, non-blocking) ---
910 DIM SND$(4)
920 LET SND$(0) = "C:/Windows/Media/ding.wav"
930 LET SND$(1) = "C:/Windows/Media/chimes.wav"
940 LET SND$(2) = "C:/Windows/Media/chord.wav"
950 LET SND$(3) = "C:/Windows/Media/tada.wav"
960 LET SND$(4) = "C:/Windows/Media/notify.wav"

1000 REM --- Load all textures up front ---
1010 DIM TEX(4)
1020 LET NSLIDES = 5
1030 FOR I = 0 TO 4
1040   LET SURF = FN imgload(IMG$(I))
1050   IF SURF = 0 THEN PRINT "Failed to load "; IMG$(I): GOTO 8700
1060   LET TEX(I) = FN maketexture(REN, SURF)
1070   FN freesurface(SURF)
1080   IF TEX(I) = 0 THEN PRINT "Failed to create texture for "; IMG$(I): GOTO 8700
1090 NEXT I
1100 PRINT "All images loaded. Starting slideshow..."

1200 REM --- Slideshow main loop (GOTO-based, no nested FOR) ---
1205 LET DONE = 0
1206 LET SLIDE = 0
1210 DIM EVT(13)

1300 REM --- Show slide SLIDE ---
1310 PRINT CAP$(SLIDE)

1400 LET OPENCMD$ = "open " + SND$(SLIDE) + " type waveaudio alias sndslide"
1410 FN mci(OPENCMD$, 0, 0, 0)
1420 FN mci("play sndslide", 0, 0, 0)

1500 FN renderclear(REN)
1510 FN rendercopy(REN, TEX(SLIDE), 0, 0)
1520 FN renderpresent(REN)

1600 REM --- Poll loop: pre-compute deadline ---
1610 LET T0 = FN getticks()
1615 LET TEND = T0 + 4000

1620 LET TNOW = FN getticks()
1625 IF TNOW >= TEND THEN 1750
1630 LET GOT = FN pollevent(EVT)
1640 IF GOT = 0 THEN 1680
1642 LET ETYPE = EVT(0)
1650 IF ETYPE = QUIT_EVT THEN 1720
1660 IF ETYPE = KEY_EVT THEN 1750
1670 GOTO 1620
1680 FN sdldelay(50)
1690 GOTO 1620
1720 LET DONE = 1
1730 GOTO 1750

1750 FN mci("close sndslide", 0, 0, 0)
1760 IF DONE = 1 THEN 2010
1770 LET SLIDE = SLIDE + 1
1780 IF SLIDE = 5 THEN LET SLIDE = 0
1790 GOTO 1310

2000 REM --- Cleanup ---
2010 PRINT "Slideshow finished. Cleaning up..."
2020 FOR I = 0 TO 4
2030   IF TEX(I) <> 0 THEN FN destroytex(TEX(I))
2040 NEXT I
2050 FN destroyrenderer(REN)
2060 FN destroywindow(WIN)
2070 FN sdlquit()
2080 PRINT "Done."
2090 END

8700 REM --- Error: texture load failed, release textures loaded so far ---
8710 FOR I = 0 TO 4
8720   IF TEX(I) <> 0 THEN FN destroytex(TEX(I))
8730 NEXT I
8800 FN destroyrenderer(REN)
8900 FN destroywindow(WIN)
9000 FN sdlquit()
9010 PRINT "Exiting due to error."
9020 END
