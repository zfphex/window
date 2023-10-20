use win_window::*;

//https://rust-tutorials.github.io/triangle-from-scratch/opening_a_window/win32.html
fn main() {
    let window = create_window("test window", 600, 400, WS_VISIBLE | WS_OVERLAPPEDWINDOW);
    let mut msg = MSG::default();

    loop {
        let message_result = unsafe { GetMessageA(&mut msg, 0, 0, 0) };
        if message_result == 0 {
            break;
        } else if message_result == -1 {
            let last_error = unsafe { GetLastError() };
            panic!("Error with `GetMessageA`, error code: {}", last_error);
        } else {
            //Handle message here.
            unsafe {
                TranslateMessage(&mut msg);
                DispatchMessageA(&mut msg);
            }
        }
    }
}
