use dioxus::prelude::*;

#[component]
pub fn Tab<'a>(cx: Scope<'a>, children: Element<'a>) -> Element<'a> {
    render!(div {
        font_size: "1.2em",
        padding: "10px 0",
        text_align: "center",
        children
    })
}
