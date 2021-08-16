Utilities for improving accessibility with screen readers.

## Example

```rust
use tailwind_css::TailwindBuilder;

#[test]
fn build_screen_reader() {
    let builder = TailwindBuilder::default();
    let out = builder.inline("sr-only");
    assert_eq!(
        out,
        "border-width:0;clip:rect(0,0,0,0);height:1px;margin:-1px;overflow:hidden;padding:0;position:absolute;white-space:nowrap;width:1px;"
    );
    let out = builder.inline("not-sr-only");
    assert_eq!(
        out,
        "clip:auto;height:auto;margin:0;overflow:visible;padding:0;position:static;white-space:normal;width:auto;"
    );
}
```

## Reference

- [screen-readers](https://tailwindcss.com/docs/screen-readers)
- [screen-readers/cn](https://tailwindcss.com/docs/screen-readers)
