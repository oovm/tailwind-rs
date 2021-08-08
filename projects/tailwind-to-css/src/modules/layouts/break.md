Utilities for controlling how a column or page should break an element.

## Examples

```rust
use tailwind_css::TailwindBuilder;

#[test]
fn build_break() {
    let builder = TailwindBuilder::default();
    let out = builder.inline("break-before-auto break-inside-auto break-after-auto");
    assert_eq!(out, "break-after:auto;break-before:auto;break-inside:auto;");
}
```

## Reference

- [break-before](https://tailwindcss.com/docs/break-before)
- [break-before/cn](https://tailwindcss.c/docs/break-before)
- [break-inside](https://tailwindcss.com/docs/break-inside)
- [break-inside/cn](https://tailwindcss.c/docs/break-inside)
- [break-after](https://tailwindcss.com/docs/break-after)
- [break-after/cn](https://tailwindcss.c/docs/break-after)