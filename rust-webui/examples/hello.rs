use rust_webui::Window;

fn main() {
    let window = Window::new();
    window.show("<html><script src=\"webui.js\"></script> Hello World from Rust! </html>");
    rust_webui::wait();
}
