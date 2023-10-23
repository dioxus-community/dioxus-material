use crate::use_theme;
use dioxus::prelude::*;

#[component]
pub fn Dialog<'a>(cx: Scope<'a>, children: Element<'a>, is_visible: bool) -> Element<'a> {
    let theme = use_theme(cx);

    render!(
        div {
            display: if *is_visible { "block" } else { "none" },
            position: "fixed",
            top: 0,
            left: 0,
            width: "100vw",
            height: "100vh",
            background: "rgba(0, 0, 0, 0.4)",
            div {
                position: "absolute",
                top: "50%",
                left: "50%",
                transform: "translate(-50%, -50%)",
                border_radius: "{theme.border_radius}",
                background: "#fff",
                children
            }
        }
    )
}
