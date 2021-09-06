Utilities for controlling how flex and grid items are positioned along a container's main axis..

## Justify Content

```rust
use tailwind_css::TailwindBuilder;

#[test]
fn build_justify_content() {
    let builder = TailwindBuilder::default();
    let out = format!("{:?}", builder.inline("justify-start"));
    assert_eq!(out, "{justify-content: flex-start;}")
}
```

## Reference

- [justify-content](https://tailwindcss.com/docs/justify-content)
- [justify-content/cn](https://tailwindcss.cn/docs/justify-content)
