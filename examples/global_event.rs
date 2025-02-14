use window::*;

fn main() {
    // Event based

    // unsafe {
    // let instance = GetModuleHandleA(core::ptr::null());
    // HOOK = SetWindowsHookExA(WH_MOUSE_LL, Some(mouse_proc), instance, 0);
    // assert!(!HOOK.is_null());
    // let mut msg: MSG = core::mem::zeroed();

    // while GetMessageA(&mut msg, 0, 0, 0) > 0 {
    //     DispatchMessageA(&msg);
    // }

    // UnhookWindowsHookEx(HOOK);
    // return;
    // }

    //Polling based
    loop {
        if is_down(VK_LBUTTON) {
            eprintln!("Left button pressed");
        }
    }
}
