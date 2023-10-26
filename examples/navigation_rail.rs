use dioxus::prelude::*;
use dioxus_material::{Icon, IconFont, IconKind, NavigationRail, NavigationRailItem, Theme};

fn app(cx: Scope) -> Element {
    render!(
        IconFont {}
        Theme {
            NavigationRail {
                NavigationRailItem {
                    icon: render!(Icon { kind : IconKind::Home }),
                    label: render!("All files"),
                    is_selected: false,
                    onselect: |_| {}
                }
                NavigationRailItem {
                    icon: render!(Icon { kind : IconKind::History }),
                    label: render!("Recent"),
                    is_selected: true,
                    onselect: |_| {}
                }
                NavigationRailItem {
                    icon: render!(Icon { kind : IconKind::PhotoAlbum }),
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
