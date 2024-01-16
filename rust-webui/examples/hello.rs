use rust_webui::Window;

fn main() {
    let window = Window::new();
    window.show(
        r#"
    <html>
    <script src="webui.js"></script>
    Hello World from Rust! 
    <button id="x">click</button>
    </html>
    "#,
    );
    window.bind("x", |event| {
        dbg!(event);
    });
    // rust_webui::set_timeout(0);
    rust_webui::wait();
    rust_webui::clean();
}
