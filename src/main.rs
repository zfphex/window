use window::*;

static mut WIDTH: i32 = 0;
static mut HEIGHT: i32 = 0;

#[inline(always)]
pub const fn r(color: u32) -> u8 {
    (color >> 16 & 0xFF) as u8
}

#[inline(always)]
pub const fn g(color: u32) -> u8 {
    (color >> 8 & 0xFF) as u8
}

#[inline(always)]
pub const fn b(color: u32) -> u8 {
    (color & 0xFF) as u8
}
pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    (a * (1.0 - t)) + (b * t)
}

//Hex colors don't typicaly contain alpha values.
pub fn lerp_hex(color1: u32, color2: u32, t: f32) -> u32 {
    let r = lerp(r(color1) as f32, r(color2) as f32, t) as u8;
    let g = lerp(g(color1) as f32, g(color2) as f32, t) as u8;
    let b = lerp(b(color1) as f32, b(color2) as f32, t) as u8;

    (r as u32) << 16 | (g as u32) << 8 | (b as u32)
}

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

    //TODO: I want each window to have it's own framebuffer.
    // window.draw(buffer);

    std::thread::spawn(move || unsafe {
        let context = GetDC(hwnd);
        let mut bitmap = BITMAPINFOHEADER::new(width, height);
        let mut buffer = vec![0u32; width as usize * height as usize];
        let mut t: f32 = 0.0;
        let mut fill_color = 0x305679;
        buffer.fill(fill_color);

        loop {
            t += 0.005;
            fill_color = lerp_hex(0x305679f, 0x2158b7, t.sin());
            buffer.fill(fill_color);

            if width != WIDTH || height != HEIGHT {
                width = WIDTH;
                height = HEIGHT;
                buffer.clear();
                buffer.resize(width as usize * height as usize, 0);
                buffer.fill(fill_color);
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
