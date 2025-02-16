![](img/coordinates.png)

There are two types of coordinates that are important when working with windows.

Screen coordinates (the position on the monitor)
Client coordinates (the position inside the window)

The top left corner for client coordinates will always be (0, 0). Client coordinates will be used for all of your drawing/rendering logic.

Screen coordinates are used when creating the window. So if you want to create a framebuffer with a with of 800 and height of 600. You will need to convert those coordinates to screen coordinates.

The width and height will be slightly larger since they need to include the title bar and window borders. Note that if you have borders disabled, for example using fullscreen windowed, the width and height of the window should match using both client and screen coordinates. Note that the X and Y position may be different because screen coordinates would take in the position of the monitor and client coordinates always have an X and Y of (0, 0).

For my primary display, the top left corner has a screen coordiante of x: -8, y: 0. I wonder if that's because of the window borders. I thought it would be (0, 0)
For some reason it's (-8, -8) maximized but (-8, 0) windowed. I do not understand.

### Why is (0, 0) in screen coordinates not the top corner?

https://github.com/dotnet/winforms/issues/4776

In Windows 10 the design was changed so that window borders would have thin (1px at 96 DPI) borders.

Literally switching to 1px of non-client area on the edges wouldn't work out because grabbing the edges with the mouse would be too hard. So, the amount of non-client space on the edges was left as 8px wide, but 7px of that space is now transparent. The programmatic edge of the window is now in a different position than the visible edge of the window.

Apps can query the amount of this transparent space with DwmGetWindowAttribute + DWMWA_EXTENDED_FRAME_BOUNDS. But note that that API is NOT DPI virtualized, and will return raw physical pixels regardless of the DPI awareness mode of the caller. That means that it will work fine for per-monitor-aware apps, but will return confusing answers to unaware or system-aware apps.

Source: [Location 0, 0 not working on windows 10
](https://github.com/dotnet/winforms/issues/4776#issuecomment-1227637666)

## Message Handling

Not all messages can be intercepted using `PeekMessage`, some messages like `WM_MOVE` and `WM_RESIZE` must be handled using the specified window callback.

## [Why does the primary monitor have (0,0) as its upper left coordinate?](https://devblogs.microsoft.com/oldnewthing/20100820-00/?p=13093)

Programs which pass CW_USEDEFAULT to the CreateWindow function explicitly abdicated the choice of the window position and therefore the monitor. The window manager tries to guess an appropriate monitor. If the new window has a parent or owner, then it is placed on the same monitor as that parent or owner.

### Framerate Limiting

https://learn.microsoft.com/en-us/windows/win32/directcomp/compositor-clock/compositor-clock
https://learn.microsoft.com/en-us/windows/win32/api/timeapi/nf-timeapi-timebeginperiod
https://www.gdcvault.com/play/1025031/Advanced-Graphics-Techniques-Tutorial-The
