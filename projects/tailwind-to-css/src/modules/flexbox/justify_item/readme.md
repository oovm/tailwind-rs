Utilities for controlling how grid items are aligned along their inline axis..

## Justify Items

```rust
use tailwind_css::TailwindBuilder;

#[test]
fn build_justify_items() {
    let builder = TailwindBuilder::default();
    let out = format!("{:?}", builder.inline("justify-items-start"));
    assert_eq!(out, "{justify-items: start;}")
}
```

## Reference

- [brightness](https://tailwindcss.com/docs/justify-items)
- [brightness/cn](https://tailwindcss.cn/docs/justify-items)
