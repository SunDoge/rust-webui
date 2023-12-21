use once_cell::sync::OnceCell;
use std::{
    borrow::Borrow,
    collections::HashMap,
    ffi::CString,
    path::Path,
    sync::{Mutex, RwLock},
};

use webui_sys as ffi;

static CALLBACKS: OnceCell<Mutex<HashMap<usize, Box<dyn Fn() + Send + 'static>>>> = OnceCell::new();

#[derive(Debug)]
#[repr(usize)]
pub enum Browser {
    NoBrowser = 0,
    AnyBrowser = 1,
    Chrome,
    Firefox,
    Edge,
    Safari,
    Chromium,
    Opera,
    Brave,
    Vivaldi,
    Epic,
    Yandex,
    ChromiumBased,
}

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

    pub fn show_browser(&self, content: &str, browser: Browser) -> bool {
        let cstring = CString::new(content).unwrap();
        unsafe { ffi::webui_show_browser(self.0, cstring.as_ptr(), browser as usize) }
    }

    pub fn is_shown(&self) -> bool {
        unsafe { ffi::webui_is_shown(self.0) }
    }

    pub fn set_size(&mut self, width: u32, height: u32) {
        unsafe {
            ffi::webui_set_size(self.0, width, height);
        }
    }

    pub fn set_position(&mut self, x: u32, y: u32) {
        unsafe {
            ffi::webui_set_position(self.0, x, y);
        }
    }

    pub fn set_root_folder(&mut self, path: impl AsRef<Path>) -> bool {
        let cstring = CString::new(path.as_ref().as_os_str().to_str().unwrap()).unwrap();
        unsafe { ffi::webui_set_root_folder(self.0, cstring.as_ptr()) }
    }

    pub fn set_icon(&self, icon: &str, icon_type: &str) {
        // unsafe {ffi::webui_set_icon(self.0, icon, icon_type)}
    }

    pub fn bind(&self, element: &str) {
        let cstring = CString::new(element).unwrap();

        // unsafe extern "C" fn wrapper

        unsafe {
            // let unsafe_func: Option<unsafe extern "C" fn(*mut ffi::webui_event_t)> = Some(func);
            ffi::webui_interface_bind(self.0, cstring.as_ptr(), Some(event_handler));
        }
    }
}

unsafe extern "C" fn event_handler(
    window: usize,
    event_type: usize,
    element_ptr: *mut std::os::raw::c_char,
    data: usize,
    bind_id: usize,
) {
    CALLBACKS.get().map(|cbs| {
        cbs.lock().unwrap().get(&bind_id).map(|cb| {
            cb();
        });
    });
}

pub fn wait() {
    unsafe {
        ffi::webui_wait();
    }
}

pub fn clean() {
    unsafe {
        ffi::webui_clean();
    }
}

pub fn set_timeout(second: usize) {
    unsafe {
        ffi::webui_set_timeout(second);
    }
}
