use crate::{use_theme, Ripple};
use dioxus::prelude::*;

#[component]
pub fn Button<'a>(
    cx: Scope<'a>,

    /// Handler for button press events.
    onpress: EventHandler<'a, Event<MouseData>>,

    /// Label child element.
    children: Element<'a>,

    /// Background color of the container (optional).
    background_color: Option<&'a str>,

    /// Border radius of the container (optional).
    border_radius: Option<&'a str>,

    /// Height of the container (optional).
    height: Option<&'a str>,
) -> Element<'a> {
    let theme = use_theme(cx);
    let background_color = background_color.unwrap_or(&theme.primary_color);
    let border_radius = border_radius.unwrap_or(&theme.border_radius_medium);
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
            Ripple { onclick: move |event| onpress.call(event),
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

    /// Handler for button press events.
    onpress: EventHandler<'a, Event<MouseData>>,

    /// Label child element.
    children: Element<'a>,

    /// Border radiusof the container (optional).
    border_radius: Option<&'a str>,

    /// Text color (optional).
    color: Option<&'a str>,

    /// Height of the container (optional).
    height: Option<&'a str>,
) -> Element<'a> {
    let theme = use_theme(cx);
    let color = color.unwrap_or(&theme.primary_color);
    let border_radius = border_radius.unwrap_or(&theme.border_radius_medium);
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
            Ripple { onclick: move |event| onpress.call(event),
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
