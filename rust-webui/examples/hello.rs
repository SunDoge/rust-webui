use rust_webui::Window;

fn main() {
    let window = Window::new();
    dbg!(&window);
    let res = window.show(
        r#"
    <html>
    <script src="webui.js"></script>
    Hello World from Rust! 
    <button id="x">click</button>
    </html>
    "#,
    );
    dbg!(res);
    dbg!(&window);
    window.bind("x", |event| {
        dbg!(event);
    });
    // rust_webui::set_timeout(0);
    rust_webui::wait();
    rust_webui::clean();
}
