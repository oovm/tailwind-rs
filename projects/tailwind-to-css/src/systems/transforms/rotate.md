Utilities for rotating elements with transform.

## Example

```rust
use tailwind_css::TailwindBuilder;

#[test]
fn build_rotate() {
    let builder = TailwindBuilder::default();
    let out = format!("{:?}", builder.inline("rotate-0"));
    assert_eq!(out, "{transform: rotate(0deg);}")
}
```

## Reference

- [rotate](https://tailwindcss.com/docs/rotate)
- [rotate/cn](https://tailwindcss.cn/docs/rotate)