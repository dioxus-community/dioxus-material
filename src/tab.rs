use dioxus::prelude::*;

#[component]
pub fn Tab(children: Element) -> Element {
    rsx!(
        div { font_size: "1.2em", padding: "10px 0", text_align: "center", {children} }
    )
}
