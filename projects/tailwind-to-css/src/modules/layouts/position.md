Utilities for controlling how an element is positioned in the DOM.

## Position

```rust
use tailwind_css::{TailwindBuilder};

#[test]
fn build_position() {
    let builder = TailwindBuilder::default();
    let out = format!("{:?}", builder.inline("static"));
    assert_eq!(out, "{position: static;}")
}
```

## Reference

- [position](https://tailwindcss.com/docs/position)
- [position/cn](https://tailwindcss.cn/docs/position)
