use once_cell::sync::Lazy;
use std::{
    collections::HashMap,
    ffi::{CStr, CString},
    fmt::Debug,
    path::Path,
    sync::RwLock,
};

use webui_sys as ffi;

type CallbackMap = HashMap<usize, Box<dyn Fn(Event) + Send + Sync>>;

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
    file_handler: Option<Box<dyn Fn(&str)>>,
}

impl Debug for Window {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Window")
            .field("handle", &self.handle)
            .field("file_handler", &self.file_handler.is_some())
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
        let window_id = unsafe { ffi::webui_new_window() };
        Self {
            handle: window_id,
            file_handler: None,
        }
    }

    pub fn id(&self) -> usize {
        self.handle
    }

    pub fn with_id(window_number: usize) -> Self {
        unsafe {
            ffi::webui_new_window_id(window_number);
        }
        Self {
            handle: window_number,
            file_handler: None,
        }
    }

    pub fn show(&self, content: &str) -> bool {
        let cstring = CString::new(content).unwrap();
        dbg!(&cstring);
        unsafe { ffi::webui_show(self.id(), cstring.as_ptr()) }
    }

    pub fn show_browser(&self, content: &str, browser: Browser) -> bool {
        let cstring = CString::new(content).unwrap();
        unsafe { ffi::webui_show_browser(self.id(), cstring.as_ptr(), browser as usize) }
    }

    pub fn is_shown(&self) -> bool {
        unsafe { ffi::webui_is_shown(self.id()) }
    }

    pub fn set_size(&mut self, width: u32, height: u32) {
        unsafe {
            ffi::webui_set_size(self.id(), width, height);
        }
    }

    pub fn set_position(&mut self, x: u32, y: u32) {
        unsafe {
            ffi::webui_set_position(self.id(), x, y);
        }
    }

    pub fn set_root_folder(&mut self, path: impl AsRef<Path>) -> bool {
        let cstring = CString::new(path.as_ref().as_os_str().to_str().unwrap()).unwrap();
        unsafe { ffi::webui_set_root_folder(self.id(), cstring.as_ptr()) }
    }

    pub fn set_icon(&self, icon: &str, icon_type: &str) {
        // unsafe {ffi::webui_set_icon(self.id(), icon, icon_type)}
    }

    pub fn set_runtime(&mut self, runtime: Runtime) {
        unsafe {
            ffi::webui_set_runtime(self.id(), runtime as usize);
        }
    }

    // pub fn set_file_handler()

    pub fn bind(&self, element: &str, func: impl Fn(Event) + Send + Sync + 'static) {
        let cstring = CString::new(element).unwrap();

        let bind_id =
            unsafe { ffi::webui_interface_bind(self.id(), cstring.as_ptr(), Some(event_handler)) };

        {
            let mut cbs = EVENT_HANDLERS.write().unwrap();
            cbs.insert(bind_id, Box::new(func));
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
pub struct Event {
    pub window: Window,
    pub event_type: EventType,
    pub element: String,
    pub event_number: usize,
    pub bind_id: usize,
}

impl Event {
    pub fn get_int_at(&self, index: usize) -> i64 {
        unsafe { ffi::webui_interface_get_int_at(self.window.id(), self.event_number, index) }
    }

    pub fn get_bool_at(&self, index: usize) -> bool {
        unsafe { ffi::webui_interface_get_bool_at(self.window.id(), self.event_number, index) }
    }

    pub fn get_string_at(&self, index: usize) -> &str {
        unsafe {
            let ptr =
                ffi::webui_interface_get_string_at(self.window.id(), self.event_number, index);
            let length =
                ffi::webui_interface_get_size_at(self.window.id(), self.event_number, index);
            let s = std::slice::from_raw_parts(ptr as *const u8, length);
            std::str::from_utf8_unchecked(s)
        }
    }
}

unsafe extern "C" fn event_handler(
    window: usize,
    event_type: usize,
    element_ptr: *mut std::os::raw::c_char,
    event_number: usize,
    bind_id: usize,
) {
    let window = Window::with_id(window);
    let element = CStr::from_ptr(element_ptr).to_str().unwrap().to_string();
    let event = Event {
        window,
        event_type: event_type.try_into().unwrap(),
        element,
        event_number,
        bind_id,
    };
    {
        let cbs = EVENT_HANDLERS.read().unwrap();
        cbs[&bind_id](event);
    }
}

// unsafe extern "C" fn file_handler(

// )

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
