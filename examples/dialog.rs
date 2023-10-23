use dioxus::prelude::*;
use dioxus_material::{Dialog, Theme};

fn app(cx: Scope) -> Element {
    render!(
        Theme { 
            Dialog { is_visible: true, h1 { "Dialog" } }
        }
    )
}

fn main() {
    dioxus_logger::init(log::LevelFilter::Info).expect("failed to init logger");

    dioxus_web::launch(app)
}
