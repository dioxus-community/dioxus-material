use crate::IconKind;
use dioxus::prelude::*;

/// Material Symbols icon font.
#[component]
pub fn IconFont() -> Element {
    rsx!(
        link {
            href: "https://fonts.googleapis.com/css2?family=Material+Symbols+Rounded:opsz,wght,FILL,GRAD@20..48,100..700,0..1,-50..200",
            rel: "stylesheet"
        }
    )
}

/// Material Symbols icon.
#[component]
pub fn Icon(
    /// Kind of icon.
    kind: IconKind,

    /// Fill of the icon (optional).
    is_filled: Option<bool>,

    /// Weight of the icon (optional).
    weight: Option<u32>,

    /// Optical size of the icon (optional).
    size: Option<f32>,
) -> Element {
    // TODO memo
    let font_variation_settings = {
        let mut s = String::new();
        let mut is_first = true;
        if is_filled == Some(true) {
            if !is_first {
                s.push_str(", ");
            }
            s.push_str("'FILL' 1");
            is_first = false;
        }
        if let Some(weight) = weight {
            if !is_first {
                s.push_str(", ");
            }
            s.push_str(&format!("'wght' {}", weight));
            is_first = false;
        }
        if let Some(size) = size {
            if !is_first {
                s.push_str(", ");
            }
            s.push_str(&format!("'opsz' {}", size));
        }
        s
    };

    rsx!(
        span {
            class: "material-symbols-rounded",
            style: "font-variation-settings: {font_variation_settings};",
            { kind.name()}
        }
    )
}
