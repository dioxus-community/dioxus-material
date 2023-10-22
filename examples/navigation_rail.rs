use dioxus::prelude::*;
use dioxus_material::{NavigationRail, NavigationRailItem, Theme};

fn app(cx: Scope) -> Element {
    render!(
        Theme {
            NavigationRail {
                NavigationRailItem {
                    icon: render!("A"),
                    label: render!("All files"),
                    is_selected: false,
                    onselect: |_| {}
                }
                NavigationRailItem { icon: render!("B"), label: render!("Recent"), is_selected: true, onselect: |_| {} }
                NavigationRailItem {
                    icon: render!("C"),
                    label: render!("Photos"),
                    is_selected: false,
                    onselect: |_| {}
                }
            }
        }
    )
}

fn main() {
    dioxus_logger::init(log::LevelFilter::Info).expect("failed to init logger");

    dioxus_web::launch(app)
}
