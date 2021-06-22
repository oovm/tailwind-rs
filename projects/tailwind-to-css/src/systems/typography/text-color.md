Utilities for controlling the font smoothing of an element.

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

- [font-smoothing](https://tailwindcss.com/docs/font-smoothing)
- [font-smoothing/cn](https://tailwindcss.cn/docs/font-smoothing)
