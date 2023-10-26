use dioxus::prelude::*;
use dioxus_material::{TextField, Theme};

fn app(cx: Scope) -> Element {
    let value = use_state(cx, || String::from("Filled"));
    render!(
        Theme {
            TextField {
                label: "Text field",
                value: "{value}",
                onchange: move |event: FormEvent| value.set(event.value.clone())
            }
        }
    )
}

fn main() {
    dioxus_logger::init(log::LevelFilter::Info).expect("failed to init logger");

    dioxus_web::launch(app)
}
