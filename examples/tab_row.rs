use dioxus::prelude::*;
use dioxus_material::{Tab, TabRow, Theme};

fn app(cx: Scope) -> Element {
    render!(
        Theme {
            TabRow {
                onselect: |idx| log::info!("{}", idx),
                selected: 0,
                tabs: cx
                    .bump()
                    .alloc([
                        render!(Tab { "Tab 1" }),
                        render!(Tab { "Tab 2" }),
                        render!(Tab { "Tab 3" }),
                    ])
            }
        }
    )
}

fn main() {
    dioxus_logger::init(log::LevelFilter::Info).expect("failed to init logger");

    dioxus_web::launch(app)
}
