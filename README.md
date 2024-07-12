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


## WndProc Hell and Message Handling

Not all messages can be intercepted using `PeekMessage`, some messages like `WM_MOVE` and `WM_RESIZE` must be handled using the specified window callback. 

## [Why does the primary monitor have (0,0) as its upper left coordinate?](https://devblogs.microsoft.com/oldnewthing/20100820-00/?p=13093)

Programs which pass CW_USEDEFAULT to the CreateWindow function explicitly abdicated the choice of the window position and therefore the monitor. The window manager tries to guess an appropriate monitor. If the new window has a parent or owner, then it is placed on the same monitor as that parent or owner. 

## [High DPI Desktop Application Development on Windows](https://learn.microsoft.com/en-us/windows/win32/hidpi/high-dpi-desktop-application-development-on-windows)

Desktop applications must tell Windows if they support DPI scaling. By default, the system considers desktop applications DPI unaware and bitmap-stretches their windows.

There are two versions of Per-Monitor awareness that an application can register itself as: version 1 and version 2 (PMv2). Registering a process as running in PMv2 awareness mode results in:

The application being notified when the DPI changes (both the top-level and child HWNDs)

1. The application seeing the raw pixels of each display
2. The application never being bitmap scaled by Windows
3. Automatic non-client area (window caption, scroll bars, etc.) DPI scaling by Windows
4. Win32 dialogs (from CreateDialog) automatically DPI scaled by Windows
5. Theme-drawn bitmap assets in common controls (checkboxes, button backgrounds, etc.) being automatically rendered at the appropriate DPI scale factor

| DPI Awareness Mode | Windows Version Introduced | Application's view of DPI | Behavior on DPI change |
| --- | --- | --- | --- |
| Unaware | N/A | All displays are 96 DPI | Bitmap-stretching \(blurry\) |
| System | Vista | All displays have the same DPI \(the DPI of the primary display at the time the current user session was started\) | Bitmap-stretching \(blurry\) |
| Per-Monitor | 8.1 | The DPI of the display that the application window is primarily located on | Top-level HWND is notified of DPI change\. No DPI scaling of any UI elements\. |
| Per-Monitor V2 | Windows 10 Creators Update \(1703\) | The DPI of the display that the application window is primarily located on | Top-level and child HWNDs are notified of DPI change\. Automatic DPI scaling of\: - Non-client area- Theme-drawn bitmaps in common controls \(comctl32 V6\)  - Dialogs \(CreateDialog\) |

DPI change notification ([WM_DPICHANGED](https://learn.microsoft.com/en-us/windows/win32/hidpi/wm-dpichanged)). 

| Single DPI version | Per-Monitor version |
| --- | --- |
| GetSystemMetrics | GetSystemMetricsForDpi |
| AdjustWindowRectEx | AdjustWindowRectExForDpi |
| SystemParametersInfo | SystemParametersInfoForDpi |
| GetDpiForMonitor | GetDpiForWindow |


### Mixed-Mode DPI Scaling

It can sometimes become impractical or impossible to update every window in the application in one go. This can simply be due to the time and effort required to update and test all UI, or if your application perhaps loads third-party UI.
Windows offers a way to run some of your application windows (top-level only) in their original DPI-awareness mode while other's are DPI-aware.

To enable sub-process DPI awareness, call [SetThreadDpiAwarenessContext](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setthreaddpiawarenesscontext) before and after any window creation calls.

### Using DPI Awareness

[SetThreadDpiAwarenessContext](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setthreaddpiawarenesscontext)



### Common Pitfalls

When Windows sends your application window a WM_DPICHANGED message, this message includes a suggested rectangle that you should use to resize your window. It is critical that your application use this rectangle to resize itself, as this will:

1. Ensure that the mouse cursor will stay in the same relative position on the Window when dragging between displays
2. Prevent the application window from getting into a recursive dpi-change cycle where one DPI change triggers a subsequent DPI change, which triggers yet another DPI change.
3. If you have application-specific requirements that prevent you from using the suggested rectangle that Windows provides in the WM_DPICHANGED message, see [WM_GETDPISCALEDSIZE](https://learn.microsoft.com/en-us/windows/win32/hidpi/wm-getdpiscaledsize). This message can be used to give Windows a desired size you'd like used once the DPI change has occurred, while still avoiding the issues described above.


### Framerate Limiting

https://learn.microsoft.com/en-us/windows/win32/directcomp/compositor-clock/compositor-clock
https://learn.microsoft.com/en-us/windows/win32/api/timeapi/nf-timeapi-timebeginperiod
https://www.gdcvault.com/play/1025031/Advanced-Graphics-Techniques-Tutorial-The
https://learn.microsoft.com/en-us/windows/win32/comp_swapchain/comp-swapchain-examples#example-11synchronizing-presentation-using-buffer-available-events-and-handling-presentation-manager-lost-events
https://hero.handmade.network/forums/game-discussion/t/8612-what_does_timebeginperiod_actually_do
https://randomascii.wordpress.com/2013/07/08/windows-timer-resolution-megawatts-wasted/


### Window cannot update while being dragged or resized

https://github.com/glfw/glfw/pull/1426
https://github.com/libsdl-org/SDL/commit/509c70c6982b6927f5a8d4fb32f9319cbaf0c2ef