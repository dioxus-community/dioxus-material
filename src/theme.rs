use dioxus::prelude::*;
use std::{borrow::Cow, rc::Rc};

pub struct Theme {
    pub primary_color: Cow<'static, str>,
    pub border_radius: Cow<'static, str>,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            primary_color: Cow::Borrowed("#416529"),
            border_radius: Cow::Borrowed("25px"),
        }
    }
}

pub fn use_theme_provider<T>(cx: Scope<T>, theme: Theme) {
    use_context_provider(cx, move || Rc::new(theme));
}

pub fn use_theme<T>(cx: Scope<T>) -> &Theme {
    let rc: &Rc<Theme> = use_context(cx).unwrap();
    rc.as_ref()
}
