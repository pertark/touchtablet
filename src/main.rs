// use std::sync::mpsc::{channel, Receiver, Sender};
// use std::{ptr::null_mut, sync::Arc};
use touchtablet::*;
// use winapi::{shared::ntdef::NULL, um::winuser::*};

/*
unsafe extern "system" fn block_mouse_input(n_code: i32, w_param: usize, l_param: isize) -> isize {
    if n_code < 0 {
        return CallNextHookEx(null_mut(), n_code, w_param, l_param);
    }
    return -1;
}

fn hook_loop(rx: Receiver<()>) {
    unsafe {
        let hook_handle = SetWindowsHookExA(WH_MOUSE_LL, Some(block_mouse_input), null_mut(), 0);
        rx.recv().unwrap();
        UnhookWindowsHookEx(hook_handle);
    }
}
*/

fn main() {
    println!("touchtablet started.");

    let hwnd = listener_window();
    attach(hwnd);
    message_loop(hwnd);
}
