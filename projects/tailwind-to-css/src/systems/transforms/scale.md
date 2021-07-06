Utilities for scaling elements with transform.

## Example

```rust
use tailwind_css::TailwindBuilder;

#[test]
fn build_scale() {
    let builder = TailwindBuilder::default();
    let out = format!("{:?}", builder.inline("scale-0"));
    assert_eq!(out, "{transform: scale(0);}")
}
```

## Reference

- [scale](https://tailwindcss.com/docs/scale)
- [scale/cn](https://tailwindcss.cn/docs/scale)