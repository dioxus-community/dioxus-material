# Dioxus-material

```rs
use_theme_provider(cx, Theme::default());

Button { onclick: |_| log::info!("clicked!"), "Click me!" }

TextButton { onclick: |_| log::info!("clicked!"), "Click me!" }

TabRow {
    onselect: |idx| log::info!("{}", idx),
    tabs: cx
        .bump()
        .alloc([
            render!(Tab { "Tab 1" }),
            render!(Tab { "Tab 2" }),
            render!(Tab { "Tab 3" }),
        ])
}
```
