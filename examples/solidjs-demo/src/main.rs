use rust_webui::Window;

fn main() {
    let mut window = Window::new();

    if cfg!(debug_assertions) {
        window.set_port(8080);
        window.show("http://localhost:3000");
    } else {
        // embed html and js
    }

    window.bind("add", |event| {
        let x: f64 = event.get_string_at(0).parse().unwrap();
        let y: f64 = event.get_string_at(1).parse().unwrap();
        let res = x + y;
        event.set_response(&res.to_string());
    });

    rust_webui::wait();
    rust_webui::clean();
}
