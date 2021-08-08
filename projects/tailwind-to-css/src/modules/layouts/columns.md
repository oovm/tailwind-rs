Utilities for controlling the number of columns within an element.

## Examples

```rust
use tailwind_css::TailwindBuilder;

#[test]
fn build_columns() {
    let builder = TailwindBuilder::default();
    let out = builder.inline("columns-12");
    assert_eq!(out, "columns:12;");
    let out = builder.inline("columns-xs");
    assert_eq!(out, "columns:20rem;");
    let out = builder.inline("columns-auto");
    assert_eq!(out, "columns:auto;");
}
```

## Reference

- [columns](https://tailwindcss.com/docs/columns)
- [columns/cn](https://tailwindcss.c/docs/columns)