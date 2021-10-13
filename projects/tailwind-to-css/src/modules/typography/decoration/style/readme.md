Utilities for controlling the style of text decorations.

- `decoration-solid`
<p style="text-decoration-line:underline;text-decoration-style:solid;">The quick brown fox jumps over the lazy dog.</p>

- `decoration-double`
<p style="text-decoration-line:underline;text-decoration-style:double;">The quick brown fox jumps over the lazy dog.</p>

- `decoration-dotted`
<p style="text-decoration-line:underline;text-decoration-style:dotted;">The quick brown fox jumps over the lazy dog.</p>

- `decoration-dashed`
<p style="text-decoration-line:underline;text-decoration-style:dashed;">The quick brown fox jumps over the lazy dog.</p>

- `decoration-wavy`
<p style="text-decoration-line:underline;text-decoration-style:wavy;">The quick brown fox jumps over the lazy dog.</p>


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

## Reference

- [font-smoothing](https://tailwindcss.com/docs/font-smoothing)
- [font-smoothing/cn](https://tailwindcss.cn/docs/font-smoothing)
