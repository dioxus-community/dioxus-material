use dioxus::prelude::*;
use dioxus_material::{Tab, TabRow};
use lookbook::{preview, Json};

/// Tabs show multiple options for information.
#[preview]
pub fn TabRowPreview<'a>(
    cx: Scope<'a>,

    /// Selected tab index.
    #[lookbook(default = 0)]
    selected: Json<usize>,

    /// Tab elements.
    #[lookbook(default = vec![String::from("Tab A"), String::from("Tab B")])]
    tabs: Json<Vec<String>>,
) -> Element {
    rsx!(
        div { width: "500px",
            TabRow {
                onselect: |_| {},
                selected: selected.0,
                tabs: cx.bump()
                    .alloc(tabs.0.iter().map(|label| rsx!(Tab { "{label}" })).collect::<Vec<_>>())
            }
        }
    )
}
