use window::*;

fn main() {
    let window = create_window(
        "Window",
        0,
        0,
        800,
        600,
        // WindowStyle::DEFAULT,
        WindowStyle::DEFAULT.ex_style(WS_EX_LAYERED),
    );

    //This will cause lag when resizing the window if mouse polling rate >125hz.
    //Probably better to hand roll this.
    unsafe { set_acrylic(window.hwnd, ACCENT_ENABLE_ACRYLICBLURBEHIND, 0x22282936) };

    //Cannot drag the window when using this?
    // unsafe { SetLayeredWindowAttributes(window.hwnd, 0, 128, LWA_ALPHA) };

    loop {
        match window.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            Some(Event::Input(key, modifiers)) => println!("{:?} {:?}", key, modifiers),
            _ => {}
        }

        // window.buffer.fill(0);
        // window.draw();
    }
}
