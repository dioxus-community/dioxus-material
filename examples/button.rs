use dioxus::prelude::*;
use dioxus_material::{Button, Theme};

fn app() -> Element {
    rsx! {
        Theme {
            Button { onpress: |_| log::info!("clicked!"), "Click me!" }
        }
    }
}

fn main() {
    dioxus::launch(app)
}
