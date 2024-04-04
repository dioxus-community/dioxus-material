use dioxus::prelude::*;
use std::{borrow::Cow, rc::Rc};

/// Theme component.
///
/// This component provides access to [`UseTheme`](UseTheme) to its children.
#[component]
pub fn Theme(
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

    children: Element,
) -> Element {
    use_context_provider(move || {
        Rc::new(UseTheme {
            primary_color: primary_color.clone(),
            background_color: background_color.clone(),
            secondary_container_color: secondary_container_color.clone(),
            border_radius_medium: border_radius_medium.clone(),
            border_radius_small: border_radius_small.clone(),
            label_small,
            label_medium,
        })
    });

    children
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

pub fn use_theme() -> Rc<UseTheme> {
    use_context()
}
