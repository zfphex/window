use window::*;

fn main() {
    // Blocking message

    // loop {
    //     if let Some(event) = wait_for_global_event() {
    //         println!("{:#?}", event);
    //     }
    // }

    //Polling message

    loop {
        if let Some(event) = poll_global_event() {
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
