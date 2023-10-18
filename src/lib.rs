use dioxus::prelude::*;
use dioxus_resize_observer::use_size;
use dioxus_spring::{use_animated, use_spring_signal};
use dioxus_use_mounted::use_mounted;
use std::time::Duration;

#[derive(Props)]
pub struct ButtonProps<'a> {
    onclick: EventHandler<'a, Event<MouseData>>,
    children: Element<'a>
}

#[component]
pub fn Button<'a>(cx: Scope<'a, ButtonProps<'a>>) -> Element<'a> {
    let is_pressed = use_state(cx, || false);

    let container_ref = use_mounted(cx);
    let (width, height) = use_size(cx, container_ref);
    let size = width.max(height) * 1.2;

    let (spring_ref, value_ref) = use_spring_signal(cx, [0f32; 2]);
    let animated_ref = use_mounted(cx);
    use_animated(cx, animated_ref, value_ref, |[size, opacity]| {
        format!(r"
            width: {size}px;
            height: {size}px;
            opacity: {opacity};
            position: absolute;
            top: 50%;
            left: 50%;
            transform: translate(-50%, -50%);
            border-radius: 50%;
            background: rgba(255, 255, 255, 0.5);
        "
        )
    });

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
            onmounted: move |event| container_ref.onmounted(event),
            onmousedown: move |_| {
                spring_ref.animate([size as _, 1.], Duration::from_millis(200));
                is_pressed.set(true)
            },
            onmouseup: move |event| {
                if **is_pressed {
                    spring_ref.queue([size as _, 0.], Duration::from_millis(200));
                    spring_ref.queue([0., 0.], Duration::from_millis(0));
                    cx.props.onclick.call(event);
                    is_pressed.set(false)
                }
            },
            onmouseleave: move |_| {
                spring_ref.animate([0., 0.], Duration::from_millis(200));
                is_pressed.set(false)
            },
            div { onmounted: move |event| animated_ref.onmounted(event) }
            div {
                position: "relative",
                z_index: 9,
                padding: "0 25px",
                font_family: "sans-serif",
                user_select: "none",
                webkit_user_select: "none",
                &cx.props.children
            }
        }
    )
}
