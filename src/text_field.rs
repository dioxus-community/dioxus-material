use crate::use_theme;
use dioxus::prelude::*;
use dioxus_spring::{use_animated, use_spring};
use dioxus_use_mounted::use_mounted;
use std::time::Duration;

/// Text field component.
/// 
/// Text fields let users enter text into a UI.
/// 
/// [material.io](https://m3.material.io/components/text-fields)
/// 
/// ## Panics
/// This component requires access to a [`Theme`](crate::Theme).
/// 
/// ## Examples
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_material::{TextField, Theme};
/// 
/// fn app(cx: Scope) -> Element {
///     let value = use_state(cx, || String::from("Filled"));
///     render!(
///         Theme { 
///             TextField {
///                 label: "Text field",
///                 value: "{value}",
///                 onchange: move |event: FormEvent| value.set(event.value.clone())
///             }
///         }
///     )
/// }
/// ```
#[component]
pub fn TextField<'a>(
    cx: Scope<'a>,
    label: &'a str,
    value: &'a str,
    onchange: EventHandler<'a, FormEvent>,
    background: Option<&'a str>,
    font_size: Option<f32>,
    width: Option<&'a str>
) -> Element<'a> {
    let is_populated = use_state(cx, || !value.is_empty());
    let theme = use_theme(cx);

    let font_size = font_size.unwrap_or(theme.label_medium);
    let spring = use_spring(
        cx,
        if **is_populated {
            [10f32, 12f32, 16f32]
        } else {
            [20., font_size, 24.]
        },
        Duration::from_millis(50),
    );

    let mounted = use_mounted(cx);
    use_animated(cx, mounted, spring, |[top, font_size, line_height]| {
        format!(
            r"
            position: absolute;
            top: {top}px;
            left: 20px;
            font-size: {font_size}px;
            line-height: {line_height}px;
        "
        )
    });

    let background = background.unwrap_or(&theme.background_color);
    let width = width.unwrap_or("200px");

    render!(
        div {
            position: "relative",
            display: "flex",
            width: width,
            background: "{background}",
            font_family: "sans-serif",
            border_bottom: "2px solid #999",
            label { onmounted: move |event| mounted.onmounted(event), "{label}" }
            input {
                position: "relative",
                z_index: 9,
                r#type: "text",
                value: *value,
                padding: "10px 20px",
                padding_top: "30px",
                font_size: "{font_size}px",
                height: "34px",
                border: "none",
                outline: "none",
                background: "none",
                onfocusin: move |_| {
                    if !is_populated {
                        is_populated.set(true)
                    }
                },
                onfocusout: move |_| {
                    if **is_populated && value.is_empty() {
                        is_populated.set(false)
                    }
                },
                oninput: move |event| onchange.call(event)
            }
        }
    )
}
