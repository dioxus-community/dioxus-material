use dioxus::prelude::*;
use dioxus_material::{use_theme, Chip};
use lookbook::{preview, Json};

/// Buttons let people take action and make choices with one tap.
#[preview]
pub fn ChipPreview(
    cx: Scope,
    /// Label for the button.
    #[lookbook(default = "Label")]
    label: &'a str,

    /// Background color of the container (optional).
    #[lookbook(default = false)]
    is_selected: Json<bool>,

) -> Element {
    render!(
        Chip { is_selected: is_selected.0, onclick: |_| {}, label }
    )
}
