use crate::use_ripple;
use dioxus::prelude::*;
use std::time::Duration;

#[component]
pub fn Ripple(
    onclick: EventHandler<Event<MouseData>>,
    children: Element,
    duration: Option<Duration>,
) -> Element {
    let duration = duration.unwrap_or(Duration::from_millis(200));
    let mut ripple = use_ripple(duration);

    rsx!(
        div {
            display: "inline-flex",
            flex: 1,
            position: "relative",
            overflow: "hidden",
            cursor: "pointer",
            onmounted: move |event| ripple.container_ref.onmounted(event),
            onmousedown: move |_| ripple.onmousedown(),
            onmouseup: move |event| {
                if ripple.onmouseup() {
                    onclick.call(event)
                }
            },
            onmouseleave: move |_| ripple.onmouseleave(),
            div { onmounted: move |event| ripple.animated_ref.onmounted(event) }
            div {
                position: "relative",
                z_index: 9,
                user_select: "none",
                webkit_user_select: "none",
                {children}
            }
        }
    )
}
