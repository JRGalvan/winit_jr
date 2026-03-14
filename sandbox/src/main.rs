use windows_sys::Win32::Foundation;
use windows_sys::Win32::Graphics::Gdi;
use windows_sys::Win32::UI::WindowsAndMessaging;
use windows_sys::core::w;

unsafe extern "system" fn wnd_proc(
    hwnd: Foundation::HWND,
    msg: u32,
    wparam: Foundation::WPARAM,
    lparam: Foundation::LPARAM,
) -> Foundation::LRESULT {
    match msg {
        WindowsAndMessaging::WM_DESTROY => {
            // https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-postquitmessage
            unsafe { WindowsAndMessaging::PostQuitMessage(0) };

            return 0;
        }
        _ => unsafe { WindowsAndMessaging::DefWindowProcW(hwnd, msg, wparam, lparam) },
    }
}

fn main() {
    let class_name = w!("test");
    let window_name = w!("test");

    // https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadcursorw
    let hcusor = unsafe {
        WindowsAndMessaging::LoadCursorW(core::ptr::null_mut(), WindowsAndMessaging::IDC_ARROW)
    };

    // https://learn.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-wndclassexw
    let window_class = WindowsAndMessaging::WNDCLASSEXW {
        cbSize: std::mem::size_of::<WindowsAndMessaging::WNDCLASSEXW>() as u32,
        style: WindowsAndMessaging::CS_HREDRAW | WindowsAndMessaging::CS_VREDRAW,
        lpfnWndProc: Some(wnd_proc),
        cbClsExtra: 0,
        cbWndExtra: 0,
        hInstance: core::ptr::null_mut(),
        hIcon: core::ptr::null_mut(),
        hCursor: hcusor,
        hbrBackground: core::ptr::null_mut(),
        lpszMenuName: core::ptr::null(),
        lpszClassName: class_name,
        hIconSm: core::ptr::null_mut(),
    };

    // https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassexa
    let window_class_handle = unsafe { WindowsAndMessaging::RegisterClassExW(&window_class) };

    match window_class_handle {
        // https://learn.microsoft.com/en-us/windows/win32/debug/system-error-code-lookup-tool
        0 => {
            println!(
                "There was a problem registering the window class.\nWindows error code: {:#x}",
                unsafe { Foundation::GetLastError() }
            )
        }
        _ => println!(
            "The window class was registered successfully.\nWindow class handle: {:?}",
            window_class_handle
        ),
    }

    // https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw
    let window_handle = unsafe {
        WindowsAndMessaging::CreateWindowExW(
            WindowsAndMessaging::WS_EX_LEFT,
            class_name,
            window_name,
            WindowsAndMessaging::WS_OVERLAPPEDWINDOW,
            WindowsAndMessaging::CW_USEDEFAULT,
            WindowsAndMessaging::CW_USEDEFAULT,
            WindowsAndMessaging::CW_USEDEFAULT,
            WindowsAndMessaging::CW_USEDEFAULT,
            core::ptr::null_mut(),
            core::ptr::null_mut(),
            core::ptr::null_mut(),
            core::ptr::null_mut(),
        )
    };

    match window_handle.is_null() {
        // https://learn.microsoft.com/en-us/windows/win32/debug/system-error-code-lookup-tool
        true => {
            println!(
                "There was a problem creating the window.\nWindows error code: {:#x}",
                unsafe { Foundation::GetLastError() }
            )
        }
        false => println!(
            "The window was created successfully.\nWindow handle: {:?}",
            window_handle
        ),
    }

    // https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow
    unsafe { WindowsAndMessaging::ShowWindow(window_handle, WindowsAndMessaging::SW_SHOW) };

    // https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-updatewindow
    unsafe { Gdi::UpdateWindow(window_handle) };

    let mut message: WindowsAndMessaging::MSG = unsafe { std::mem::zeroed() };

    loop {
        // https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmessage
        if unsafe { WindowsAndMessaging::GetMessageW(&mut message, core::ptr::null_mut(), 0, 0) }
            == 0
        {
            break;
        }

        // https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-translatemessage
        unsafe { WindowsAndMessaging::TranslateMessage(&mut message) };

        // https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-dispatchmessage
        unsafe { WindowsAndMessaging::DispatchMessageW(&mut message) };
    }
}
