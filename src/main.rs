use window::*;

//https://rust-tutorials.github.io/triangle-from-scratch/opening_a_window/win32.html
fn main() {
    let _window = create_window("test window", 600, 400);

    loop {
        // println!("{:?}", mouse_pos());
        match event() {
            Some(Event::Mouse(x, y)) => println!("{} {}", x, y),
            Some(Event::Quit) => break,
            _ => {}
        }
    }
}
