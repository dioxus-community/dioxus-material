use crate::use_theme;
use dioxus::prelude::*;

#[component]
pub fn NavigationRail<'a>(cx: Scope<'a>, children: Element<'a>) -> Element<'a> {
    render!(
        nav {
            ul {
                display: "flex",
                flex_direction: "column",
                align_items: "center",
                width: "50px",
                list_style: "none",
                margin: 0,
                padding: 0,
                children
            }
        }
    )
}

#[component]
pub fn NavigationRailItem<'a>(
    cx: Scope<'a>,
    icon: Element<'a>,
    label: Element<'a>,
    is_selected: bool,
    onselect: EventHandler<'a, MouseEvent>,
) -> Element<'a> {
    let theme = use_theme(cx);

    render!(
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
                border_radius: "{theme.border_radius}",
                background: if *is_selected { &theme.secondary_container_color } else { "" },
                icon
            }
            div { margin_top: "5px", font_size: "{theme.label_small}px", line_height: "16px", label }
        }
    )
}
