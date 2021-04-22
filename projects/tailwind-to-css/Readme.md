# Tailwind to Css



```rust
use tailwind_css::TailwindBuilder;
fn build() {
    let mut tailwind = TailwindBuilder::default();
    // 
    tailwind.trace("py-2 px-4 bg-green-500");
    println!(tailwind.build())
}
```

## Implement Progress

- [x] [preflight](https://tailwindcss.com/docs/preflight)
- **Layout**
  - [ ] [aspect-ratio](https://tailwindcss.com/docs/aspect-ratio)
  - [ ] [container](https://tailwindcss.com/docs/container)
  - [ ] [columns](https://tailwindcss.com/docs/columns)
  - [ ] [break-after](https://tailwindcss.com/docs/break-after)
  - [ ] [break-before](https://tailwindcss.com/docs/break-before)
  - [ ] [break-inside](https://tailwindcss.com/docs/break-inside)
