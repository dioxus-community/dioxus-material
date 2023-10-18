use dioxus::prelude::*;
use dioxus_material::TabRow;

fn app(cx: Scope) -> Element {
    render!(
        div { TabRow { tabs: cx.bump().alloc([render!("Tab 1"), render!("Tab 2")]) } }
    )
}

fn main() {
    dioxus_logger::init(log::LevelFilter::Info).expect("failed to init logger");

    dioxus_web::launch(app)
}
