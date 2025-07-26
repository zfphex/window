use window::*;

fn main() {
    let mut window = create_window("Window", 0, 0, 600, 400, WindowStyle::DEFAULT);
    // let mut window2 = create_window("Window2", 0, 0, 600, 400, WindowStyle::DEFAULT);

    loop {
        // match window2.event() {
        //     _ => {}
        // }

        match window.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            Some(Event::Input(key, modifiers)) => println!("{:?} {:?}", key, modifiers),
            _ => {}
        }

        window.buffer.fill(0x4fa3a8);
        window.draw();

        // window2.buffer.fill(0x165d6a);
        // window2.draw();
        return
    }
}
