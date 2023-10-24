use dioxus::prelude::*;
use dioxus_material::{use_theme, Button, TextButton, TextField};
use lookbook::{preview, LookBook};

#[preview]
fn ButtonPreview(
    cx: Scope,
    /// Label for the button.
    #[lookbook(default = "Label")]
    label: &'a str,
) -> Element {
    let theme = use_theme(cx);
    let background_color = use_state(cx, || theme.primary_color.to_string());

    render!(Button {
        background_color: background_color,
        onpress: |_| {},
        label
    })
}

#[preview]

fn TextButtonPreview(
    cx: Scope,
    /// Label for the text button.
    #[lookbook(default = "Label")]
    label: &'a str,
) -> Element {
    render!(TextButton {
        onpress: |_| {},
        label
    })
}

/// Text fields let users enter text into a UI.
#[preview]
fn TextFieldPreview<'a>(
    cx: Scope,
    /// Label for the text field.
    #[lookbook(default = "Label")]
    label: &'a str,
) -> Element<'a> {
    let value = use_state(cx, || String::from("Text Field"));

    render!(TextField {
        label: label,
        value: value,
        onchange: move |event: FormEvent| value.set(event.data.value.clone())
    })
}

#[component]
fn Home(cx: Scope) -> Element {
    render!(
        div { padding: "20px",
            h1 { "Dioxus Material" }
            h5 { "Material You design library for dioxus." }
            a { href: "https://github.com/matthunz/dioxus-material", "Github" }

            div { margin_top: "20px",
                "Made with "
                a { href: "https://github.com/matthunz/lookbook", "Lookbook" }
                "."
            }
        }
    )
}

fn app(cx: Scope) -> Element {
    render!(LookBook {
        home: Home,
        previews: [ButtonPreview, TextButtonPreview, TextFieldPreview]
    })
}

fn main() {
    dioxus_web::launch(app);
}
