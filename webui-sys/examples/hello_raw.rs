use std::ffi::CString;

fn main() {
    unsafe {
        let window_id = webui_sys::webui_new_window();
        let content =
            CString::new("<html><script src=\"webui.js\"></script> Hello World from Rust! </html>")
                .unwrap();
        let res = webui_sys::webui_show(window_id, content.as_ptr());
        dbg!(res);
        webui_sys::webui_wait();
    }
}
