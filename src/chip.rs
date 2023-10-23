use crate::{use_theme, Icon, IconKind};
use dioxus::prelude::*;

#[component]
pub fn Chip<'a>(cx: Scope, children: Element<'a>, is_selected: Option<bool>) -> Element<'a> {
    let theme = use_theme(cx);

    let (border_color, background) = if *is_selected == Some(true) {
        (
            &*theme.secondary_container_color,
            &*theme.secondary_container_color,
        )
    } else {
        ("#79747E", "none")
    };

    render!(
        div {
            display: "inline-flex",
            flex_direction: "row",
            align_items: "center",
            height: "32px",
            line_height: "32px",
            border_radius: "{theme.border_radius_small}",
            padding: "0 14px",
            font_family: "sans-serif",
            font_size: "14px",
            font_weight: 500,
            border: "1px solid {border_color}",
            background: background,
            if *is_selected == Some(true) {
                render!(Icon { kind: IconKind::Check })
            } else {
                None
            }
            children
        }
    )
}
