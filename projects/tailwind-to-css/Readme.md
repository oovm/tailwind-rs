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
    tailwind.trace("py-2 px-4 bg-green-500", false);
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

In inline mode, the latter will overwrite the former, and finally get `padding:.5rem 1px`

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
    - [x] [flex-basis](https://tailwindcss.com/docs/flex-basis): [`TailwindBasis`]
    - [x] [flex-direction](https://tailwindcss.com/docs/flex-direction): [`TailwindFlexDirection`]
    - [x] [flex-wrap](https://tailwindcss.com/docs/flex-wrap): [`TailwindFlexWrap`]
    - [x] [flex](https://tailwindcss.com/docs/flex): [`TailwindFlex`]
    - [x] [flex-grow](https://tailwindcss.com/docs/flex-grow): [`TailWindGrow`]
    - [x] [flex-shrink](https://tailwindcss.com/docs/flex-shrink): [`TailWindShrink`]
    - [x] [order](https://tailwindcss.com/docs/order): [`TailWindOrder`]
    - [x] [grid-template-columns](https://tailwindcss.com/docs/grid-template-columns): [`TailwindGridColumns`]
    - [x] [grid-template-rows](https://tailwindcss.com/docs/grid-template-rows): [`TailwindGridRows`]
    - [x] [grid-column](https://tailwindcss.com/docs/grid-column): [`TailwindColumn`]
    - [x] [grid-row](https://tailwindcss.com/docs/grid-row): [`TailwindRow`]
    - [x] [grid-auto-flow](https://tailwindcss.com/docs/grid-auto-flow): [`TailwindGridFlow`]
    - [x] [grid-auto-columns](https://tailwindcss.com/docs/grid-auto-columns): [`TailwindGridAuto`]
    - [x] [grid-auto-rows](https://tailwindcss.com/docs/grid-auto-rows): [`TailwindGridAuto`]
    - [x] [gap](https://tailwindcss.com/docs/gap): [`TailwindGap`]
    - [x] [justify-content](https://tailwindcss.com/docs/justify-content): [`TailwindJustifyContent`]
    - [x] [justify-items](https://tailwindcss.com/docs/justify-items): [`TailwindJustifyItems`]
    - [x] [justify-self](https://tailwindcss.com/docs/justify-self): [`TailwindJustifySelf`]
    - [x] [justify-content](https://tailwindcss.com/docs/align-content): [`TailwindContent`]
    - [x] [align-items](https://tailwindcss.com/docs/align-items): [`TailwindContent`]
    - [x] [align-self](https://tailwindcss.com/docs/align-self): [`TailwindItems`]
    - [x] [place-content](https://tailwindcss.com/docs/place-content): [`TailwindPlaceContent`]
    - [x] [place-items](https://tailwindcss.com/docs/place-items): [`TailwindPlaceItems`]
    - [x] [place-self](https://tailwindcss.com/docs/place-self): [`TailwindPlaceSelf`]
- **Spacing**
    - [x] [padding](https://tailwindcss.com/docs/padding): [`TailwindPadding`]
    - [x] [margin](https://tailwindcss.com/docs/margin): [`TailwindMargin`]
    - [x] [space](https://tailwindcss.com/docs/space): [`TailwindSpace`]
- **Sizing**
    - [x] [width](https://tailwindcss.com/docs/width): [`TailwindSizing`]
    - [x] [min-width](https://tailwindcss.com/docs/min-width): [`TailwindSizing`]
    - [x] [max-width](https://tailwindcss.com/docs/max-width): [`TailwindSizing`]
    - [x] [height](https://tailwindcss.com/docs/height): [`TailwindSizing`]
    - [x] [min-height](https://tailwindcss.com/docs/min-height): [`TailwindSizing`]
    - [x] [max-height](https://tailwindcss.com/docs/max-height): [`TailwindSizing`]
- **Typography**
    - [x] [font-family](https://tailwindcss.com/docs/font-family): [`TailwindFontFamily`]
    - [x] [font-size](https://tailwindcss.com/docs/font-size): [`TailwindFontSize`]
    - [x] [font-smoothing](https://tailwindcss.com/docs/font-smoothing): [`FontSmoothing`]
    - [x] [font-style](https://tailwindcss.com/docs/font-style): [`TailwindFontStyle`]
    - [x] [font-weight](https://tailwindcss.com/docs/font-weight): [`TailwindFontWeight`]
    - [x] [font-variant-numeric](https://tailwindcss.com/docs/font-variant-numeric): [`TailwindFontVariantNumeric`]
    - [x] [letter-spacing](https://tailwindcss.com/docs/letter-spacing): [`TailwindTracking`]
    - [x] [line-height](https://tailwindcss.com/docs/line-height): [`TailwindLeading`]
    - [x] [list-style-type](https://tailwindcss.com/docs/list-style-type): [`TailwindListStyle`]
    - [x] [list-style-position](https://tailwindcss.com/docs/list-style-position): [`TailwindListPosition`]
    - [x] [text-align](https://tailwindcss.com/docs/text-align): [`TailwindTextAlignment`]
    - [x] [text-color](https://tailwindcss.com/docs/text-color): [`TailwindTextColor`]
    - [x] [text-decoration](https://tailwindcss.com/docs/text-decoration): [`TailwindDecoration`]
    - [x] [text-decoration-color](https://tailwindcss.com/docs/text-decoration-color): [`TailwindDecorationColor`]
    - [x] [text-decoration-style](https://tailwindcss.com/docs/text-decoration-style): [`TailwindDecorationThickness`]
    - [x] [text-decoration-thickness](https://tailwindcss.com/docs/text-decoration-thickness): [`TailwindSizing`]
    - [x] [text-underline-offset](https://tailwindcss.com/docs/text-underline-offset): [`TailwindUnderlineOffset`]
    - [x] [text-transform](https://tailwindcss.com/docs/text-transform): [`TailwindTextTransform`]
    - [x] [text-overflow](https://tailwindcss.com/docs/text-overflow): [`TailwindOverflow`]
    - [x] [text-indent](https://tailwindcss.com/docs/text-indent): [`TailwindIndent`]
    - [x] [vertical-align](https://tailwindcss.com/docs/vertical-align): [`TailwindAlign`]
    - [x] [whitespace](https://tailwindcss.com/docs/whitespace): [`TailwindWhiteSpace`]
    - [x] [word-break](https://tailwindcss.com/docs/word-break): [`TailwindBreak`]
    - [x] [content](https://tailwindcss.com/docs/content): [`TailwindContent`]
- **Backgrounds**
    - [x] [background-attachment](https://tailwindcss.com/docs/background-attachment): [`TailwindBackgroundAttachment`]
    - [x] [background-clip](https://tailwindcss.com/docs/background-clip): [`TailwindBackgroundClip`]
    - [x] [background-color](https://tailwindcss.com/docs/background-color): [`TailwindBackgroundColor`]
    - [x] [background-origin](https://tailwindcss.com/docs/background-origin): [`TailwindBackgroundOrigin`]
    - [x] [background-position](https://tailwindcss.com/docs/background-position): [`TailwindBackgroundPosition`]
    - [x] [background-repeat](https://tailwindcss.com/docs/background-repeat): [`TailwindBackgroundRepeat`]
    - [x] [background-size](https://tailwindcss.com/docs/background-size): [`TailwindBackgroundSize`]
    - [x] [background-image](https://tailwindcss.com/docs/background-image): [`TailwindBackgroundImage`]
    - [x] [background-from](https://tailwindcss.com/docs/gradient-color-stops): [`TailwindFrom`]
    - [x] [background-via](https://tailwindcss.com/docs/gradient-color-stops): [`TailwindVia`]
    - [x] [background-to](https://tailwindcss.com/docs/gradient-color-stops): [`TailwindTo`]
- **Borders**
    - [x] [border-radius](https://tailwindcss.com/docs/border-radius): [`TailwindRounded`]
    - [x] [border-width](https://tailwindcss.com/docs/border-width): [`TailwindBorderWidth`]
    - [x] [border-color](https://tailwindcss.com/docs/border-color): [`TailwindBorderColor`]
    - [x] [border-style](https://tailwindcss.com/docs/border-style): [`TailwindBorderStyle`]
    - [x] [divide-width](https://tailwindcss.com/docs/divide-width): [`TailwindDivideWidth`]
    - [x] [divide-color](https://tailwindcss.com/docs/divide-color): [`TailwindDivideColor`]
    - [x] [divide-style](https://tailwindcss.com/docs/divide-style): [`TailwindDivideStyle`]
    - [x] [outline-width](https://tailwindcss.com/docs/outline-width): [`TailwindOutlineWidth`]
    - [x] [outline-color](https://tailwindcss.com/docs/outline-color): [`TailwindOutlineColor`]
    - [x] [outline-style](https://tailwindcss.com/docs/outline-style): [`TailwindOutlineStyle`]
    - [x] [ring-width](https://tailwindcss.com/docs/ring-width): [`TailwindRingWidth`]
    - [x] [ring-color](https://tailwindcss.com/docs/ring-color): [`TailwindRingColor`]
    - [x] [ring-offset-width](https://tailwindcss.com/docs/ring-offset-width): [`TailwindRingOffsetWidth`]
    - [x] [ring-offset-width](https://tailwindcss.com/docs/ring-offset-width): [`TailwindRingOffsetColor`]
- **Effects**
    - [x] [box-shadow](https://tailwindcss.com/docs/box-shadow): [`TailwindShadow`]
    - [x] [box-shadow-color](https://tailwindcss.com/docs/box-shadow-color): [`TailwindShadowColor`]
    - [x] [opacity](https://tailwindcss.com/docs/opacity): [`TailwindOpacity`]
    - [x] [mix-blend-mode](https://tailwindcss.com/docs/mix-blend-mode): [`TailwindBlend`]
    - [x] [background-blend-mode](https://tailwindcss.com/docs/background-blend-mode): [`TailwindBackgroundBlend`]
- **Filters**
    - [x] [blur](https://tailwindcss.com/docs/blur): [`TailwindBlur`]
    - [x] [brightness](https://tailwindcss.com/docs/brightness): [`TailwindBrightness`]
    - [x] [contrast](https://tailwindcss.com/docs/contrast): [`TailwindContrast`]
    - [x] [drop-shadow](https://tailwindcss.com/docs/drop-shadow): [`TailwindShadow`]
    - [x] [grayscale](https://tailwindcss.com/docs/grayscale): [`TailwindGrayscale`]
    - [x] [hue-rotate](https://tailwindcss.com/docs/hue-rotate): [`TailwindHueRotate`]
    - [x] [invert](https://tailwindcss.com/docs/invert): [`TailwindInvert`]
    - [x] [saturate](https://tailwindcss.com/docs/saturate): [`TailwindSaturate`]
    - [x] [sepia](https://tailwindcss.com/docs/sepia): [`TailwindSepia`]
    - [x] [backdrop-blur](https://tailwindcss.com/docs/backdrop-blur): [`TailwindBlur`]
    - [x] [backdrop-brightness](https://tailwindcss.com/docs/backdrop-brightness): [`TailwindBrightness`]
    - [x] [backdrop-contrast](https://tailwindcss.com/docs/backdrop-contrast): [`TailwindContrast`]
    - [x] [backdrop-grayscale](https://tailwindcss.com/docs/backdrop-grayscale): [`TailwindGrayscale`]
    - [x] [backdrop-hue-rotate](https://tailwindcss.com/docs/backdrop-hue-rotate): [`TailwindHueRotate`]
    - [x] [backdrop-invert](https://tailwindcss.com/docs/backdrop-invert): [`TailwindInvert`]
    - [x] [backdrop-opacity](https://tailwindcss.com/docs/backdrop-opacity): [`TailwindOpacity`]
    - [x] [backdrop-saturate](https://tailwindcss.com/docs/backdrop-saturate): [`TailwindSaturate`]
    - [x] [backdrop-sepia](https://tailwindcss.com/docs/backdrop-sepia): [`TailwindSepia`]
- **Tables**
    - [x] [border-collapse](https://tailwindcss.com/docs/border-collapse): [`TailwindBorderCollapse`]
    - [x] [table-layout](https://tailwindcss.com/docs/table-layout): [`TailwindTableLayout`]
- **Transitions & Animation**
    - [x] [transition-property](https://tailwindcss.com/docs/transition-property): [`TailwindTransition`]
    - [x] [transition-duration](https://tailwindcss.com/docs/transition-duration): [`TailwindDuration`]
    - [x] [transition-timing-function](https://tailwindcss.com/docs/transition-timing-function): [`TailwindEase`]
    - [x] [transition-delay](https://tailwindcss.com/docs/transition-delay): [`TailwindDelay`]
    - [x] [animation](https://tailwindcss.com/docs/animation): [`TailwindAnimate`]
- **Transforms**
    - [x] [scale](https://tailwindcss.com/docs/scale): [`TailwindScale`]
    - [x] [rotate](https://tailwindcss.com/docs/rotate): [`TailwindRotate`]
    - [x] [translate](https://tailwindcss.com/docs/translate): [`TailwindTranslate`]
    - [x] [skew](https://tailwindcss.com/docs/skew): [`TailwindSkew`]
    - [x] [transform-origin](https://tailwindcss.com/docs/transform-origin): [`TailwindOrigin`]
- **Interactivity**
    - [x] [accent-color](https://tailwindcss.com/docs/accent-color): [`TailwindAccentColor`]
    - [x] [appearance](https://tailwindcss.com/docs/appearance): [`TailwindAppearance`]
    - [x] [cursor](https://tailwindcss.com/docs/cursor): [`TailwindCursor`]
    - [x] [caret-color](https://tailwindcss.com/docs/caret-color): [`TailwindCaretColor`]
    - [x] [pointer-events](https://tailwindcss.com/docs/pointer-events): [`TailwindPointerEvents`]
    - [x] [resize](https://tailwindcss.com/docs/resize): [`TailwindResize`]
    - [x] [scroll-behavior](https://tailwindcss.com/docs/scroll-behavior): [`TailwindOverscroll`]
    - [x] [scroll-margin](https://tailwindcss.com/docs/scroll-margin): [`TailwindScrollMargin`]
    - [x] [scroll-padding](https://tailwindcss.com/docs/scroll-padding): [`TailwindScrollPadding`]
    - [x] [scroll-snap-align](https://tailwindcss.com/docs/scroll-snap-align): [`TailwindSnapAlign`]
    - [x] [scroll-snap-stop](https://tailwindcss.com/docs/scroll-snap-stop): [`TailwindSnapStop`]
    - [x] [scroll-snap-type](https://tailwindcss.com/docs/scroll-snap-type): [`TailwindSnapType`]
    - [x] [touch-action](https://tailwindcss.com/docs/touch-action): [`TailwindTorch`]
    - [x] [user-select](https://tailwindcss.com/docs/user-select): [`TailwindSelect`]
    - [x] [will-change](https://tailwindcss.com/docs/will-change): [`TailwindWillChange`]
- **SVG**
    - [x] [fill](https://tailwindcss.com/docs/fill): [`TailwindFillColor`]
    - [x] [stroke](https://tailwindcss.com/docs/stroke): [`TailwindStrokeColor`]
    - [x] [stroke-width](https://tailwindcss.com/docs/stroke-width): [`TailwindStrokeWidth`]
- **Accessibility**
    - [x] [screen-readers](https://tailwindcss.com/docs/screen-readers): [`TailwindScreenReader`]
