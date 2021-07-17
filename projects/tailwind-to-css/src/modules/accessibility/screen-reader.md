Utilities for improving accessibility with screen readers.

## Example

```rust
use tailwind_css::TailwindBuilder;

#[test]
fn build_aspect() {
    let builder = TailwindBuilder::default();
    let out = format!("{:?}", builder.inline("sr-only"));
    assert_eq!(out, "{aspect-ratio: 1 / 1;}")
}
```

## Reference

- [screen-readers](https://tailwindcss.com/docs/screen-readers)
- [screen-readers/cn](https://tailwindcss.com/docs/screen-readers)
