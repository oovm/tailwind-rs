A component for fixing an element's width to the current breakpoint.

## Container

```rust
use tailwind_css::TailwindBuilder;

#[test]
fn build_aspect() {
    let builder = TailwindBuilder::default();
    let out = format!("{:?}", builder.inline("container"));
    assert_eq!(out, "{aspect-ratio: 1 / 1;}")
}
```

## Reference

- [container](https://tailwindcss.com/docs/container)
- [container/cn](https://tailwindcss.c/docs/container)