use crate::Ripple;
use dioxus::prelude::*;
use dioxus_resize_observer::{use_resize, Rect};
use dioxus_signals::{use_signal, Signal};
use dioxus_spring::{use_animated, use_spring};
use dioxus_use_mounted::use_mounted;
use std::{collections::HashMap, time::Duration};

#[component]
pub fn TabRow<'a>(
    cx: Scope<'a>,
    tabs: &'a [Element<'a>],
    onselect: EventHandler<'a, usize>,
) -> Element<'a> {
    let sizes = use_signal(cx, || HashMap::new());
    let selected = use_state(cx, || 0);

    let width = sizes
        .read()
        .get(&**selected)
        .map(|rect: &Rect| rect.width() as f32)
        .unwrap_or_default();
    let left: f32 = (0..**selected)
        .map(|idx| {
            sizes
                .read()
                .get(&idx)
                .map(|rect: &Rect| rect.width() as f32)
                .unwrap_or_default()
        })
        .sum();

    let value_ref = use_spring(cx, [width, left], Duration::from_millis(200));
    let animated_ref = use_mounted(cx);
    use_animated(cx, animated_ref, value_ref, |[width, left]| {
        format!(
            r"
            position: absolute;
            bottom: 0;
            left: {left}px;
            width: {width}px;
            height: 4px;
            background: #416529;
            "
        )
    });

    render!(
        div { position: "relative",
            ul { display: "flex", flex_direction: "row", justify_content: "space-evenly", list_style: "none", margin: 0, padding: 0,
                tabs.iter().enumerate().map(|(idx, tab)| render!(TabRowItem { index: idx, sizes: sizes, onselect: move |idx| {
                    onselect.call(idx);
                    selected.set(idx);
                }, tab }))
            }
            div { onmounted: move |event| animated_ref.onmounted(event) }
        }
    )
}

#[component]
fn TabRowItem<'a>(
    cx: Scope<'a, TabRowItemProps<'a>>,
    children: Element<'a>,
    index: usize,
    sizes: Signal<HashMap<usize, Rect>>,
    onselect: EventHandler<'a, usize>,
) -> Element<'a> {
    let mounted = use_mounted(cx);
    let resize = use_resize(cx, mounted);

    let idx = *index;
    let sizes = *sizes;
    dioxus_signals::use_effect(cx, move || {
        if let Some(content_rect) = &*resize.read() {
            sizes
                .write()
                .entry(idx)
                .and_modify(|rect| *rect = content_rect.clone())
                .or_insert(content_rect.clone());
        }
    });

    render!(
        li {
            display: "flex",
            flex_direction: "row",
            flex: 1,
            margin: 0,
            padding: 0,
            onmounted: move |event| mounted.onmounted(event),
            Ripple { onclick: move |_| onselect.call(idx), children }
        }
    )
}
