use crate::{use_theme, Icon, IconKind, Ripple};
use dioxus::prelude::*;

/// Chip component.
///
/// Chips help people enter information, make selections, filter content, or trigger actions.
///
/// [material.io](https://m3.material.io/components/chips)
///
/// ## Panics
/// This component requires access to a [`Theme`](crate::Theme) and [`IconFont`](crate::IconFont).
///
/// ## Examples
/// ```rust
///
/// use dioxus::prelude::*;
/// use dioxus_material::{Chip, Theme, IconFont};
///
/// fn app(cx: Scope) -> Element {
///     rsx!(Theme {
///         IconFont {}
///         div { display: "flex", gap: "10px",
///             Chip { onclick: |_| {}, "Asset chip" }
///             Chip { is_selected: true, onclick: |_| {}, "Asset chip" }
///         }
///     })
/// }
/// ```
#[component]
pub fn Chip(
    children: Element,
    is_selected: Option<bool>,
    onclick: EventHandler<Event<MouseData>>,
) -> Element {
    let theme = use_theme();
    let (border_color, background) = if is_selected == Some(true) {
        (
            &*theme.secondary_container_color,
            &*theme.secondary_container_color,
        )
    } else {
        ("#79747E", "none")
    };

    rsx!(
        div {
            display: "inline-flex",
            flex_direction: "row",
            align_items: "center",
            height: "32px",
            line_height: "32px",
            border_radius: "{theme.border_radius_small}",
            font_family: "sans-serif",
            font_size: "14px",
            font_weight: 500,
            border: "1px solid {border_color}",
            background,
            Ripple { onclick: move |event| onclick.call(event),
                div {
                    display: "inline-flex",
                    flex_direction: "row",
                    align_items: "center",
                    if is_selected == Some(true) {
                        Icon { kind: IconKind::Check }
                    }

                    div { padding: "0 14px", {children} }
                }
            }
        }
    )
}
