   10 rem Download a Seattle skyline photo and display it in the default image viewer
   20 rem Uses URLDownloadToFileA (urlmon.dll) and ShellExecuteA (shell32.dll)

   30 def xfn download("urlmon.dll|URLDownloadToFileA|ptr,cstr,cstr,u32,ptr|i32|")
   40 def xfn shellopen("shell32.dll|ShellExecuteA|ptr,cstr,cstr,ptr,ptr,i32|ptr|")

   50 let url$ = "https://upload.wikimedia.org/wikipedia/commons/thumb/e/e3/Seattle_Kerry_Park_Skyline.jpg/1280px-Seattle_Kerry_Park_Skyline.jpg"
   60 let dest$ = "C:\Temp\seattle.jpg"

   70 print "Downloading Seattle skyline..."
   80 let r = fn download(0, url$, dest$, 0, 0)
   90 if r <> 0 then print "Download failed, error: "; r : end

  100 print "Download complete. Opening image..."
  110 let r = fn shellopen(0, "open", dest$, 0, 0, 1)
  120 print "Done."
