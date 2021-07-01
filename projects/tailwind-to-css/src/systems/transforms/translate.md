Utilities for controlling the aspect ratio of an element.

## Aspect Ratio

```rust
use tailwind_css::{TailwindBuilder};

#[test]
fn build_aspect() {
    let builder = TailwindBuilder::default();
    let out = format!("{:?}", builder.inline("aspect-square"));
    assert_eq!(out, "{aspect-ratio: 1 / 1;}")
}
```

## Reference

- [aspect-ratio](https://tailwindcss.com/docs/aspect-ratio)
- [aspect-ratio/cn](https://tailwindcss.cn/docs/aspect-ratio)
- [tailwindcss-aspect-ratio](https://github.com/tailwindlabs/tailwindcss-aspect-ratio)
