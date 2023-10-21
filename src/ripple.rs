use dioxus::prelude::*;
use dioxus_resize_observer::use_size;
use dioxus_spring::{use_animated, use_spring_signal};
use dioxus_use_mounted::use_mounted;
use std::time::Duration;

#[component]
pub fn Ripple<'a>(
    cx: Scope<'a>,
    onclick: EventHandler<'a, Event<MouseData>>,
    children: Element<'a>,
    duration: Option<Duration>,
) -> Element<'a> {
    let is_pressed = use_state(cx, || false);

    let container_ref = use_mounted(cx);
    let content_rect = use_size(cx, container_ref);
    let size = content_rect.width().max(content_rect.height()) * 1.2;

    let (spring_ref, value_ref) = use_spring_signal(cx, [0f32; 2]);
    let animated_ref = use_mounted(cx);

    let duration = duration.unwrap_or(Duration::from_millis(200));

    use_animated(cx, animated_ref, value_ref, |[size, opacity]| {
        format!(
            r"
            width: {size}px;
            height: {size}px;
            opacity: {opacity};
            position: absolute;
            top: 50%;
            left: 50%;
            transform: translate(-50%, -50%);
            border-radius: 50%;
            background: rgba(150, 150, 150, 0.5);
            "
        )
    });

    render!(
        div {
            display: "inline-block",
            flex: 1,
            position: "relative",
            overflow: "hidden",
            cursor: "pointer",
            onmounted: move |event| container_ref.onmounted(event),
            onmousedown: move |_| {
                spring_ref.animate([size as _, 1.], duration);
                is_pressed.set(true)
            },
            onmouseup: move |event| {
                if **is_pressed {
                    spring_ref.queue([size as _, 0.], duration);
                    spring_ref.queue([0., 0.], Duration::ZERO);
                    onclick.call(event);
                    is_pressed.set(false)
                }
            },
            onmouseleave: move |_| {
                if **is_pressed {
                    spring_ref.animate([0., 0.], duration);
                    is_pressed.set(false)
                }
            },
            div { onmounted: move |event| animated_ref.onmounted(event) }
            div { position: "relative", z_index: 9, user_select: "none", webkit_user_select: "none", children }
        }
    )
}
