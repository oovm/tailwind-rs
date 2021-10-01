Utilities for controlling how element fragments should be rendered across multiple lines, columns, or pages.

## Examples

```rust
use tailwind_css::TailwindBuilder;

#[test]
#[test]
fn build_box_decoration() {
    let builder = TailwindBuilder::default();
    let out = builder.inline("box-decoration-clone");
    assert_eq!(out, "box-decoration-break:clone;");
    let out = builder.inline("box-decoration-slice");
    assert_eq!(out, "box-decoration-break:slice;");
}
```

## Reference

- [box-decoration-break](https://tailwindcss.com/docs/box-decoration-break)
- [box-decoration-break/cn](https://tailwindcss.cn/docs/box-decoration-break)
