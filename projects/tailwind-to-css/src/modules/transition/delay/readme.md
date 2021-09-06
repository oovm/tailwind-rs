Utilities for controlling the delay of CSS transitions.

## Example

```rust
use tailwind_css::TailwindBuilder;

#[test]
fn build_delay() {
    let mut builder = TailwindBuilder::default();
    let out = builder.inline("delay-1000");
    assert_eq!(out, "transition-delay:1000ms;");
}
```

## Reference

- [transition-delay](https://tailwindcss.com/docs/transition-delay)
- [transition-delay/cn](https://tailwindcss.cn/docs/transition-delay)