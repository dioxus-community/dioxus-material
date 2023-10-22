use dioxus::prelude::*;
use dioxus_material::{Button, TextButton, Theme};

fn app(cx: Scope) -> Element {
    render!(
        Theme { 
            Button { onpress: |_| log::info!("clicked!"), "Click me!" }
            TextButton { onpress: |_| log::info!("clicked!"), "Click me!" }
        }
    )
}

fn main() {
    dioxus_logger::init(log::LevelFilter::Info).expect("failed to init logger");

    dioxus_web::launch(app)
}
