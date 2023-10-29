use dioxus::prelude::*;
use dioxus_resize_observer::use_size;
use dioxus_spring::{use_animated, use_spring_signal, UseSpringSignal};
use dioxus_use_mounted::{use_mounted, UseMounted};
use std::time::Duration;

/// Hook to create a ripple from a container and animated component.
pub fn use_ripple<T>(cx: Scope<T>, duration: Duration) -> UseRipple {
    let is_pressed = use_state(cx, || false);

    let container_ref = use_mounted(cx);
    let content_rect = use_size(cx, container_ref);
    let size = content_rect.width().max(content_rect.height()) * 1.2;

    let (spring_ref, value_ref) = use_spring_signal(cx, [0f32; 2]);
    let animated_ref = use_mounted(cx);

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

    UseRipple {
        spring_ref,
        size,
        is_pressed,
        duration,
        container_ref,
        animated_ref,
    }
}

#[derive(Clone, Copy)]
pub struct UseRipple<'a> {
    pub spring_ref: UseSpringSignal<[f32; 2]>,
    pub size: f64,
    pub is_pressed: &'a UseState<bool>,
    pub duration: Duration,
    pub container_ref: UseMounted,
    pub animated_ref: UseMounted,
}

impl UseRipple<'_> {
    pub fn onmousedown(&self) {
        self.spring_ref.animate([self.size as _, 1.], self.duration);
        self.is_pressed.set(true)
    }

    pub fn onmouseup(&self) -> bool {
        if **self.is_pressed {
            self.spring_ref.queue([self.size as _, 0.], self.duration);
            self.spring_ref.queue([0., 0.], Duration::ZERO);
            self.is_pressed.set(false);
            true
        } else {
            false
        }
    }

    pub fn onmouseleave(&self) {
        if **self.is_pressed {
            self.spring_ref.animate([0., 0.], self.duration);
            self.is_pressed.set(false)
        }
    }
}
