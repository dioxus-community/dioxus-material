use dioxus::prelude::*;
use std::{borrow::Cow, rc::Rc};

#[component]
pub fn Theme<'a>(
    cx: Scope<'a>,

    /// Primary color.
    #[props(into, default = Cow::Borrowed("#6750A4"))]
    primary_color: Cow<'static, str>,

    /// Background color.
    #[props(into, default = Cow::Borrowed("#eeeeee"))]
    background_color: Cow<'static, str>,

    /// Secondary container color.
    #[props(into, default = Cow::Borrowed("#E8DEF8"))]
    secondary_container_color: Cow<'static, str>,

    /// Border radius medium.
    #[props(into, default = Cow::Borrowed("25px"))]
    border_radius_medium: Cow<'static, str>,

    /// Border radius.
    #[props(into, default = Cow::Borrowed("8px"))]
    border_radius_small: Cow<'static, str>,

    /// Small label font size.
    #[props(default = 12.)]
    label_small: f32,

    /// Medium label font size.
    #[props(default = 16.)]
    label_medium: f32,

    children: Element<'a>,
) -> Element<'a> {
    use_context_provider(cx, move || {
        Rc::new(UseTheme {
            primary_color: primary_color.clone(),
            background_color: background_color.clone(),
            secondary_container_color: secondary_container_color.clone(),
            border_radius_medium: border_radius_medium.clone(),
            border_radius_small: border_radius_small.clone(),
            label_small: *label_small,
            label_medium: *label_medium,
        })
    });

    render!(children)
}

pub struct UseTheme {
    pub primary_color: Cow<'static, str>,
    pub background_color: Cow<'static, str>,
    pub secondary_container_color: Cow<'static, str>,
    pub border_radius_medium: Cow<'static, str>,
    pub border_radius_small: Cow<'static, str>,
    pub label_small: f32,
    pub label_medium: f32,
}

pub fn use_theme<T>(cx: Scope<T>) -> &UseTheme {
    let rc: &Rc<UseTheme> = use_context(cx).unwrap();
    rc.as_ref()
}
