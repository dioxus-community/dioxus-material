<div align="center">
<h1>Dioxus Spring</h1>
 <a href="https://crates.io/crates/dioxus-spring">
    <img src="https://img.shields.io/crates/v/dioxus-spring?style=flat-square"
    alt="Crates.io version" />
  </a>
  <a href="https://docs.rs/dioxus-spring/latest/dioxus_spring/">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
      alt="docs.rs docs" />
  </a>
   <a href="https://github.com/matthunz/dioxus-spring/actions">
    <img src="https://github.com/matthunz/dioxus-spring/actions/workflows/ci.yml/badge.svg"
      alt="CI status" />
  </a>
</div>

<div align="center">
 <a href="https://github.com/matthunz/dioxus-spring/tree/main/examples">Examples</a>
</div>

<br>

Animation library for [Dioxus](https://dioxuslabs.com).

Pairs great with [dioxus-use-gesture](https://github.com/matthunz/dioxus-use-gesture)!



```rust
let is_big = use_state(cx, || false);
let spring = use_spring(
    cx,
    if **is_big { 2f32 } else { 1f32 },
    Duration::from_millis(500),
);

let mounted = use_mounted(cx);
use_animated(cx, mounted, spring, |scale| {
    format!("transform-origin: top left; transform: scale({scale})")
});

render!(
    div {
        onmounted: move |event| mounted.onmounted(event),
        onclick: move |_| is_big.set(!is_big),
        "Click me!"
    }
)
```
