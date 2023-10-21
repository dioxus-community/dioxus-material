use crate::{use_theme, Ripple};
use dioxus::prelude::*;

#[component]
pub fn Button<'a>(
    cx: Scope<'a>,
    onclick: EventHandler<'a, Event<MouseData>>,
    children: Element<'a>,
    background_color: Option<&'a str>,
    border_radius: Option<&'a str>,
    height: Option<&'a str>,
) -> Element<'a> {
    let theme = use_theme(cx);
    let background_color = background_color.unwrap_or(&theme.primary_color);
    let border_radius = border_radius.unwrap_or(&theme.border_radius);
    let height = height.unwrap_or("50px");

    render!(
        div {
            display: "inline-block",
            position: "relative",
            height: "{height}",
            line_height: "{height}",
            color: "#fff",
            background: "{background_color}",
            border_radius: "{border_radius}",
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
    border_radius: Option<&'a str>,
    color: Option<&'a str>,
    height: Option<&'a str>,
) -> Element<'a> {
    let theme = use_theme(cx);
    let color = color.unwrap_or(&theme.primary_color);
    let border_radius = border_radius.unwrap_or(&theme.border_radius);
    let height = height.unwrap_or("40px");

    render!(
        div {
            display: "inline-block",
            position: "relative",
            height: "{height}",
            line_height: "{height}",
            border_radius: "{border_radius}",
            color: "{color}",
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
