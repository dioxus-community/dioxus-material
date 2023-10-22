use dioxus::prelude::*;
use dioxus_material::{use_theme, Button, TextButton, TextField};
use lookbook::{preview, Look, LookBook};

#[preview]
fn ButtonPreview(cx: Scope) -> Element {
    let label = use_state(cx, || String::from("Filled Button"));
    let theme = use_theme(cx);
    let background_color = use_state(cx, || theme.primary_color.to_string());

    render! {
        Look {
            name: "Button",
            controls: render! {
                TextField { label : "Label", value : label, onchange : move | event : FormEvent |
                label.set(event.data.value.clone()) } TextField { label : "Background color", value :
                background_color, onchange : move | event : FormEvent | background_color.set(event
                .data.value.clone()) }
            },
            Button { background_color: background_color, onpress: |_| {}, &*** label }
        }
    }
}

#[preview]
fn TextButtonPreview(cx: Scope) -> Element {
    let label = use_state(cx, || String::from("Text Button"));

    render!(
        Look {
            name: "TextButton",
            controls: render!(
                TextField { label : "Label", value : label, onchange : move | event : FormEvent |
                label.set(event.data.value.clone()) }
            ),
            TextButton { onpress: |_| {}, &*** label }
        }
    )
}

#[preview]
fn TextFieldPreview(cx: Scope) -> Element {
    let value = use_state(cx, || String::from("Text Field"));
    let label = use_state(cx, || String::from("Label"));

    render!(
        Look {
            name: "TextField",
            controls: render!(
                TextField { label : "Label", value : label, onchange : move | event : FormEvent |
                label.set(event.data.value.clone()) }
            ),
            TextField {
                label: &**label,
                value: value,
                onchange: move |event: FormEvent| value.set(event.data.value.clone())
            }
        }
    )
}

#[component]
fn Home(cx: Scope) -> Element {
    render!(
        div { padding: "20px",
            h1 { "Dioxus Material" }
            h5 { "Material You design library for dioxus." }
            a { href: "https://github.com/matthunz/dioxus-material", "Github" }

            div {
                margin_top: "20px",
                "Made with "
                a { href: "https://github.com/matthunz/lookbook", "Lookbook" }
                "."
            }
            
        }
    )
}

fn app(cx: Scope) -> Element {
    render!(
        LookBook {
            home: Home,
            previews: [ButtonPreview, TextButtonPreview, TextFieldPreview]
        }
    )
}

fn main() {
    dioxus_web::launch(app);
}
