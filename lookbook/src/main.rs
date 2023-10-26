use dioxus::prelude::*;
use dioxus_material::{use_theme, Button, Tab, TabRow, TextButton, TextField};
use lookbook::{preview, Json, LookBook};

/// Buttons let people take action and make choices with one tap.
#[preview]
fn ButtonPreview(
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
    render!(Button {
        background_color: background_color,
        border_radius: border_radius,
        onpress: |_| {},
        label
    })
}

/// Tabs show multiple options for information.
#[preview]
fn TabRowPreview<'a>(
    cx: Scope<'a>,

    /// Label for tab A.
    #[lookbook(default = vec![String::from("Tab A")])]
    tabs: Json<Vec<String>>,
) -> Element<'a> {
    render!(
        div { width: "500px",
            TabRow {
                onselect: |_| {},
                selected: 0,
                tabs: cx
                    .bump()
                    .alloc(tabs.0.iter().map(|label| render!(Tab { "{label}" })).collect::<Vec<_>>())
            }
        }
    )
}

/// Buttons let people take action and make choices with one tap.
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
        previews: [
            ButtonPreview,
            TabRowPreview,
            TextButtonPreview,
            TextFieldPreview
        ]
    })
}

fn main() {
    dioxus_web::launch(app);
}
