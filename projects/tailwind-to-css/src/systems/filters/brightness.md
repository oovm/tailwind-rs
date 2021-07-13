Utilities for applying brightness filters to an element.

## Brightness

```rust
use tailwind_css::TailwindBuilder;

#[test]
fn build_brightness() {
    let builder = TailwindBuilder::default();
    let out = format!("{:?}", builder.inline("brightness-125"));
    assert_eq!(out, "{filter: brightness(1.25);}")
}
```

## Reference

- [brightness](https://tailwindcss.com/docs/brightness)
- [brightness/cn](https://tailwindcss.cn/docs/brightness)
