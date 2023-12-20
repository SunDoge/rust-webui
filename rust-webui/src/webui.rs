use std::ffi::CString;

use webui_sys as ffi;

pub struct Window(usize);

impl Window {
    pub fn new() -> Self {
        let window_id = unsafe { ffi::webui_new_window() };
        Self(window_id)
    }

    pub fn with_id(window_number: usize) -> Self {
        unsafe {
            ffi::webui_new_window_id(window_number);
        }
        Self(window_number)
    }

    pub fn show(&self, content: &str) -> bool {
        let cstring = CString::new(content).unwrap();
        unsafe { ffi::webui_show(self.0, cstring.as_ptr()) }
    }
}

pub fn wait() {
    unsafe {
        ffi::webui_wait();
    }
}
