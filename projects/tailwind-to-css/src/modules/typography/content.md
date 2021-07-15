Utilities for controlling the content of the before and after pseudo-elements.

## Example

```rust
use tailwind_css::TailwindBuilder;

#[test]
fn build_content() {
    let builder = TailwindBuilder::default();
    let out = format!("{:?}", builder.inline("content-none"));
    assert_eq!(out, "{content: none;}")
}
```

## Reference

- [content](https://tailwindcss.com/docs/content)
- [content/cn](https://tailwindcss.cn/docs/content)
