use window::*;

extern "system" {
    fn glClearColor(r: f32, g: f32, b: f32, a: f32);
    fn glClear(mask: u32);
}

const GL_COLOR_BUFFER_BIT: u32 = 0x0000_4000;

fn main() {
    let window = create_window("WGL", 0, 0, 800, 600, WindowStyle::DEFAULT);

    let mut t: f32 = 0.0;
    loop {
        match window.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            _ => {}
        }

        t += 0.02;
        let r = (t.sin() * 0.5 + 0.5) as f32;
        let g = (t.cos() * 0.5 + 0.5) as f32;
        unsafe {
            glClearColor(r, g, 0.25, 1.0);
            glClear(GL_COLOR_BUFFER_BIT);
        }
        window.swap_buffers();
    }
}
