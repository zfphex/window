use window::*;

fn main() {
    let mut window = create_window("Window", 0, 0, 600, 400, WindowStyle::DEFAULT);
    let mut window2 = create_window("Window2", 0, 0, 50, 50, WindowStyle::BORDERLESS);
    let mut point = POINT::default();
    unsafe { GetCursorPos(&mut point) };
    let width = window2.width();
    let height = window2.height();
    window2.set_pos(
        point.x as usize,
        point.y as usize,
        width,
        height,
        SWP_FRAMECHANGED,
    );

    loop {
        //Events need to be polled.
        let _ = window2.event();

        match window.event() {
            Some(Event::Quit) => break,
            _ => {}
        }

        window.draw(|w| {
            w.buffer.fill(0x4fa3a8);
        });

        window2.draw(|w| {
            w.buffer.fill(0x165d6a);
        });
    }
}
