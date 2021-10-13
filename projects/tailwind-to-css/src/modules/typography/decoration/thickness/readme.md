Utilities for controlling the font smoothing of an element.

## Example

```rust
use tailwind_css::TailwindBuilder;

#[test]
fn build_aspect() {
    let builder = crate::TailwindBuilder::default();
    let out = format!("{:?}", builder.inline("aspect-square"));
    assert_eq!(out, "{aspect-ratio: 1 / 1;}")
}
```

<p style="text-decoration-line:underline;text-decoration-thickness:auto;">The quick brown fox jumps over the lazy dog.</p>
<p style="text-decoration-line:underline;text-decoration-thickness:from-font;">The quick brown fox jumps over the lazy dog.</p>
<p style="text-decoration-line:underline;text-decoration-thickness:2px;">The quick brown fox jumps over the lazy dog.</p>

## Reference

- [font-smoothing](https://tailwindcss.com/docs/font-smoothing)
- [font-smoothing/cn](https://tailwindcss.cn/docs/font-smoothing)
