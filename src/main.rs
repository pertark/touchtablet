use winapi::ctypes::c_void;
use winapi::shared::minwindef::LPARAM;
use winit::event::{DeviceEvent,Event,WindowEvent};
use winit::event_loop::EventLoop;
use winit::event_loop::ControlFlow;
use winit::event::Touch;
use winit::window::WindowBuilder;

use winapi::um::winuser::*;

use touchtablet::*;

fn main() {
    println!("touchtablet started.");
    get_devices();


    let hwnd = listener_window();
    attach(hwnd);
    message_loop(hwnd);   

    // unsafe {
    //     let mut raw_input: RAWINPUT = std::mem::MaybeUninit::uninit().assume_init();
    //     let l_param: LPARAM = std::mem::MaybeUninit::uninit().assume_init();

    //     GetRawInputData(&mut l_param as *mut _ as HRAWINPUT,
    //         RID_INPUT, 
    //         &mut raw_input as *mut _ as *mut winapi::ctypes::c_void, 
    //         pcbSize, cbSizeHeader);
    // }


    // unsafe {
    //     let confuse: *mut c_void;
    //     let mut raw_dev_list = RAWINPUTDEVICELIST { 
    //         hDevice: confuse,
    //         dwType: RIM_TYPEHID
    //     }; 
        
    //     let mut raw_raw_dev_list = &mut raw_dev_list as *mut RAWINPUTDEVICELIST;

    //     let mut num = 3u32;
    //     let puiNumDevices = &mut num as *mut u32;

    //     GetRawInputDeviceList(raw_raw_dev_list, puiNumDevices, 48);
    //     // winuser::GetRawInputDeviceInfoA(winuser::TOUCHINPUT, uiCommand, pData, pcbSize);
    //     // winuser::GetRawInputDeviceInfoW(hDevice, uiCommand, pData, pcbSize);
    // }
    // winuser::PT_TOUCHPAD
    // let el = EventLoop::new();
    // let window = WindowBuilder::new().build(&el).unwrap();

    // el.run(|event, _, control_flow| {
    //     // *control_flow = ControlFlow::Wait;

    //     println!("{:?}\n{:?}", event, control_flow);

    //     match event {
    //         Event::DeviceEvent { device_id, event: device_event } => {
    //             println!("{:?} {:?}", device_event, device_id)
    //         },
    //         Event::WindowEvent { window_id, event: window_event } => {
    //             println!("{:?} {:?}", window_event, window_id);
    //         }
    //         _ => ()
    //     }
    // })
}
