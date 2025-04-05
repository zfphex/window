#![allow(static_mut_refs)]
use window::*;

fn main() {
    loop {
        let gm = global_state();

        if gm.left_mouse.clicked() {
            println!("Clicked left mouse :)");
        }

        if gm.right_mouse.clicked() {
            println!("Clicked right mouse :)");
        }

        if gm.middle_mouse.clicked() {
            println!("Clicked middle mouse :)");
        }

        if gm.mouse_4.clicked() {
            println!("Clicked mouse 4 mouse :)");
        }

        if gm.mouse_5.clicked() {
            println!("Clicked mouse 5 mouse :)");
        }

        poll_global_events();
    }
}
