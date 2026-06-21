use window::*;

fn main() {
    let mut window = create_window("Window", 0, 0, 800, 600, WindowStyle::DEFAULT);

    loop {
        match window.sink_events() {
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
        // if input.is_key_held(VK_SPACE as usize) {
        //     println!("up");
        // }

        window.buffer.fill(0x4fa3a8);
        window.draw();
    }
}
