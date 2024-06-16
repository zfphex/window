use window::*;

//https://rust-tutorials.github.io/triangle-from-scratch/opening_a_window/win32.html
fn main() {
    let mut window = create_window("Window", 0, 0, 600, 400);

    let context = unsafe { GetDC(window.hwnd) };
    let mut area = window.client_area();
    let mut width = area.width() as usize;
    let mut height = area.height() as usize;
    let mut buffer = vec![0u32; width * height];
    let mut bitmap = create_bitmap(width as i32, height as i32);
    buffer.fill(0x305679);

    loop {
        let new_area = window.client_area();
        if area != new_area {
            area = new_area;
            width = area.width() as usize;
            height = area.height() as usize;
            buffer.clear();
            buffer.resize(width * height * std::mem::size_of::<RGBQUAD>(), 0);
            buffer.fill(0x305679);
            bitmap = create_bitmap(width as i32, height as i32);

            println!(
                "resized: client: {:?}\nscreen: {:?} ",
                area,
                screen_area(window.hwnd)
            );
        }

        unsafe {
            StretchDIBits(
                context,
                0,
                0,
                width as i32,
                height as i32,
                0,
                0,
                width as i32,
                height as i32,
                buffer.as_mut_ptr() as *const VOID,
                &bitmap,
                0,
                SRCCOPY,
            )
        };
        // println!("{:?}", mouse_pos());
        // let screen = screen_area(window.hwnd);
        // println!(
        //     "screen: {:?}, {}, {}",
        //     screen,
        //     screen.width(),
        //     screen.height()
        // );
        // let client = client_area(window.hwnd);
        // println!(
        //     "client: {:?}, {}, {}",
        //     client,
        //     client.width(),
        //     client.height()
        // );
        println!("{:?}", window.screen_mouse_pos);
        match window.event(None) {
            Some(Event::Quit) => break,
            _ => {}
        }
        // match event(None) {
        //     Some(Event::Mouse(x, y)) => println!("{} {}", x, y),
        //     Some(Event::Quit) => break,
        //     _ => {}
        // }
    }
}
