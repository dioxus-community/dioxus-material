use dioxus::prelude::*;
use dioxus_material::Button;

fn app(cx: Scope) -> Element {
    render!(
        Button { onclick: |_| log::info!("clicked!"), "Click me!" }
    )
}

fn main() {
    dioxus_logger::init(log::LevelFilter::Info).expect("failed to init logger");

    dioxus_web::launch(app)
}
