use dioxus::prelude::*;
use dioxus_material::{Button, TextButton};

fn app(cx: Scope) -> Element {
    render!(
        Button { onclick: |_| log::info!("clicked!"), "Click me!" }
        TextButton { onclick: |_| log::info!("clicked!"), "Click me!" }
    )
}

fn main() {
    dioxus_logger::init(log::LevelFilter::Info).expect("failed to init logger");

    dioxus_web::launch(app)
}
