use once_cell::sync::OnceCell;
use std::{
    borrow::Borrow,
    collections::HashMap,
    ffi::{CStr, CString},
    path::Path,
    sync::{Mutex, RwLock},
};

use std::cell::RefCell;
use webui_sys as ffi;

thread_local! {
    static CALLBACKS: RefCell<HashMap<usize, Box<dyn Fn(Event)>>> = RefCell::new(HashMap::new());
}

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
        dbg!(&cstring);
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

    pub fn bind(&self, element: &str, func: impl Fn(Event) + 'static) {
        println!("element {}", element);
        let cstring = CString::new(element).unwrap();

        let bind_id =
            unsafe { ffi::webui_interface_bind(self.0, cstring.as_ptr(), Some(event_handler)) };

        println!("bind id: {}", bind_id);

        CALLBACKS.with(move |cbs| {
            cbs.borrow_mut().insert(bind_id, Box::new(func));
            dbg!(&cbs.borrow().keys());
        });
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

unsafe extern "C" fn event_handler(
    window: usize,
    event_type: usize,
    element_ptr: *mut std::os::raw::c_char,
    event_number: usize,
    bind_id: usize,
) {
    dbg!(window, event_type, bind_id);
    CALLBACKS.with(|cbs| {
        dbg!(cbs.borrow().keys());
        cbs.borrow().get(&bind_id).map(|cb| {
            let window = Window(window);
            let element = CStr::from_ptr(element_ptr).to_str().unwrap().to_string();
            let event = Event {
                window,
                event_type: event_type.try_into().unwrap(),
                element,
                event_number,
                bind_id,
            };
            cb(event);
        })
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
