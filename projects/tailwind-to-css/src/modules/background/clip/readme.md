Utilities for controlling how a background image behaves when scrolling.

## Example

```rust
use tailwind_css::TailwindBuilder;

#[test]
fn build_bg_clip() {
    let builder = TailwindBuilder::default();
    let out = builder.inline("bg-clip-content");
    assert_eq!(out, "background-clip:content-box;");
}
```

## Reference

- [background-clip](https://tailwindcss.com/docs/background-clip)
- [background-clip/cn](https://tailwindcss.com/docs/background-clip)
