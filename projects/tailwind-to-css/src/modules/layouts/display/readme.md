Utilities for controlling how the browser should calculate an element's total size.

## Examples

```rust
use tailwind_css::TailwindBuilder;

#[test]
fn build_box_sizing() {
    let builder = TailwindBuilder::default();
    let out = builder.inline("box-border");
    assert_eq!(out, "box-sizing:border-box;");
    let out = builder.inline("box-content");
    assert_eq!(out, "box-sizing:content-box;");
}
```

## Reference

- [box-sizing](https://tailwindcss.com/docs/box-sizing)
- [box-sizing/cn](https://tailwindcss.c/docs/box-sizing)