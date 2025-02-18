use window::*;

fn main() {
    let window = create_window("test", 800, 500, WindowStyle::DEFAULT);

    // Blocking message

    // loop {
    //     if let Some(event) = wait_for_global_event() {
    //         println!("{:#?}", event);
    //     }
    // }

    //Polling message

    loop {
        let _ = window.event();
        if let Some(event) = poll_global_events() {
            println!("{:#?}", event);
        }
    }

    //TODO: Is this needed?
    // unsafe { UnhookWindowsHookEx(HOOK) };

    //Polling
    // loop {
    //     if is_down(VK_LBUTTON) {
    //         eprintln!("Left button pressed");
    //     }
    // }
}
