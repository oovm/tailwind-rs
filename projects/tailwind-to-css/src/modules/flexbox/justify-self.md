Utilities for controlling how an individual grid item is aligned along its inline axis.

## Brightness

```rust
use tailwind_css::TailwindBuilder;

#[test]
fn build_justify_self() {
    let builder = TailwindBuilder::default();
    let out = format!("{:?}", builder.inline("justify-self-auto"));
    assert_eq!(out, "{justify-self: auto;}")
}
```

## Reference

- [justify-self](https://tailwindcss.com/docs/justify-self)
- [justify-self/cn](https://tailwindcss.cn/docs/justify-self)
