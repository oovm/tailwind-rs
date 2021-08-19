Utilities for controlling how a background image behaves when scrolling.

## Example

```rust
use tailwind_css::TailwindBuilder;

#[test]
fn build_bg_attach() {
    let builder = TailwindBuilder::default();
    let out = builder.inline("bg-scroll");
    assert_eq!(out, "background-attachment:scroll;");
}
```

## Patterns

| Class       | Properties                       |
|:------------|:---------------------------------|
| `bg-fixed`  | `background-attachment: fixed;`  |
| `bg-local`  | `background-attachment: local;`  |
| `bg-scroll` | `background-attachment: scroll;` |

## Reference

- [background-attachment](https://tailwindcss.com/docs/background-attachment)
- [background-attachment/cn](https://tailwindcss.com/docs/background-attachment)
