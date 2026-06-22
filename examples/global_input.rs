use window::*;

fn main() {
    loop {
        let gs = global_state();

        if gs.left_mouse.clicked() {
            println!("Clicked left mouse :)");
        }

        if gs.right_mouse.clicked() {
            println!("Clicked right mouse :)");
        }

        if gs.middle_mouse.clicked() {
            println!("Clicked middle mouse :)");
        }

        if gs.mouse_4.clicked() {
            println!("Clicked mouse 4 mouse :)");
        }

        if gs.mouse_5.clicked() {
            println!("Clicked mouse 5 mouse :)");
        }

        unsafe {
            watch_global_mouse_events();
        }
    }
}
