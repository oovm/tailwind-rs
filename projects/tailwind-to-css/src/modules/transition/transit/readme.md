Utilities for controlling which CSS properties transition.

## Example

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

- [transition-property](https://tailwindcss.com/docs/transition-property)
- [transition-property/cn](https://tailwindcss.cn/docs/transition-property)

