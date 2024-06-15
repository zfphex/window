use window::*;

//https://rust-tutorials.github.io/triangle-from-scratch/opening_a_window/win32.html
fn main() {
    let window = create_window("test window", 0, 0, 600, 400);
    // let w2 = create_window("window2", 800, 0, 600, 400);

    let context = unsafe { GetDC(window.hwnd) };
    let area = window.area();
    let width = area.width();
    let height = area.height();
    let mut buffer = vec![0; width as usize * height as usize];
    let bitmap = create_bitmap(width, height);
    buffer.fill(0x305679);

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

    //  let area = Rect::from(self.window.area());
    //         if self.area != area {
    //             self.area = area;
    //             self.width = self.area.width as usize;
    //             self.height = self.area.height as usize;
    //             self.buffer.clear();
    //             self.buffer
    //                 .resize(self.width * self.height * std::mem::size_of::<RGBQUAD>(), 0);
    //             self.bitmap = create_bitmap(self.width as i32, self.height as i32);
    //         }

    loop {
        // println!("{:?}", mouse_pos());
        let screen = screen_area(window.hwnd);
        println!(
            "screen: {:?}, {}, {}",
            screen,
            screen.width(),
            screen.height()
        );
        // let client = client_area(window.hwnd);
        // println!(
        //     "client: {:?}, {}, {}",
        //     client,
        //     client.width(),
        //     client.height()
        // );
        match event(window.hwnd) {
            // Some(Event::Mouse(x, y)) => println!("{} {}", x, y),
            Some(Event::Move) => println!("moved"),
            Some(Event::Quit) => break,
            _ => {}
        }
    }
}
