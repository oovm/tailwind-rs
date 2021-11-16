# Tailwind CSS

**Tailwind style tracer, JIT + AOT Interpreter!**

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
    - [x] [preflight](https://tailwindcss.com/docs/preflight): [PreflightSystem](https://docs.rs/tailwind-css/latest/tailwind_css/struct.PreflightSystem.html)
- **Layout**
    - [x] [aspect-ratio](https://tailwindcss.com/docs/aspect-ratio): [`TailwindAspect`]
    - [x] [container](https://tailwindcss.com/docs/container): [`TailwindContainer`]
    - [x] [columns](https://tailwindcss.com/docs/columns): [`TailwindColumns`]
    - [x] [break-after](https://tailwindcss.com/docs/break-after): [`TailwindBreakAfter`]
    - [x] [break-before](https://tailwindcss.com/docs/break-before): [`TailwindBreakBefore`]
    - [x] [break-inside](https://tailwindcss.com/docs/break-inside): [`TailwindBreakInside`]
    - [x] [box-decoration-break](https://tailwindcss.com/docs/box-decoration-break): [`TailwindBoxDecoration`]
    - [x] [box-sizing](https://tailwindcss.com/docs/box-sizing): [`TailwindBoxSize`]
    - [x] [display](https://tailwindcss.com/docs/display): [`TailwindDisplay`]
    - [x] [float](https://tailwindcss.com/docs/float): [`TailwindFloat`]
    - [x] [clear](https://tailwindcss.com/docs/clear): [`TailwindClear`]
    - [x] [isolation](https://tailwindcss.com/docs/isolation): [`TailwindIsolation`]
    - [x] [object-fit](https://tailwindcss.com/docs/object-fit): [`TailwindObjectFit`]
    - [x] [object-position](https://tailwindcss.com/docs/object-position): [`TailwindObjectPosition`]
    - [x] [overflow](https://tailwindcss.com/docs/overflow): [`TailwindOverflow`]
    - [x] [overscroll-behavior](https://tailwindcss.com/docs/overscroll-behavior): [`TailwindOverscroll`]
    - [x] [position](https://tailwindcss.com/docs/position): [`TailwindPosition`]
    - [x] [inset](https://tailwindcss.com/docs/top-right-bottom-left): [`TailwindInset`]
    - [x] [left](https://tailwindcss.com/docs/top-right-bottom-left): [`TailwindLeft`]
    - [x] [right](https://tailwindcss.com/docs/top-right-bottom-left): [`TailwindRight`]
    - [x] [top](https://tailwindcss.com/docs/top-right-bottom-left): [`TailwindTop`]
    - [x] [bottom](https://tailwindcss.com/docs/top-right-bottom-left): [`TailwindBottom`]
    - [x] [visibility](https://tailwindcss.com/docs/visibility): [`TailwindVisibility`]
    - [x] [z-index](https://tailwindcss.com/docs/z-index): [`TailwindZIndex`]
- **Flexbox & Grid**
    - [x] [flex-basis](https://tailwindcss.com/docs/flex-basis): [`TailWindZIndex`]
    - [x] [flex-direction](https://tailwindcss.com/docs/flex-direction): [`TailWindZIndex`]
    - [x] [flex-wrap](https://tailwindcss.com/docs/flex-wrap): [`TailWindZIndex`]
    - [x] [flex](https://tailwindcss.com/docs/flex): [`TailWindZIndex`]
    - [x] [flex-grow](https://tailwindcss.com/docs/flex-grow): [`TailWindZIndex`]
    - [x] [flex-shrink](https://tailwindcss.com/docs/flex-shrink): [`TailWindZIndex`]
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
    - [x] [font-family](https://tailwindcss.com/docs/font-family)
- **Backgrounds**
    - [x] [background-attachment](https://tailwindcss.com/docs/background-attachment)
- **Borders**
    - [x] [border-radius](https://tailwindcss.com/docs/background-attachment)
- **Effects**
    - [x] [box-shadow](https://tailwindcss.com/docs/box-shadow): [`ShadowSystem`]
    - [x] [box-shadow-color](https://tailwindcss.com/docs/box-shadow-color): [`ShadowSystem`]
    - [x] [opacity](https://tailwindcss.com/docs/opacity): [`TailwindOpacity`]
    - [x] [mix-blend-mode](https://tailwindcss.com/docs/mix-blend-mode): [`TailwindBlender`]
    - [x] [background-blend-mode](https://tailwindcss.com/docs/background-blend-mode): [`TailwindBlender`]
- **Filters**
    - [x] [blur](https://tailwindcss.com/docs/blur)
- **Tables**
    - [x] [border-collapse](https://tailwindcss.com/docs/border-collapse): [`TailwindBorderCollapse`]
    - [x] [table-layout](https://tailwindcss.com/docs/table-layout): [`TailwindTableLayout`]
- **Transitions & Animation**
    - [x] [border-collapse](https://tailwindcss.com/docs/border-collapse): [`TailwindTableLayout`]
    - [x] [table-layout](https://tailwindcss.com/docs/table-layout): [`TailwindTableLayout`]
- **Transforms**
    - [x] [scale](https://tailwindcss.com/docs/scale): [`TailwindScale`]
    - [x] [rotate](https://tailwindcss.com/docs/rotate): [`TailwindRotate`]
    - [x] [translate](https://tailwindcss.com/docs/translate): [`TailwindTranslate`]
    - [x] [skew](https://tailwindcss.com/docs/skew): [`TailwindSkew`]
    - [x] [transform-origin](https://tailwindcss.com/docs/transform-origin): [`TailwindOrigin`]
