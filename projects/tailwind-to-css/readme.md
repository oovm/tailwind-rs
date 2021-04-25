# Tailwind to Css



```rust
use tailwind_css::TailwindBuilder;
fn build() {
    let mut tailwind = TailwindBuilder::default();
    // 
    tailwind.trace("py-2 px-4 bg-green-500");
    println!("{}", tailwind.build().unwrap())
}
```

## Implement Progress

- [x] [preflight](https://tailwindcss.com/docs/preflight): [`PreflightSystem`]
- **Layout**
  - [x] [aspect-ratio](https://tailwindcss.com/docs/aspect-ratio): [`TailwindAspect`]
  - [ ] [container](https://tailwindcss.com/docs/container): [``]
  - [ ] [columns](https://tailwindcss.com/docs/columns): [``]
  - [x] [break-after](https://tailwindcss.com/docs/break-after): [`TailwindBreak::After`]
  - [x] [break-before](https://tailwindcss.com/docs/break-before): [`TailwindBreak::Before`]
  - [x] [break-inside](https://tailwindcss.com/docs/break-inside): [`TailwindBreak::Inside`]
  - [ ] [box-decoration-break](https://tailwindcss.com/docs/box-decoration-break)
  - [ ] [box-sizing](https://tailwindcss.com/docs/box-sizing)
  - [ ] [display](https://tailwindcss.com/docs/display)
  - [ ] [float](https://tailwindcss.com/docs/display)
  - [ ] [clear](https://tailwindcss.com/docs/display)
  - [ ] [isolation](https://tailwindcss.com/docs/display)
  - [ ] [object-fit](https://tailwindcss.com/docs/display)
  - [ ] [object-position](https://tailwindcss.com/docs/display)
  - [ ] [overflow](https://tailwindcss.com/docs/display)
  - [ ] [overscroll-behavior](https://tailwindcss.com/docs/display)
  - [ ] [position](https://tailwindcss.com/docs/display)
  - [ ] [top-right-bottom-left](https://tailwindcss.com/docs/display)
  - [ ] [visibility](https://tailwindcss.com/docs/display)
  - [ ] [z-index](https://tailwindcss.com/docs/z-index)
- **Flexbox & Grid**
  - [ ] [z-index](https://tailwindcss.com/docs/flex-basis)
- **Spacing**
  - [x] [padding](https://tailwindcss.com/docs/padding)
  - [x] [margin](https://tailwindcss.com/docs/margin)
  - [x] [space](https://tailwindcss.com/docs/space)
- **Sizing**
  - [ ] [width](https://tailwindcss.com/docs/flex-basis)
- **Typography**
  - [ ] [font-family](https://tailwindcss.com/docs/font-family)
- **Backgrounds**
  - [ ] [background-attachment](https://tailwindcss.com/docs/background-attachment)
- **Borders**
  - [ ] [border-radius](https://tailwindcss.com/docs/background-attachment)
- **Effects**
  - [ ] [box-shadow](https://tailwindcss.com/docs/background-attachment)
- **Filters**
  - [ ] [box-shadow](https://tailwindcss.com/docs/background-attachment)
- **Tables**
  - [ ] [box-shadow](https://tailwindcss.com/docs/background-attachment)












