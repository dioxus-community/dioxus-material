<div align="center">
<h1>Dioxus Material</h1>
 <a href="https://crates.io/crates/dioxus-material">
    <img src="https://img.shields.io/crates/v/dioxus-material?style=flat-square"
    alt="Crates.io version" />
  </a>
  <a href="https://docs.rs/dioxus-material/latest/dioxus_material/">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
      alt="docs.rs docs" />
  </a>
    <a href="https://matthunz.github.io/">
    <img src="https://img.shields.io/badge/lookbook%20%F0%9F%91%80-purple"
      alt="lookbook docs" />
  </a>
   <a href="https://github.com/matthunz/dioxus-material/actions">
    <img src="https://github.com/matthunz/dioxus-material/actions/workflows/ci.yml/badge.svg"
      alt="CI status" />
  </a>
</div>

<div align="center">
 <a href="https://github.com/matthunz/dioxus-spring/tree/main/examples">Examples</a>
</div>

<br>

```rs
Theme {
  Button { onclick: |_| log::info!("clicked!"), "Click me!" }

  Icon { kind: IconKind::Home, is_filled: true, size: 100. }

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

  TextField {
      label: "Text field",
      value: "{value}",
      onchange: move |event: FormEvent| value.set(event.value.clone())
  }
}
```
