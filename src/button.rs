use crate::{use_theme, Ripple};
use dioxus::prelude::*;

/// Filled button component.
///
/// Buttons let people take action and make choices with one tap.
///
/// [material.io](https://m3.material.io/components/buttons)
///
/// ## Panics
/// This component requires access to a [`Theme`](crate::Theme).
///
/// ## Examples
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_material::{Button, Theme};
///
/// fn app() -> Element {
///    rsx!(Theme {
///         Button { onpress: |_| log::info!("clicked!"), "Click me!" } }
///    )
/// }
/// ```
#[component]
pub fn Button(
    /// Handler for button press events.
    onpress: EventHandler<Event<MouseData>>,

    /// Label child element.
    children: Element,

    /// Background color of the container (optional).
    background_color: Option<String>,

    /// Border radius of the container (optional).
    border_radius: Option<String>,

    /// Height of the container (optional).
    height: Option<String>,
) -> Element {
    let theme = use_theme();
    let background_color = background_color.as_deref().unwrap_or(&theme.primary_color);
    let border_radius = border_radius
        .as_deref()
        .unwrap_or(&theme.border_radius_medium);
    let height = height.as_deref().unwrap_or("50px");

    rsx!(
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
                    {children}
                }
            }
        }
    )
}

#[component]
pub fn TextButton(
    /// Handler for button press events.
    onpress: EventHandler<Event<MouseData>>,

    /// Label child element.
    children: Element,

    /// Border radiusof the container (optional).
    border_radius: Option<String>,

    /// Text color (optional).
    color: Option<String>,

    /// Height of the container (optional).
    height: Option<String>,
) -> Element {
    let theme = use_theme();
    let color = color.as_deref().unwrap_or(&theme.primary_color);
    let border_radius = border_radius
        .as_deref()
        .unwrap_or(&theme.border_radius_medium);
    let height = height.as_deref().unwrap_or("40px");

    rsx!(
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
                    {children}
                }
            }
        }
    )
}
