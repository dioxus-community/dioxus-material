use dioxus::prelude::*;
use crate::Ripple;

#[component]
pub fn Button<'a>(
    cx: Scope<'a>,
    onclick: EventHandler<'a, Event<MouseData>>,
    children: Element<'a>,
) -> Element<'a> {
    render!(
        div {
            display: "inline-block",
            position: "relative",
            height: "50px",
            line_height: "50px",
            color: "#fff",
            background: "#416529",
            border_radius: "25px",
            overflow: "hidden",
            cursor: "pointer",
            Ripple { onclick: move |event| onclick.call(event),
                div {
                    position: "relative",
                    z_index: 9,
                    padding: "0 25px",
                    font_family: "sans-serif",
                    user_select: "none",
                    webkit_user_select: "none",
                    children
                }
            }
        }
    )
}

#[component]
pub fn TextButton<'a>(
    cx: Scope<'a>,
    onclick: EventHandler<'a, Event<MouseData>>,
    children: Element<'a>,
) -> Element<'a> {
    render!(
        div {
            display: "inline-block",
            position: "relative",
            height: "40px",
            line_height: "40px",
            border_radius: "25px",
            color: "#416529",
            font_weight: "bold",
            overflow: "hidden",
            cursor: "pointer",
            Ripple { onclick: move |event| onclick.call(event),
                div {
                    position: "relative",
                    z_index: 9,
                    padding: "0 25px",
                    font_family: "sans-serif",
                    user_select: "none",
                    webkit_user_select: "none",
                    children
                }
            }
        }
    )
}
