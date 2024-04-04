use dioxus::prelude::*;
use dioxus_resize_observer::use_size;
use dioxus_spring::{use_animated, use_spring_signal, UseSpringSignal};
use dioxus_use_mounted::{use_mounted, UseMounted};
use std::time::Duration;

/// Hook to create a ripple from a container and animated component.
pub fn use_ripple(duration: Duration) -> UseRipple {
    let is_pressed = use_signal(|| false);

    let container_ref = use_mounted();
    let content_rect = use_size(container_ref);
    let size = content_rect.width().max(content_rect.height()) * 1.2;

    let (spring_ref, value_ref) = use_spring_signal([0f32; 2]);
    let animated_ref = use_mounted();

    use_animated(animated_ref, value_ref, |[size, opacity]| {
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
pub struct UseRipple {
    pub spring_ref: UseSpringSignal<[f32; 2]>,
    pub size: f64,
    pub is_pressed: Signal<bool>,
    pub duration: Duration,
    pub container_ref: UseMounted,
    pub animated_ref: UseMounted,
}

impl UseRipple {
    pub fn onmousedown(&mut self) {
        self.spring_ref.animate([self.size as _, 1.], self.duration);
        self.is_pressed.set(true)
    }

    pub fn onmouseup(&mut self) -> bool {
        if (self.is_pressed)() {
            self.spring_ref.queue([self.size as _, 0.], self.duration);
            self.spring_ref.queue([0., 0.], Duration::ZERO);
            self.is_pressed.set(false);
            true
        } else {
            false
        }
    }

    pub fn onmouseleave(&mut self) {
        if (self.is_pressed)() {
            self.spring_ref.animate([0., 0.], self.duration);
            self.is_pressed.set(false)
        }
    }
}
