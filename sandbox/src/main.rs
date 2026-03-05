use windows_sys::Win32::Foundation;
use windows_sys::Win32::UI::WindowsAndMessaging;
use windows_sys::core::w;

fn main() {
    // https://learn.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-wndclassexw
    let window_class = WindowsAndMessaging::WNDCLASSEXW {
        cbSize: std::mem::size_of::<WindowsAndMessaging::WNDCLASSEXW>() as u32,
        style: WindowsAndMessaging::CS_HREDRAW | WindowsAndMessaging::CS_VREDRAW,
        lpfnWndProc: None,
        cbClsExtra: 0,
        cbWndExtra: 0,
        hInstance: core::ptr::null_mut(),
        hIcon: core::ptr::null_mut(),
        hCursor: core::ptr::null_mut(),
        hbrBackground: core::ptr::null_mut(),
        lpszMenuName: core::ptr::null(),
        lpszClassName: w!("test"),
        hIconSm: core::ptr::null_mut(),
    };

    let class_handle = unsafe { WindowsAndMessaging::RegisterClassExW(&window_class) };

    match class_handle {
        0 => {
            println!(
                "There was a problem registering the window class.\nWindows error code: {:#x}",
                unsafe { Foundation::GetLastError() }
            )
        }
        _ => (),
    }
}
