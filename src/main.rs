use win_window::*;

//https://rust-tutorials.github.io/triangle-from-scratch/opening_a_window/win32.html
fn main() {
    let _window = create_window("test window", 600, 400, WS_VISIBLE | WS_OVERLAPPEDWINDOW);

    loop {
        match event() {
            Event::Quit => break,
            _ => {}
        }
    }
}
