use std::{
    os::raw::c_void,
    time::{Duration, Instant},
};
use window::*;
// use windows::Win32::System::Threading::SetWaitableTimer;

#[link(name = "dwmapi")]
#[link(name = "winmm")]
unsafe extern "system" {
    pub fn DwmFlush() -> i32;
    pub fn timeBeginPeriod(uPeriod: UINT) -> u32;
    pub fn timeEndPeriod(uPeriod: UINT) -> u32;
    pub fn CreateWaitableTimerA(
        lpTimerAttributes: *mut SECURITY_ATTRIBUTES,
        bManualReset: BOOL,
        lpTimerName: LPCSTR,
    ) -> HANDLE;
    pub fn SetWaitableTimer(
        hTimer: HANDLE,
        lpDueTime: *const i64,
        lPeriod: LONG,
        pfnCompletionRoutine: PTIMERAPCROUTINE,
        lpArgToCompletionRoutine: *mut c_void,
        fResume: BOOL,
    ) -> BOOL;
    pub fn WaitForSingleObject(hHandle: HANDLE, dwMilliseconds: DWORD) -> DWORD;
    pub fn QueryPerformanceCounter(lpPerformanceCount: *mut i64) -> BOOL;
    pub fn QueryPerformanceFrequency(lpFrequency: *mut i64) -> BOOL;
}

pub type PTIMERAPCROUTINE = Option<
    unsafe extern "system" fn(
        lpArgToCompletionRoutine: *mut c_void,
        dwTimerLowValue: DWORD,
        dwTimerHighValue: DWORD,
    ),
>;

#[repr(C)]
pub struct SECURITY_ATTRIBUTES {
    pub nLength: DWORD,
    pub lpSecurityDescriptor: *mut c_void,
    pub bInheritHandle: BOOL,
}

fn main() {
    // dwm_flush();
    wait_timer();
}

fn dwm_flush() {
    let window = create_window("Window", 600, 400, WindowStyle::DEFAULT);
    let mut frame_counter = 0;
    let mut last_time = Instant::now();

    loop {
        match window.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            Some(Event::Input(key, modifiers)) => println!("{:?} {:?}", key, modifiers),
            _ => {}
        }

        frame_counter += 1;

        // Sync with the system compositor (VSync)
        unsafe { DwmFlush() };

        // Compute FPS every second
        let elapsed = last_time.elapsed();
        if elapsed >= Duration::from_secs(1) {
            println!("FPS: {}", frame_counter);
            frame_counter = 0;
            last_time = Instant::now();
        }
    }
}

//This is not very accurate, likely as bad as sleep and more complex.
fn wait_timer() {
    unsafe {
        let window = create_window("Window", 600, 400, WindowStyle::DEFAULT);

        assert!(timeBeginPeriod(1) == 0);

        let timer = CreateWaitableTimerA(std::ptr::null_mut(), 1, std::ptr::null());
        assert!(!timer.is_null());

        //TODO: Dynamically get the refresh rate.
        let refresh_rate = 60.0;

        //Measured in 100 nanosecond intervals.
        //Negative because relative.
        let due_time = -(10_000_000.0 / refresh_rate) as i64;

        let mut frame_counter = 0;
        let mut last_time = Instant::now();

        loop {
            match window.event() {
                Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
                Some(Event::Input(key, modifiers)) => println!("{:?} {:?}", key, modifiers),
                _ => {}
            }

            frame_counter += 1;

            let elapsed = last_time.elapsed();
            if elapsed >= Duration::from_secs(1) {
                println!("FPS: {}", frame_counter);
                frame_counter = 0;
                last_time = Instant::now();
            }

            SetWaitableTimer(timer, &due_time, 0, None, core::mem::zeroed(), 0);
            WaitForSingleObject(timer, u32::MAX);
        }
    }

    unsafe { timeEndPeriod(1) }; // Restore normal timing precision
}
