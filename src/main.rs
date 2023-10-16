use win_window::*;

//https://rust-tutorials.github.io/triangle-from-scratch/opening_a_window/win32.html
fn main() {
    create_window("test window", WS_VISIBLE | WS_OVERLAPPEDWINDOW);
    let mut msg = MSG::default();

    loop {
        let message_return = unsafe { GetMessageA(&mut msg, 0, 0, 0) };
        if message_return == 0 {
            break;
        } else if message_return == -1 {
            let last_error = unsafe { GetLastError() };
            panic!("Error with `GetMessageW`, error code: {}", last_error);
        } else {
            unsafe {
                TranslateMessage(&msg);
                DispatchMessageA(&msg);
            }
        }
    }
}
