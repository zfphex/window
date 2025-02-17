use window::*;

fn main() {
    let mut window = create_window("Window", 600, 400, WindowStyle::DEFAULT);
    let mut window2 = create_window("Window2", 50, 50, WindowStyle::BORDERLESS);
    let mut point = POINT::default();
    unsafe { GetCursorPos(&mut point) };
    window2.set_pos(
        point.x as usize,
        point.y as usize,
        window2.width(),
        window2.height(),
        SWP_FRAMECHANGED,
    );

    loop {
        //Events need to be polled.
        let _ = window2.event();

        match window.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            Some(Event::Input(key, modifiers)) => println!("{:?} {:?}", key, modifiers),
            _ => {}
        }

        window.buffer.fill(0x4fa3a8);
        window.draw();

        window2.buffer.fill(0x165d6a);
        window2.draw();
    }
}
