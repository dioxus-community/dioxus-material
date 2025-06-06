use crate::use_theme;
use dioxus::prelude::*;

#[component]
pub fn NavigationRail(children: Element) -> Element {
    rsx!(
        nav {
            ul {
                display: "flex",
                flex_direction: "column",
                align_items: "center",
                width: "50px",
                list_style: "none",
                margin: 0,
                padding: "0 10px",
                {children}
            }
        }
    )
}

#[component]
pub fn NavigationRailItem(
    icon: Element,
    label: Element,
    is_selected: bool,
    onselect: EventHandler<MouseEvent>,
) -> Element {
    let theme = use_theme();

    rsx!(
        li {
            display: "flex",
            flex_direction: "column",
            align_items: "center",
            width: "100%",
            list_style: "none",
            padding: "10px 0",
            cursor: "pointer",
            font_family: "sans-serif",
            onclick: move |event| onselect.call(event),
            div {
                width: "100%",
                padding: "5px 0",
                text_align: "center",
                border_radius: "{theme.border_radius_medium}",
                background: if is_selected { Some(theme.secondary_container_color.to_string()) } else { None },
                {icon}
            }
            div {
                margin_top: "5px",
                font_size: "{theme.label_small}px",
                line_height: "16px",
                {label}
            }
        }
    )
}
