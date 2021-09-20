Utilities for controlling the font size of an element.

## Example

```rust
use tailwind_css::TailwindBuilder;

#[test]
fn build_aspect() {
    let builder = crate::TailwindBuilder::default();
    let out = format!("{:?}", builder.inline("aspect-square"));
    assert_eq!(out, "{aspect-ratio: 1 / 1;}")
}
```

## Reference

- [font-size](https://tailwindcss.com/docs/font-size)
- [font-size/cn](https://tailwindcss.cn/docs/font-size)
