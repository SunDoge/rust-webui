use rust_webui::Window;

fn main() {
    let window = Window::new();
    let res = window.show(
        r#"
    <html>
    <script src="webui.js"></script>
    Hello World from Rust! 
    <button id="x">click</button>
    <button onclick="webui.call('func1', 'hello', 1, true)">pass args</button>
    </html>
    "#,
    );
    dbg!(res);
    window.bind("x", |event| {
        dbg!(&event);
        // event.set_response("");
    });
    // window.bind("func1", |event| {
    //     dbg!(event.get_string_at(0));
    //     dbg!(event.get_int_at(1));
    //     dbg!(event.get_bool_at(2));
    // });
    rust_webui::wait();
    // rust_webui::clean();
}
