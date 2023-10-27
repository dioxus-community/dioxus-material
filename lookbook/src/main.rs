use dioxus::prelude::*;
use lookbook::LookBook;

mod previews;
use previews::{ButtonPreview, TabRowPreview, TextButtonPreview, TextFieldPreview};

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
