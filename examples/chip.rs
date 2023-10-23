use dioxus::prelude::*;
use dioxus_material::{Chip, Theme, IconFont};

fn app(cx: Scope) -> Element {
    render!(
        Theme { 
            IconFont {}
            div { display: "flex",
                Chip { "Asset chip" }
                Chip { is_selected: true, "Asset chip" }
            }
        }
    )
}

fn main() {
    dioxus_logger::init(log::LevelFilter::Info).expect("failed to init logger");

    dioxus_web::launch(app)
}
