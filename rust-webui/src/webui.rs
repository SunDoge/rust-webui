use once_cell::sync::Lazy;
use std::{
    collections::HashMap,
    ffi::{CStr, CString},
    fmt::Debug,
    path::Path,
    sync::RwLock,
};

use webui_sys as ffi;

type CallbackMap = HashMap<usize, Box<dyn Fn(&mut Event) + Send + Sync>>;

static EVENT_HANDLERS: Lazy<RwLock<CallbackMap>> = Lazy::new(|| RwLock::new(HashMap::new()));

// type FileHandlerMap = HashMap<usize, Box<dyn Fn(&str) + Send + Sync>>;
// static FILE_HANDLERS: Lazy<RwLock<FileHandlerMap>> = Lazy::new(|| RwLock::new(HashMap::new()));

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

#[derive(Debug)]
#[repr(usize)]
pub enum Runtime {
    None = 0,
    Deno,
    NodeJs,
}

pub struct Window {
    handle: usize,
    // file_handler: Option<Box<dyn Fn(&str)>>,
}

impl Debug for Window {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Window")
            .field("handle", &self.handle)
            // .field("file_handler", &self.file_handler.is_some())
            .finish()
    }
}

impl Default for Window {
    fn default() -> Self {
        Self::new()
    }
}

impl Window {
    pub fn new() -> Self {
        let window_number = unsafe { ffi::webui_new_window() };
        Self {
            handle: window_number,
        }
    }

    pub fn handle(&self) -> usize {
        self.handle
    }

    pub fn get_unique_window_id(&self) -> usize {
        unsafe { ffi::webui_interface_get_window_id(self.handle()) }
    }

    pub fn with_number(window_number: usize) -> Self {
        unsafe {
            ffi::webui_new_window_id(window_number);
        }
        Self {
            handle: window_number,
            // file_handler: None,
        }
    }

    pub fn show(&self, content: &str) -> bool {
        let cstring = CString::new(content).unwrap();
        unsafe { ffi::webui_show(self.handle(), cstring.as_ptr()) }
    }

    pub fn show_browser(&self, content: &str, browser: Browser) -> bool {
        let cstring = CString::new(content).unwrap();
        unsafe { ffi::webui_show_browser(self.handle(), cstring.as_ptr(), browser as usize) }
    }

    pub fn is_shown(&self) -> bool {
        unsafe { ffi::webui_is_shown(self.handle()) }
    }

    pub fn set_size(&mut self, width: u32, height: u32) {
        unsafe {
            ffi::webui_set_size(self.handle(), width, height);
        }
    }

    pub fn set_position(&mut self, x: u32, y: u32) {
        unsafe {
            ffi::webui_set_position(self.handle(), x, y);
        }
    }

    pub fn set_root_folder(&mut self, path: impl AsRef<Path>) -> bool {
        let cstring = CString::new(path.as_ref().as_os_str().to_str().unwrap()).unwrap();
        unsafe { ffi::webui_set_root_folder(self.handle(), cstring.as_ptr()) }
    }

    pub fn set_icon(&mut self, icon: &str, icon_type: &str) {
        let icon_c = CString::new(icon).unwrap();
        let icon_type_c = CString::new(icon_type).unwrap();
        unsafe { ffi::webui_set_icon(self.handle(), icon_c.as_ptr(), icon_type_c.as_ptr()) }
    }

    pub fn set_port(&mut self, port: usize) -> bool {
        unsafe { ffi::webui_set_port(self.handle(), port) }
    }

    pub fn set_runtime(&mut self, runtime: Runtime) {
        unsafe {
            ffi::webui_set_runtime(self.handle(), runtime as usize);
        }
    }

    // pub fn set_file_handler()

    pub fn bind(&self, element: &str, func: impl Fn(&mut Event) + Send + Sync + 'static) {
        let cstring = CString::new(element).unwrap();

        let bind_id = unsafe {
            ffi::webui_interface_bind(self.handle(), cstring.as_ptr(), Some(event_handler))
        };

        {
            let mut cbs = EVENT_HANDLERS.write().unwrap();
            cbs.insert(bind_id, Box::new(func));
        }
    }

    pub fn send_raw(&self, func: &str, buf: &[u8]) {
        let func_cstring = CString::new(func).unwrap();

        unsafe {
            ffi::webui_send_raw(
                self.handle(),
                func_cstring.as_ptr(),
                buf.as_ptr() as *const _,
                buf.len(),
            )
        }
    }
}

#[derive(Debug)]
pub enum EventType {
    Disconnected = 0,
    Connected,
    MouseClick,
    Navigation,
    Callback,
}

impl TryFrom<usize> for EventType {
    type Error = ();
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(EventType::Disconnected),
            1 => Ok(EventType::Connected),
            2 => Ok(EventType::MouseClick),
            3 => Ok(EventType::Navigation),
            4 => Ok(EventType::Callback),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub struct Event<'a> {
    pub window: Window,
    pub event_type: EventType,
    pub element: &'a str,
    pub event_number: usize,
    pub bind_id: usize,
}

impl<'a> Event<'a> {
    pub fn get_int_at(&self, index: usize) -> i64 {
        unsafe { ffi::webui_interface_get_int_at(self.window.handle(), self.event_number, index) }
    }

    pub fn get_bool_at(&self, index: usize) -> bool {
        unsafe { ffi::webui_interface_get_bool_at(self.window.handle(), self.event_number, index) }
    }

    pub fn get_string_at(&self, index: usize) -> &str {
        unsafe {
            let ptr =
                ffi::webui_interface_get_string_at(self.window.handle(), self.event_number, index);
            let length =
                ffi::webui_interface_get_size_at(self.window.handle(), self.event_number, index);
            let s = std::slice::from_raw_parts(ptr as *const u8, length);
            std::str::from_utf8_unchecked(s)
        }
    }

    pub fn set_response(&mut self, response: &str) {
        let cstring = CString::new(response).unwrap();
        unsafe {
            ffi::webui_interface_set_response(
                self.window.handle(),
                self.event_number,
                cstring.as_ptr(),
            );
        }
    }

    pub fn set_cstr_response(&mut self, response: &std::ffi::CStr) {
        unsafe {
            ffi::webui_interface_set_response(
                self.window.handle(),
                self.event_number,
                response.as_ptr(),
            );
        }
    }
}

unsafe extern "C" fn event_handler(
    window_number: usize,
    event_type: usize,
    element_ptr: *mut std::os::raw::c_char,
    event_number: usize,
    bind_id: usize,
) {
    let window = Window {
        handle: window_number,
    };
    let element = CStr::from_ptr(element_ptr)
        .to_str()
        .expect("element name is not valid utf8");
    let mut event = Event {
        window,
        event_type: event_type.try_into().unwrap(),
        element,
        event_number,
        bind_id,
    };
    {
        let cbs = EVENT_HANDLERS.read().unwrap();
        cbs[&bind_id](&mut event);
    }
}

pub fn wait() {
    unsafe { ffi::webui_wait() }
}

pub fn clean() {
    unsafe { ffi::webui_clean() }
}

pub fn is_app_running() -> bool {
    unsafe { ffi::webui_interface_is_app_running() }
}

pub fn set_timeout(second: usize) {
    unsafe { ffi::webui_set_timeout(second) }
}

pub fn get_new_window_id() -> usize {
    unsafe { ffi::webui_get_new_window_id() }
}

pub fn exit() {
    unsafe { ffi::webui_exit() }
}
