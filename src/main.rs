use window::*;

static mut WIDTH: i32 = 0;
static mut HEIGHT: i32 = 0;

//https://rust-tutorials.github.io/triangle-from-scratch/opening_a_window/win32.html
fn main() {
    let window = unsafe { create_window("Window", 600, 400) };

    let hwnd = window.hwnd.clone();
    let area = window.client_area();
    let mut width = area.width();
    let mut height = area.height();
    unsafe {
        WIDTH = width;
        HEIGHT = height;
    }

    std::thread::spawn(move || unsafe {
        let context = GetDC(hwnd);
        let mut bitmap = BITMAPINFOHEADER::new(width, height);
        let mut buffer = vec![0u32; width as usize * height as usize];
        buffer.fill(0x305679);

        loop {
            if width != WIDTH || height != HEIGHT {
                width = WIDTH;
                height = HEIGHT;
                buffer.clear();
                buffer.resize(width as usize * height as usize, 0);
                buffer.fill(0x305679);
                bitmap = BITMAPINFOHEADER::new(width, height);
            }

            StretchDIBits(
                context,
                0,
                0,
                width,
                height,
                0,
                0,
                width,
                height,
                buffer.as_mut_ptr() as *const c_void,
                &bitmap,
                0,
                SRCCOPY,
            );
        }
    });

    loop {
        // dbg!(window.client_area());
        match window.event() {
            Some(Event::Resize) => {
                let area = window.client_area();
                unsafe {
                    WIDTH = area.width();
                    HEIGHT = area.height();
                }
            }
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            Some(Event::Dpi(dpi)) => {
                println!("Dpi: {:?}", dpi);
                println!("Scale factor: {}", window.scale_factor());
            }
            Some(Event::Input(key, modifiers)) => println!("{:?} {:?}", key, modifiers),
            _ => {}
        }
    }
}
