Utilities for controlling the duration of CSS transitions.

## Example

```rust
use tailwind_css::TailwindBuilder;

#[test]
fn build_duration() {
    let mut builder = TailwindBuilder::default();
    let out = builder.inline("duration-1000");
    assert_eq!(out, "transition-duration:1000ms;");
}
```

## Reference

- [transition-duration](https://tailwindcss.com/docs/transition-duration)
- [transition-duration/cn](https://tailwindcss.cn/docs/transition-duration)