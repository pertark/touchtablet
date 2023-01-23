use std::ffi::c_void;
use std::ptr;

use winapi::shared::minwindef::{LPVOID, WPARAM, LPARAM, LRESULT, DWORD};
use winapi::{
    shared::windef::*,
    um::winuser::*,
};
use winapi::um::libloaderapi::GetModuleHandleW;


#[macro_export]
/// Convert regular expression to a native string, to be passable as an argument in WinAPI
macro_rules! native_str {
    ($str: expr) => {
        format!("{}\0", $str).as_ptr() as *const u16
    };
}

pub fn listener_window () -> HWND {
    unsafe {
        let h_instance = GetModuleHandleW(ptr::null_mut());
        let class_name = native_str!("touchtablet::listener");
        let win = WNDCLASSW {
            hInstance: h_instance,
            lpfnWndProc: Some(wnd_proc),
            lpszClassName: class_name,
            style: 0,
            cbClsExtra: 0,
            cbWndExtra: 0,
            hbrBackground: ptr::null_mut(),
            hCursor: ptr::null_mut(),
            hIcon: ptr::null_mut(),
            lpszMenuName: ptr::null_mut(),
        };

        assert!(RegisterClassW(&win) != 0);

        let hwnd = CreateWindowExW(
            WS_EX_CLIENTEDGE,
            class_name,
            class_name,
            WS_OVERLAPPEDWINDOW,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            ptr::null_mut(),
            ptr::null_mut(),
            h_instance,
            ptr::null_mut());

        assert!(hwnd != ptr::null_mut());

        return hwnd;
    }
}


#[allow(unused)]
pub fn get_devices() {
    unsafe {
        // let mut buffer: [RAWINPUTDEVICELIST; 1000] = std::mem::MaybeUninit::uninit().assume_init();
        let mut buffer: [RAWINPUTDEVICELIST; 1000] = [RAWINPUTDEVICELIST { 
            hDevice: std::mem::MaybeUninit::uninit().assume_init(),
            dwType: RIM_TYPEKEYBOARD
        }; 1000 ];

        // let mut buffer = raw_dev_list;

        // let mut buffer = &mut raw_dev_list as *mut RAWINPUTDEVICELIST;
        let mut num_devices: u32 = 0;
        let device_list_size = std::mem::size_of::<RAWINPUTDEVICELIST>();

        // Need to call this twice - the first time to fill out num_devices so we can
        // send it back in with the second call.
        // GetRawInputDeviceList(ptr::null_mut(), &mut num_devices, device_list_size as u32);
        GetRawInputDeviceList(buffer.as_mut_ptr() as *mut RAWINPUTDEVICELIST, &mut num_devices, device_list_size as u32);
        
        println!("{:?}", num_devices);
        GetRawInputDeviceList(buffer.as_mut_ptr() as *mut RAWINPUTDEVICELIST, &mut num_devices, device_list_size as u32);

        for pos in 0..num_devices as usize {
            let device_ptr = (&mut buffer[pos..pos+1]).as_mut_ptr() as *const RAWINPUTDEVICELIST;
            let device = *device_ptr;
            println!("{}", get_device_name(device));
        }
    }
}

fn get_device_name(device: RAWINPUTDEVICELIST) -> String {
    use std::ffi::OsString;
    use std::os::windows::ffi::OsStringExt;

    unsafe {
        let mut name: [u16; 1024] = std::mem::MaybeUninit::uninit().assume_init();
        let mut rim_type: RID_DEVICE_INFO = std::mem::MaybeUninit::uninit().assume_init();
        let mut name_size: u32 = 1024;

        let bytes = GetRawInputDeviceInfoW(device.hDevice, RIDI_DEVICENAME, name.as_mut_ptr() as LPVOID, &mut name_size);

        let size = std::mem::MaybeUninit::uninit().assume_init();
        let type_bytes = GetRawInputDeviceInfoW(device.hDevice, RIDI_DEVICEINFO, &mut rim_type as *mut _ as LPVOID, size);
        // println!("type bytes {} {:?}", type_bytes, size);
        let name_slice = &name[0..bytes as usize];
        let full_name = match OsString::from_wide(name_slice).into_string(){
            Ok(something) => something,
            Err(_) => panic!("String Conversion Failed"),
        };

        let prefix = match rim_type.dwType {
            RIM_TYPEMOUSE => "mouse_",
            RIM_TYPEKEYBOARD => "keyboard_",
            RIM_TYPEHID => "hid_",
            _ => "????_"
        };

        format!("{}{}", String::from(prefix), String::from(full_name))
    }    
}


fn handle_touchpad(raw_input: &RAWINPUT) {
    unsafe {
        let data = raw_input.data.mouse();
        let data_hid = raw_input.data.hid();
        
        let bytes: [i8; 4] = std::mem::transmute(data.ulRawButtons.to_be());
        // println!("Data: {:?} {:?}", data.ulRawButtons, data.usButtonData);
        // println!("Data HID: {:?}", data_hid.bRawData);
        // println!("Data: {:?}", bytes);
        println!("binary: {:32b}", data.ulRawButtons)
        // println!("Data: {:?}", data);
    }
}

unsafe extern "system" fn wnd_proc(
    hwnd: HWND,
    msg: u32,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    match msg {
        WM_INPUT => {
            let mut dwsize: u32 = std::mem::size_of::<RAWINPUT>() as u32;
            // let mut dwsize = std::mem::MaybeUninit::uninit().assume_init();
            // let dwsize = ptr::null_mut();
            let mut raw_input: RAWINPUT = std::mem::MaybeUninit::uninit().assume_init();
            
            let ret = GetRawInputData(
                l_param as *mut _,
                RID_INPUT,
                &mut raw_input as *mut _ as *mut winapi::ctypes::c_void,
                &mut dwsize,
               std::mem::size_of::<RAWINPUTHEADER>() as u32
            );
            
            println!("Return value: {:b}", ret);
            println!("pcbSize: {:?}", dwsize);
            // println!("RID_HEADER: {:?}", raw_input.header.dwType);
            // println!("RID_HEADER Data: {:?}", raw_input.data.hid().bRawData);

            // GetRawInputData(
            //     l_param as *mut _,
            //     RID_INPUT,
            //     &mut raw_input as *mut _ as *mut winapi::ctypes::c_void,
            //     &mut dwsize,
            //    std::mem::size_of::<RAWINPUTHEADER>() as u32
            // );

            println!("Device: {:?}", raw_input.header.hDevice);

            // GetRawInputDeviceInfoA(
            //     raw_input.header.hDevice, 
            //     RIDI_PREPARSEDDATA, 
            //     std::mem::MaybeUninit::uninit().assume_init(), 
            //     &mut dwsize
            // );

            match raw_input.header.dwType {
                RIM_TYPEHID => handle_touchpad(&raw_input),
                RIM_TYPEKEYBOARD => println!("KEYBOARD"),
                _ => { 
                    println!("This shouldn't happen. Header type: {:?}", raw_input.header.dwType);
                    handle_touchpad(&raw_input);
                }
            }
            0
        },
        _ => DefWindowProcW(hwnd, msg, w_param, l_param),
    }
}

pub fn attach(hwnd: HWND) {
    
    let touchpad = RAWINPUTDEVICE {
	    usUsagePage: 0x000D,
	    usUsage: 0x0005,	// Precision Touchpad
	    dwFlags: RIDEV_INPUTSINK,
	    hwndTarget: hwnd,
    };
    
    let keyboard = RAWINPUTDEVICE {
	    usUsagePage: 1,
	    usUsage: 6,	// Keyboards
	    dwFlags: RIDEV_INPUTSINK,
	    hwndTarget: hwnd,
    };

    unsafe { 
        RegisterRawInputDevices(vec![touchpad, keyboard].as_ptr(), 2, std::mem::size_of::<RAWINPUTDEVICE>() as u32);
    }
}

pub fn message_loop(hwnd: HWND) {
    let mut msg = MSG {
        hwnd : hwnd,
        message : 0 as u32,
        wParam : 0 as WPARAM,
        lParam : 0 as LPARAM,
        time : 0 as DWORD,
        pt : POINT { x: 0, y: 0, },
    };
    unsafe {
        while GetMessageW(&mut msg, hwnd as HWND, WM_INPUT, WM_INPUT) == 1 {
            DispatchMessageW(&msg);
        }
        CloseWindow(hwnd);
    }
}
