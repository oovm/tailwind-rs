# Tailwind to Css

**Tailwind style tracer, JIT + AOT compiler!**

```rust
use tailwind_css::TailwindBuilder;

fn build() {
    let mut tailwind = TailwindBuilder::default();
    // The compiler will expand directly into the final css property
    // Inline style will not be tracked
    let inline = tailwind.inline("py-2 px-4 bg-green-500");
    // The compiler will expand into a `class`, and record the style class used
    tailwind.trace("py-2 px-4 bg-green-500");
    // Compile all traced classes into bundle
    let bundle = tailwind.bundle();
}
```

## Notice

### Tailwind++ Grammar

This library is not strictly implemented according to the original version.

Especially when some writing methods can be simplified or generalized.

For example arbitrary values of `z-index` needs brackets, but rs version does not.

- js: `z-[100]`
- rs: `z-100`

### Bundle or Inline?

For example, there are style overrides in `p-auto px-px pt-2 pb-2`.

In inline mode, the latter will overwrite the former, and finally get `{}`

In Bundle mode, the final result depends on the browser.

## Implement Progress

**tailwind-rs needs your help!**

A lot of documentation and test cases are missing, you are welcome to pr!

See the `tests` folder for details.

- **Preflight**
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
  - [x] [z-index](https://tailwindcss.com/docs/z-index)
- **Flexbox & Grid**
  - [x] [z-index](https://tailwindcss.com/docs/flex-basis): [`TailWindZIndex`]
- **Spacing**
  - [x] [padding](https://tailwindcss.com/docs/padding): [`TailwindSpacing`]
  - [x] [margin](https://tailwindcss.com/docs/margin): [`TailwindSpacing`]
  - [x] [space](https://tailwindcss.com/docs/space): [`TailwindSpacing`]
- **Sizing**
  - [x] [width](https://tailwindcss.com/docs/width): [`TailwindWidth`]
  - [x] [min-width](https://tailwindcss.com/docs/min-width): [`TailwindWidth`]
  - [x] [max-width](https://tailwindcss.com/docs/max-width): [`TailwindWidth`]
  - [x] [height](https://tailwindcss.com/docs/height): [`TailwindHeight`]
  - [x] [min-height](https://tailwindcss.com/docs/min-height): [`TailwindHeight`]
  - [x] [max-height](https://tailwindcss.com/docs/max-height): [`TailwindHeight`]
- **Typography**
  - [ ] [font-family](https://tailwindcss.com/docs/font-family)
- **Backgrounds**
  - [ ] [background-attachment](https://tailwindcss.com/docs/background-attachment)
- **Borders**
  - [ ] [border-radius](https://tailwindcss.com/docs/background-attachment)
- **Effects**
  - [ ] [box-shadow](https://tailwindcss.com/docs/box-shadow): [`ShadowSystem`]
  - [ ] [box-shadow-color](https://tailwindcss.com/docs/box-shadow-color): [`ShadowSystem`]
  - [ ] [opacity](https://tailwindcss.com/docs/opacity): [`TailwindOpacity`]
  - [ ] [mix-blend-mode](https://tailwindcss.com/docs/mix-blend-mode): [`TailwindBlender`]
  - [ ] [background-blend-mode](https://tailwindcss.com/docs/background-blend-mode): [`TailwindBlender`]
- **Filters**
  - [ ] [blur](https://tailwindcss.com/docs/blur)
- **Tables**
  - [x] [border-collapse](https://tailwindcss.com/docs/border-collapse): [`TailwindBorderCollapse`]
  - [x] [table-layout](https://tailwindcss.com/docs/table-layout): [`TailwindTableLayout`]
- **Transitions & Animation**
  - [ ] [border-collapse](https://tailwindcss.com/docs/border-collapse): [`TailwindTableLayout`]
  - [ ] [table-layout](https://tailwindcss.com/docs/table-layout): [`TailwindTableLayout`]
- **Transforms**
  - [x] [scale](https://tailwindcss.com/docs/scale): [`TailwindScale`]
  - [x] [rotate](https://tailwindcss.com/docs/rotate): [`TailwindRotate`]
  - [ ] [translate](https://tailwindcss.com/docs/translate): [`TailwindTranslate`]
  - [x] [skew](https://tailwindcss.com/docs/skew): [`TailwindSkew`]
  - [ ] [transform-origin](https://tailwindcss.com/docs/transform-origin): [`TailwindOrigin`]








