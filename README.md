# Dioxus-material

```rs
Button { onclick: |_| log::info!("clicked!"), "Click me!" }
```

```rs
TabRow {
    tabs: cx
        .bump()
        .alloc([
            render!(Tab { "Tab 1" }),
            render!(Tab { "Tab 2" }),
            render!(Tab { "Tab 3" }),
        ])
}
```
