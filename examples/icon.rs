use dioxus::prelude::*;
use dioxus_material::{Icon, IconFont, IconKind};

fn app(cx: Scope) -> Element {
    render!(
        IconFont {}
        Icon { kind: IconKind::Search }
        Icon { kind: IconKind::Home, is_filled: true, size: 100. }
    )
}

fn main() {
    dioxus_logger::init(log::LevelFilter::Info).expect("failed to init logger");

    dioxus_web::launch(app)
}
