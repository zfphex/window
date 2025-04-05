#![allow(static_mut_refs)]
use window::*;

fn main() {
    unsafe {
        loop {
            let gm = global_mouse_state();

            if gm.left_mouse.clicked() {
                println!("Clicked left mouse :)");
            }

            poll_global_events();
        }
    }
}
