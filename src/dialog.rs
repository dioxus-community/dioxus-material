use crate::use_theme;
use dioxus::prelude::*;

/// Dialogs provide important prompts in a user flow.
///
/// [material.io](https://m3.material.io/components/dialogs)
///
/// ```
/// use dioxus::prelude::*;
/// use dioxus_material::{Dialog, Theme};
///
/// fn app() -> Element {
///     rsx!(
///         Theme {
///             Dialog { is_visible: true, h1 { "Dialog" } }
///         }
///     )
/// }
/// ```
#[component]
pub fn Dialog(children: Element, is_visible: bool) -> Element {
    let theme = use_theme();

    rsx!(
        div {
            display: if is_visible { "block" } else { "none" },
            position: "fixed",
            top: 0,
            left: 0,
            width: "100vw",
            height: "100vh",
            background: "rgba(0, 0, 0, 0.4)",
            div {
                position: "absolute",
                top: "50%",
                left: "50%",
                transform: "translate(-50%, -50%)",
                border_radius: "{theme.border_radius_medium}",
                background: "#fff",
                {children}
            }
        }
    )
}
