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

    pub fn is_shown(&self) -> bool {
        unsafe { ffi::webui_is_shown(self.0) }
    }

    pub fn set_icon(&self, icon: &str, icon_type: &str) {
        // unsafe {ffi::webui_set_icon(self.0, icon, icon_type)}
    }

    // pub fn bind(&self, element: &str, func: fn(*mut ffi::webui_event_t)) {
    //     let cstring = CString::new(element).unwrap();

    //     // unsafe extern "C" fn wrapper

    //     unsafe {
    //         let unsafe_func: Option<unsafe extern "C" fn(*mut ffi::webui_event_t)> = Some(func);
    //         // ffi::webui_bind(self.0, cstring.as_ptr(), Some());
    //     }
    // }
}

pub fn wait() {
    unsafe {
        ffi::webui_wait();
    }
}
