Utilities for controlling the wrapping of content around an element.

## Examples

```rust
use tailwind_css::TailwindBuilder;

#[test]
fn build_float() {
    let builder = TailwindBuilder::default();
    let out = builder.inline("float-none");
    assert_eq!(out, "float:none;");
}
```

## Reference

- [float](https://tailwindcss.com/docs/float)
- [float/cn](https://tailwindcss.cn/docs/float)
