use dioxus::prelude::*;
use crate::IconKind;

#[component]
pub fn IconFont(cx: Scope) -> Element {
    render!(link {
        href: "https://fonts.googleapis.com/css2?family=Material+Symbols+Rounded",
        rel: "stylesheet"
    })
}

#[component]
pub fn Icon(
    cx: Scope,
    kind: IconKind,
    is_filled: Option<bool>,
    weight: Option<u32>,
    size: Option<f32>,
) -> Element {
    let font_variation_settings = use_memo(cx, (is_filled, weight, size), move |_| {
        let mut s = String::new();
        if *is_filled == Some(true) {
            s.push_str(" 'FILL' 1");
        }
        if let Some(weight) = weight {
            s.push_str(&format!(" 'wght' {}", weight));
        }
        if let Some(size) = size {
            s.push_str(&format!(" 'opsz' {}", size));
        }
        s
    });

    render!(
        span { class: "material-symbols-rounded", "font-variation-settings": "{font_variation_settings}", kind.name() }
    )
}
