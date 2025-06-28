use windows::{
    core::*, Win32::Foundation::*, Win32::Graphics::Gdi::ValidateRect,
    Win32::System::LibraryLoader::GetModuleHandleA, Win32::UI::WindowsAndMessaging::*,
};

pub struct Window<'a> {
    title: &'a str,
    h_instance: HINSTANCE,
}

impl<'a> Window<'a> {
    pub fn new(title: &'a str) -> Self {
        let h_instance =
            get_module_handle().unwrap_or_else(|_| panic!("Failed to get module handle"));

        register_window_class(h_instance, title)
            .unwrap_or_else(|_| panic!("Failed to register window class"));

        Window { title, h_instance }
    }

    pub fn show(&self) {
        let hwnd =
            create_window(self.h_instance, self.title, "Safe Windows App").unwrap_or_else(|err| {
                println!("Error creating window: {:?} tesitng", err);
                panic!("Failed to create window")
            });

        message_loop(hwnd);
    }
}

extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        match message {
            WM_PAINT => {
                println!("WM_PAINT");
                _ = ValidateRect(Some(window), None);
                LRESULT(0)
            }
            WM_DESTROY => {
                println!("WM_DESTROY");
                PostQuitMessage(0);
                LRESULT(0)
            }
            _ => DefWindowProcA(window, message, wparam, lparam),
        }
    }
}

fn get_module_handle() -> Result<HINSTANCE> {
    unsafe { Ok(HINSTANCE(GetModuleHandleA(None)?.0)) }
}

fn register_window_class(h_instance: HINSTANCE, class_name: &str) -> Result<u16> {
    let wc = WNDCLASSA {
        lpfnWndProc: Some(wnd_proc),
        hInstance: h_instance,
        lpszClassName: PCSTR::from_raw(class_name.as_ptr()),
        ..Default::default()
    };
    let atom = unsafe { RegisterClassA(&wc) };
    if atom == 0 {
        Err(Error::from_win32())
    } else {
        Ok(atom)
    }
}

fn create_window(h_instance: HINSTANCE, class_name: &str, title: &str) -> Result<HWND> {
    unsafe {
        CreateWindowExA(
            Default::default(),
            PCSTR::from_raw(class_name.as_ptr()),
            PCSTR::from_raw(title.as_ptr()),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            800,
            600,
            None,
            None,
            Some(h_instance),
            None,
        )
    }
}

fn message_loop(_hwnd: HWND) {
    let mut msg = MSG::default();
    unsafe {
        while GetMessageA(&mut msg, None, 0, 0).into() {
            let _ = TranslateMessage(&msg);
            DispatchMessageA(&msg);
        }
    }
}

extern "system" fn wnd_proc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match msg {
        WM_DESTROY => {
            unsafe { PostQuitMessage(0) };
            LRESULT(0)
        }
        _ => unsafe { DefWindowProcA(hwnd, msg, wparam, lparam) },
    }
}
