use std::ptr::{self, null_mut};

use winapi::ctypes::{c_int, c_void};
use winapi::shared::hidpi::{
    HidP_GetCaps, HidP_GetUsageValue, HidP_GetValueCaps, HidP_Input,
    HIDP_CAPS, HIDP_PREPARSED_DATA, HIDP_STATUS_INVALID_PREPARSED_DATA, HIDP_STATUS_SUCCESS,
    HIDP_VALUE_CAPS, 
};
use winapi::shared::minwindef::{DWORD, LPARAM, LPVOID, LRESULT, WPARAM};
use winapi::shared::ntdef::{PCHAR, ULONG};
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::{shared::windef::*, um::winuser::*};

#[macro_export]
macro_rules! native_str {
    ($str: expr) => {
        format!("{}\0", $str).as_ptr() as *const u16
    };
}

pub fn listener_window() -> HWND {
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
            ptr::null_mut(),
        );

        assert!(hwnd != ptr::null_mut());

        return hwnd;
    }
}

pub fn get_devices() {
    unsafe {
        let mut buffer: [RAWINPUTDEVICELIST; 1000] = [RAWINPUTDEVICELIST {
            hDevice: std::mem::MaybeUninit::uninit().assume_init(),
            dwType: RIM_TYPEKEYBOARD,
        }; 1000];

        let mut num_devices: u32 = 0;
        let device_list_size = std::mem::size_of::<RAWINPUTDEVICELIST>();

        // Need to call this twice - the first time to fill out num_devices so we can
        // send it back in with the second call.
        // GetRawInputDeviceList(ptr::null_mut(), &mut num_devices, device_list_size as u32);
        GetRawInputDeviceList(
            buffer.as_mut_ptr() as *mut RAWINPUTDEVICELIST,
            &mut num_devices,
            device_list_size as u32,
        );

        println!("{:?}", num_devices);
        GetRawInputDeviceList(
            buffer.as_mut_ptr() as *mut RAWINPUTDEVICELIST,
            &mut num_devices,
            device_list_size as u32,
        );

        // let mut hid_devices = vec![];
        for pos in 0..num_devices as usize {
            let device_ptr = (&mut buffer[pos..pos + 1]).as_mut_ptr() as *const RAWINPUTDEVICELIST;
            let device = *device_ptr;
            println!("{}", get_device_name(device));

            if device.dwType == RIM_TYPEHID {
                let mut size: u32 = 0;
                if GetRawInputDeviceInfoW(device.hDevice, RIDI_PREPARSEDDATA, null_mut(), &mut size)
                    != 0
                {
                    panic!("Failed to get size of preparsed data.");
                };

                println!("Size of buf: {:?}", size);

                let mut preparsed_data = vec![0; size as usize];
                if GetRawInputDeviceInfoW(
                    device.hDevice,
                    RIDI_PREPARSEDDATA,
                    preparsed_data.as_mut_ptr() as *mut c_void,
                    &mut size,
                ) == u32::MAX
                {
                    println!("{:?}", std::io::Error::last_os_error());
                    panic!(
                        "Failed to get preparsed data. {:?} {:?}",
                        size, preparsed_data
                    );
                };

                preparsed_data.truncate(size as usize);
                println!("{:?} {:?}", preparsed_data, size);

                let mut capabilities: HIDP_CAPS = std::mem::MaybeUninit::uninit().assume_init();
                let hid_ret = HidP_GetCaps(
                    preparsed_data.as_mut_ptr() as *mut HIDP_PREPARSED_DATA,
                    &mut capabilities,
                );
                println!(
                    "{:?}, {:?}, {:?}",
                    HIDP_STATUS_SUCCESS, HIDP_STATUS_INVALID_PREPARSED_DATA, hid_ret
                );
                if hid_ret != HIDP_STATUS_SUCCESS {
                    panic!("Failed to get HIDP capabilities.");
                }
                println!(
                    "NumberInputValueCaps: {:?}, NumberInputButtomCaps: {:?}",
                    capabilities.NumberInputButtonCaps, capabilities.NumberFeatureButtonCaps
                );
            }
        }
    }
}

fn get_device_name(device: RAWINPUTDEVICELIST) -> String {
    use std::ffi::OsString;
    use std::os::windows::ffi::OsStringExt;

    unsafe {
        let mut name: [u16; 1024] = std::mem::MaybeUninit::uninit().assume_init();
        // let mut rim_type: RID_DEVICE_INFO = std::mem::MaybeUninit::uninit().assume_init();
        let mut name_size: u32 = 1024;

        let bytes = GetRawInputDeviceInfoW(
            device.hDevice,
            RIDI_DEVICENAME,
            name.as_mut_ptr() as LPVOID,
            &mut name_size,
        );

        let name_slice = &name[0..bytes as usize];
        let full_name = match OsString::from_wide(name_slice).into_string() {
            Ok(something) => something,
            Err(_) => panic!("String Conversion Failed"),
        };

        String::from(full_name)
    }
}

pub unsafe fn garbage_vec<T>(size: usize) -> Vec<T> {
    let mut v = Vec::with_capacity(size);
    v.set_len(size);
    v
}

fn handle_touchpad(raw_input: &RAWINPUT) {
    unsafe {
        // let data = raw_input.data.mouse();
        let data_hid = raw_input.data.hid();

        let mut size: u32 = 0;
        if GetRawInputDeviceInfoW(
            raw_input.header.hDevice,
            RIDI_PREPARSEDDATA,
            null_mut(),
            &mut size,
        ) != 0
        {
            panic!("Failed to get size of preparsed data.");
        };

        let mut preparsed_data = vec![0; size as usize];
        if GetRawInputDeviceInfoW(
            raw_input.header.hDevice,
            RIDI_PREPARSEDDATA,
            preparsed_data.as_mut_ptr() as *mut c_void,
            &mut size,
        ) == u32::MAX
        {
            println!("{:?}", std::io::Error::last_os_error());
            panic!(
                "Failed to get preparsed data. {:?} {:?}",
                size, preparsed_data
            );
        };

        preparsed_data.truncate(size as usize);

        let mut capabilities: HIDP_CAPS = std::mem::MaybeUninit::uninit().assume_init();
        let hid_ret = HidP_GetCaps(
            preparsed_data.as_mut_ptr() as *mut HIDP_PREPARSED_DATA,
            &mut capabilities,
        );
        // println!("{:?}, {:?}, {:?}", HIDP_STATUS_SUCCESS, HIDP_STATUS_INVALID_PREPARSED_DATA, hid_ret);
        if hid_ret != HIDP_STATUS_SUCCESS {
            panic!("Failed to get HIDP capabilities.");
        }

        let n_value_caps: *mut u16 = &mut capabilities.NumberInputValueCaps;
        let mut value_caps: Vec<HIDP_VALUE_CAPS> =
            garbage_vec(capabilities.NumberInputValueCaps as usize);

        let _ret = HidP_GetValueCaps(
            HidP_Input,
            value_caps.as_mut_ptr() as *mut HIDP_VALUE_CAPS,
            n_value_caps,
            preparsed_data.as_mut_ptr() as *mut HIDP_PREPARSED_DATA,
        );

        for _val_cap in value_caps {
            let mut x_val: ULONG = std::mem::MaybeUninit::uninit().assume_init();

            let _hidp_ret = HidP_GetUsageValue(
                HidP_Input,
                0x01,
                0,
                0x30,
                &mut x_val,
                preparsed_data.as_mut_ptr() as *mut HIDP_PREPARSED_DATA,
                std::mem::transmute::<_, PCHAR>(data_hid.bRawData.as_ptr()),
                data_hid.dwSizeHid,
            );

            let mut y_val: ULONG = std::mem::MaybeUninit::uninit().assume_init();

            let _hidp_ret = HidP_GetUsageValue(
                HidP_Input,
                0x01,
                0,
                0x31,
                &mut y_val,
                preparsed_data.as_mut_ptr() as *mut HIDP_PREPARSED_DATA,
                std::mem::transmute::<_, PCHAR>(data_hid.bRawData.as_ptr()),
                data_hid.dwSizeHid,
            );

            println!(
                "X: {:?}, Y: {:?}",
                x_val, y_val, 
            );
            move_mouse(
                i32::try_from(x_val * 1920 / 1220).unwrap(),
                i32::try_from(y_val * 1920 / 1220).unwrap(),
            );

            break;
        }
    }
}

fn move_mouse(x: i32, y: i32) {
    let mut input = INPUT {
        type_: INPUT_MOUSE,
        u: unsafe {
            std::mem::transmute(MOUSEINPUT {
                dx: (x - GetSystemMetrics(SM_XVIRTUALSCREEN)) * 65535
                    / GetSystemMetrics(SM_CXVIRTUALSCREEN),
                dy: (y - GetSystemMetrics(SM_YVIRTUALSCREEN)) * 65535
                    / GetSystemMetrics(SM_CYVIRTUALSCREEN),
                mouseData: 0,
                dwFlags: MOUSEEVENTF_MOVE | MOUSEEVENTF_ABSOLUTE | MOUSEEVENTF_VIRTUALDESK,
                time: 0,
                dwExtraInfo: 0,
            })
        },
    };
    unsafe {
        SendInput(
            1,
            &mut input as LPINPUT,
            std::mem::size_of::<INPUT>() as c_int,
        )
    };
}

unsafe extern "system" fn wnd_proc(
    hwnd: HWND,
    msg: u32,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    match msg {
        WM_INPUT => {
            let mut dwsize = 0; // std::mem::MaybeUninit::uninit().assume_init();
            let mut raw_input: RAWINPUT = std::mem::MaybeUninit::uninit().assume_init();

            let ret = GetRawInputData(
                l_param as HRAWINPUT,
                RID_INPUT,
                null_mut(),
                &mut dwsize,
                std::mem::size_of::<RAWINPUTHEADER>() as u32,
            );

            if ret == u32::MAX {
                panic!("GetRawInputData failed.");
            }

            let ret2 = GetRawInputData(
                l_param as *mut _,
                RID_INPUT,
                &mut raw_input as *mut _ as *mut c_void,
                &mut dwsize,
                std::mem::size_of::<RAWINPUTHEADER>() as u32,
            );
            if ret2 == u32::MAX {
                panic!("Second GetRawInputData failed. Lmao ");
            };

            match raw_input.header.dwType {
                RIM_TYPEHID => handle_touchpad(&raw_input),
                RIM_TYPEKEYBOARD => println!("KEYBOARD"),
                _ => {
                    println!(
                        "This shouldn't happen. Header type: {:?}",
                        raw_input.header.dwType
                    );
                    handle_touchpad(&raw_input);
                }
            }
            0
        }
        _ => DefWindowProcW(hwnd, msg, w_param, l_param),
    }
}

pub fn attach(hwnd: HWND) {
    let touchpad = RAWINPUTDEVICE {
        usUsagePage: 0x000D,
        usUsage: 0x0005, // Precision Touchpad
        dwFlags: RIDEV_INPUTSINK,
        hwndTarget: hwnd,
    };

    // let keyboard = RAWINPUTDEVICE {
    //     usUsagePage: 1,
    //     usUsage: 6,	// Keyboards
    //     dwFlags: RIDEV_INPUTSINK,
    //     hwndTarget: hwnd,
    // };

    unsafe {
        RegisterRawInputDevices(
            vec![touchpad].as_ptr(),
            1,
            std::mem::size_of::<RAWINPUTDEVICE>() as u32,
        );
    }
}

pub fn message_loop(hwnd: HWND) {
    let mut msg = MSG {
        hwnd: hwnd,
        message: 0 as u32,
        wParam: 0 as WPARAM,
        lParam: 0 as LPARAM,
        time: 0 as DWORD,
        pt: POINT { x: 0, y: 0 },
    };
    unsafe {
        while GetMessageW(&mut msg, hwnd as HWND, WM_INPUT, WM_INPUT) == 1 {
            DispatchMessageW(&msg);
        }
        CloseWindow(hwnd);
    }
}
