use dioxus::prelude::*;
use dioxus_material::{use_theme, Button};
use lookbook::preview;

/// Buttons let people take action and make choices with one tap.
#[preview]
pub fn ButtonPreview(
    cx: Scope,
    /// Label for the button.
    #[lookbook(default = "Label")]
    label: &'a str,

    /// Background color of the container (optional).
    #[lookbook(default = &*use_theme(cx).primary_color)]
    background_color: &'a str,

    /// Background color of the container (optional).
    #[lookbook(default = &*use_theme(cx).border_radius_medium)]
    border_radius: &'a str,
) -> Element {
    rsx!(Button {
        background_color,
        border_radius,
        onpress: |_| {},
        label
    })
}
