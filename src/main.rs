use win_window::*;

//https://rust-tutorials.github.io/triangle-from-scratch/opening_a_window/win32.html
fn main() {
    create_window("test window", 600, 400, WS_VISIBLE | WS_OVERLAPPEDWINDOW);
    let mut msg = MSG::default();

    loop {
        window_event(&mut msg);
        //TODO: https://docs.rs/winapi/latest/src/winapi/um/winuser.rs.html#1267
        // match msg.message {
        // }
    }
}
