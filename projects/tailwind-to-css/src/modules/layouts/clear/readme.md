Utilities for controlling the wrapping of content around an element.

## Examples

```rust
use tailwind_css::TailwindBuilder;

#[test]
fn build_clear() {
    let builder = TailwindBuilder::default();
    let out = builder.inline("clear-none");
    assert_eq!(out, "clear:none;");
}
```

## Reference

- [clear](https://tailwindcss.com/docs/clear)
- [clear/cn](https://tailwindcss.cn/docs/clear)
