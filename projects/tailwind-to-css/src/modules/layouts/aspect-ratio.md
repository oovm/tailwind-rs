Utilities for controlling the aspect ratio of an element.

## Examples

```rust
use tailwind_css::TailwindBuilder;

#[test]
fn build_aspect() {
    let builder = TailwindBuilder::default();
    let out = builder.inline("aspect-video");
    assert_eq!(out, "aspect-ratio:16/9;");
    let out = builder.inline("aspect-16/9");
    assert_eq!(out, "aspect-ratio:16/9;");
    let out = builder.inline("aspect-auto");
    assert_eq!(out, "aspect-ratio:auto;");
}
```

## Reference

- [aspect-ratio](https://tailwindcss.com/docs/aspect-ratio)
- [aspect-ratio/cn](https://tailwindcss.cn/docs/aspect-ratio)
- [tailwindcss-aspect-ratio](https://github.com/tailwindlabs/tailwindcss-aspect-ratio)
