use touchtablet::*;
use winapi::{um::winuser::*, shared::ntdef::NULL};
use std::{ptr::{null, null_mut}, sync::Arc};
use std::sync::mpsc::{channel, Receiver, Sender};

unsafe extern "system" fn block_mouse_input(n_code: i32, w_param: usize, l_param: isize) -> isize {
    if n_code < 0 {
        return CallNextHookEx(null_mut(), n_code, w_param, l_param)
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

fn main() {
    println!("touchtablet started.");    
 
    // let (tx, rx) = channel::<()>();
    // hook_loop(rx);

    // ctrlc::set_handler(move || {
    //     tx.send(()).unwrap();
    //     println!("CTRL-C");
    //     std::process::exit(-1);
    // }).expect("Error setting ctrl-c handler");
    // get_devices();

    let hwnd = listener_window();
    attach(hwnd);
    message_loop(hwnd);   

}
