use window::*;

fn main() {
    mini::defer_results!();
    let mut window = create_window("Window", 0, 0, 800, 600, WindowStyle::DEFAULT);
    unsafe { window.init_wgl_debug() };

    loop {
        match window.event() {
            Some(Event::Quit) => break,
            Some(Event::Char(c)) => println!("{c}"),
            _ => {}
        }

        if window.input.pressed(Key::Escape) {
            return;
        }

        if window.input.scroll_delta > 0.0 {
            println!("up");
        } else if window.input.scroll_delta < 0.0 {
            println!("down");
        }

        if window.input.released(Key::Space) {
            println!("up");
        }

        if window.left_mouse.clicked(Rect::new(0, 0, 800, 600)) {
            println!("Pressed");
        }

        window.buffer.fill(0x4fa3a8);
        window.draw();
    }
}
