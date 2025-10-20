use window::*;

fn main() {
    let window = create_window("test", 0, 0, 800, 500, WindowStyle::DEFAULT);

    // Blocking message

    // loop {
    //     if let Some(event) = wait_for_global_event() {
    //         println!("{:#?}", event);
    //     }
    // }

    //Polling message

    loop {
        let event = window.event();
        if event == Some(Event::Quit) {
            return;
        }

        //This has major issues.
        // if let Some(event) = poll_global_events() {
        //     println!("{:#?}", event);
        // }
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
