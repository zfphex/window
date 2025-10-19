use core::ptr::null_mut;
use window::*;

fn main() {
    unsafe {
        let mut window = create_window("Tray Example", 100, 100, 400, 300, WindowStyle::DEFAULT);
        let h_icon = LoadIconA(null_mut(), IDI_APPLICATION as *const i8);
        create_tray_icon(window.hwnd, 1, h_icon, "Tray Example");

        loop {
            if let Some(event) = window.event() {
                match event {
                    _ => {}
                }
            }

            if window.tray.is_pressed() {
                break;
            }
        }
    }
}
