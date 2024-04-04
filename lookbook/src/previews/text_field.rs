use dioxus::prelude::*;
use dioxus_material::TextField;
use lookbook::preview;

/// Text fields let users enter text into a UI.
#[preview]
pub fn TextFieldPreview<'a>(
    cx: Scope,
    /// Label for the text field.
    #[lookbook(default = "Label")]
    label: &'a str,
) -> Element {
    let value = use_state(cx, || String::from("Text Field"));

    rsx!(TextField {
        label,
        value,
        onchange: move |event: FormEvent| value.set(event.data.value.clone())
    })
}
