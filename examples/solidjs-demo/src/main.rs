use rust_webui::Window;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Debug, Serialize)]
#[serde(tag = "t", content = "c")]
enum JsonResult<T>
where
    T: Serialize,
{
    Ok(T),
    Err(String),
}

impl<T> From<anyhow::Result<T>> for JsonResult<T>
where
    T: Serialize,
{
    fn from(value: anyhow::Result<T>) -> Self {
        match value {
            Ok(v) => JsonResult::Ok(v),
            Err(err) => JsonResult::Err(err.to_string()),
        }
    }
}

fn callable<I, O>(f: impl Fn(I) -> anyhow::Result<O> + 'static) -> impl Fn(&mut rust_webui::Event)
where
    I: DeserializeOwned,
    O: Serialize,
{
    Box::new(move |event: &mut rust_webui::Event| {
        let mut ff = || -> anyhow::Result<()> {
            let input_str = event.get_string_at(0);
            let input: I = serde_json::from_str(input_str)?;
            let output_result = JsonResult::from(f(input));
            let output_string = serde_json::to_string(&output_result)?;
            event.set_response(&output_string);
            Ok(())
        };

        ff().unwrap_or_else(|err| {
            event.set_response(&err.to_string());
        });
    })
}

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

    #[derive(Debug, Deserialize)]
    struct Add2Input {
        x: f64,
        y: f64,
    }

    window.bind(
        "add2",
        callable(|input: Add2Input| {
            dbg!(&input);
            Ok(input.x + input.y)
        }),
    );

    rust_webui::wait();
    rust_webui::clean();
}
