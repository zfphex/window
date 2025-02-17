use window::*;

fn main() {
    let mut window = create_window("Window", 50, 50, WindowStyle::BORDERLESS);

    loop {
        match window.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            Some(Event::Input(key, modifiers)) => println!("{:?} {:?}", key, modifiers),
            _ => {}
        }

        window.buffer.fill(0x4fa3a8);
        window.draw();
    }
}
