use dioxus::prelude::*;
use dioxus_material::{TextButton, use_theme};
use lookbook::preview;

/// Buttons let people take action and make choices with one tap.
#[preview]
pub fn TextButtonPreview(
    cx: Scope,
    /// Label for the text button.
    #[lookbook(default = "Label")]
    label: &'a str,

    /// Background color of the container (optional).
    #[lookbook(default = &*use_theme(cx).primary_color)]
    color: &'a str,

    /// Background color of the container (optional).
    #[lookbook(default = &*use_theme(cx).border_radius_medium)]
    border_radius: &'a str,
) -> Element {
    render!(
        TextButton { color: color, border_radius: border_radius, onpress: |_| {}, label }
    )
}
