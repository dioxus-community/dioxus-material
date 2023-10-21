use dioxus::prelude::*;
use dioxus_spring::{use_animated, use_spring};
use dioxus_use_mounted::use_mounted;
use std::time::Duration;

#[component]
pub fn TextField<'a>(
    cx: Scope<'a>,
    label: &'a str,
    value: &'a str,
    onchange: EventHandler<'a, FormEvent>,
) -> Element<'a> {
    let is_populated = use_state(cx, || !value.is_empty());

    let spring = use_spring(
        cx,
        if **is_populated {
            [10f32, 12f32, 16f32]
        } else {
            [20., 16., 24.]
        },
        Duration::from_millis(200),
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

    render!(
        div {
            position: "relative",
            width: "200px",
            background: "#eeeeee",
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
                font_size: "16px",
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
