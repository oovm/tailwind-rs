use super::*;

// noinspection SpellCheckingInspection
impl TailwindInstruction {
    #[inline(never)]
    pub fn get_instance(&self) -> Result<Box<dyn TailwindInstance>> {
        let element = self.view_elements();
        let pattern = element.as_slice();
        let arbitrary = self.view_arbitrary();
        let neg = self.negative;
        let instance = match pattern {
            // Layout System
            ["aspect", rest @ ..] => TailwindAspect::parse(rest, arbitrary)?.boxed(),
            ["container"] => TailwindContainer::default().boxed(),
            ["columns", rest @ ..] => TailwindColumns::parse(rest, arbitrary)?.boxed(),
            ["break", rest @ ..] => TailwindBreak::parse(rest, arbitrary)?,
            ["box", rest @ ..] => Self::box_adaptor(rest, arbitrary)?,
            // begin https://tailwindcss.com/docs/display
            ["block"] => TailwindDisplay::from("block").boxed(),
            ["inline", "block"] => TailwindDisplay::from("inline-block").boxed(),
            ["inline"] => TailwindDisplay::from("inline").boxed(),
            ["flex"] => TailwindDisplay::from("flex").boxed(),
            ["inline", "flex"] => TailwindDisplay::from("inline-flex").boxed(),
            ["table"] => TailwindDisplay::from("table").boxed(),
            ["inline", "table"] => TailwindDisplay::from("inline-table").boxed(),
            ["table", "caption"] => TailwindDisplay::from("table-caption").boxed(),
            ["table", "cell"] => TailwindDisplay::from("table-cell").boxed(),
            ["table", "column"] => TailwindDisplay::from("table-column").boxed(),
            ["table", "column", "group"] => TailwindDisplay::from("table-column-group").boxed(),
            ["table", "footer", "group"] => TailwindDisplay::from("table-footer-group").boxed(),
            ["table", "header", "group"] => TailwindDisplay::from("table-header-group").boxed(),
            ["table", "row", "group"] => TailwindDisplay::from("table-row-group").boxed(),
            ["table", "row"] => TailwindDisplay::from("table-row").boxed(),
            ["flow", "root"] => TailwindDisplay::from("flow-root").boxed(),
            ["grid"] => TailwindDisplay::from("grid").boxed(),
            ["inline", "grid"] => TailwindDisplay::from("inline-grid").boxed(),
            ["contents"] => TailwindDisplay::from("contents").boxed(),
            ["list", "item"] => TailwindDisplay::from("inline-grid").boxed(),
            ["hidden"] => TailwindDisplay::from("hidden").boxed(),
            // https://tailwindcss.com/docs/float
            ["float", rest @ ..] => TailwindFloat::parse(rest, arbitrary)?.boxed(),
            ["clear", rest @ ..] => TailwindClear::parse(rest, arbitrary)?.boxed(),
            ["isolate"] => TailwindIsolation::from("isolate").boxed(),
            ["isolation", rest @ ..] => TailwindIsolation::parse(rest, arbitrary)?.boxed(),
            ["object", rest @ ..] => Self::object_adaptor(rest, arbitrary)?,
            ["overflow", rest @ ..] => Self::overflow_adaptor(rest, arbitrary)?,
            ["overscroll", rest @ ..] => Self::overscroll_adaptor(rest, arbitrary)?,
            // https://tailwindcss.com/docs/position#header
            [s @ ("static" | "fixed" | "absolute" | "relative" | "sticky")] => TailwindPosition::from(*s).boxed(),
            // https://tailwindcss.com/docs/top-right-bottom-left
            ["inset", rest @ ..] => TailwindInset::parse(rest, arbitrary, neg)?.boxed(),
            ["top", rest @ ..] => TailwindTop::parse(rest, arbitrary, neg)?.boxed(),
            ["right", rest @ ..] => TailwindRight::parse(rest, arbitrary, neg)?.boxed(),
            ["bottom", rest @ ..] => TailwindBottom::parse(rest, arbitrary, neg)?.boxed(),
            ["left", rest @ ..] => TailwindLeft::parse(rest, arbitrary, neg)?.boxed(),
            // https://tailwindcss.com/docs/visibility
            ["visible"] => TailwindVisibility::from("visible").boxed(),
            ["invisible"] => TailwindVisibility::from("hidden").boxed(),
            // https://tailwindcss.com/docs/z-index
            ["z", rest @ ..] => TailWindZIndex::parse(rest, arbitrary, self.negative)?.boxed(),
            // Flexbox & Grid
            ["basis", rest @ ..] => TailwindBasis::parse(rest, arbitrary)?.boxed(),
            ["flex", rest @ ..] => flex_adaptor(rest, arbitrary)?,
            ["grow", rest @ ..] => TailWindGrow::parse(rest, arbitrary)?.boxed(),
            ["shrink", rest @ ..] => TailWindShrink::parse(rest, arbitrary)?.boxed(),
            ["order", rest @ ..] => TailWindOrder::parse(rest, arbitrary, self.negative)?.boxed(),
            ["grid", rest @ ..] => TailwindGrid::parse(rest, arbitrary)?,
            // https://tailwindcss.com/docs/grid-column
            ["col", rest @ ..] => TailwindColumn::parse(rest, arbitrary)?.boxed(),
            ["row", rest @ ..] => TailwindRow::parse(rest, arbitrary)?.boxed(),
            ["auto", rest @ ..] => TailwindGridAuto::parse(rest, arbitrary)?.boxed(),
            ["gap", rest @ ..] => TailwindGap::parse(rest, arbitrary)?.boxed(),
            ["justify", rest @ ..] => Self::justify_adaptor(rest, arbitrary)?,
            ["content", rest @ ..] => TailwindContent::parse(rest, arbitrary)?,
            ["items", rest @ ..] => TailwindItems::parse(rest, arbitrary)?.boxed(),
            ["self", rest @ ..] => TailwindSelf::parse(rest, arbitrary)?.boxed(),
            ["place", rest @ ..] => TailwindPlace::parse(rest, arbitrary)?,
            // justify catched
            // Spacing System
            ["p" | "pl" | "pr" | "pm" | "pt" | "px" | "py", ..] => TailwindPadding::parse(pattern, arbitrary, neg)?.boxed(),
            ["m" | "ml" | "mr" | "mm" | "mt" | "mx" | "my", ..] => TailwindMargin::parse(pattern, arbitrary, neg)?.boxed(),
            ["space", rest @ ..] => TailwindSpace::parse(rest, arbitrary, neg)?,
            // Sizing System
            ["w", rest @ ..] => TailwindSizing::parse_width(rest, arbitrary)?.boxed(),
            ["min", "w", rest @ ..] => TailwindSizing::parse_width_min(rest, arbitrary)?.boxed(),
            ["max", "w", rest @ ..] => TailwindSizing::parse_width_max(rest, arbitrary)?.boxed(),
            ["h", rest @ ..] => TailwindSizing::parse_width(rest, arbitrary)?.boxed(),
            ["min", "h", rest @ ..] => TailwindSizing::parse_width_min(rest, arbitrary)?.boxed(),
            ["max", "h", rest @ ..] => TailwindSizing::parse_width_max(rest, arbitrary)?.boxed(),
            // Typography System
            ["font", rest @ ..] => font_adaptor(rest, arbitrary)?,
            ["text", rest @ ..] => text_adaptor(rest, arbitrary)?,
            // begin https://tailwindcss.com/docs/font-variant-numeric
            ["antialiased"] => TailwindFontSmoothing::from("todo").boxed(),
            ["subpixel", "antialiased"] => TailwindFontSmoothing::from("todo").boxed(),
            ["italic"] => TailwindFontStyle::from("italic").boxed(),
            ["not", "italic"] => TailwindFontStyle::from("normal").boxed(),
            // https://tailwindcss.com/docs/font-variant-numeric
            ["normal", "nums"] => TailwindFontVariantNumeric::from("normal").boxed(),
            ["ordinal"] => TailwindFontVariantNumeric::from("ordinal").boxed(),
            ["slashed", "zero"] => TailwindFontVariantNumeric::from("slashed-zero").boxed(),
            ["lining", "nums"] => TailwindFontVariantNumeric::from("lining-nums").boxed(),
            ["oldstyle", "nums"] => TailwindFontVariantNumeric::from("oldstyle-nums").boxed(),
            ["proportional", "nums"] => TailwindFontVariantNumeric::from("proportional-nums").boxed(),
            ["tabular", "nums"] => TailwindFontVariantNumeric::from("tabular-nums").boxed(),
            ["diagonal", "fractions"] => TailwindFontVariantNumeric::from("diagonal-fractions").boxed(),
            ["stacked", "fractions"] => TailwindFontVariantNumeric::from("stacked-fractions").boxed(),
            // https://tailwindcss.com/docs/letter-spacing
            ["tracking", rest @ ..] => TailwindTracking::parse(rest, arbitrary)?.boxed(),
            ["leading", rest @ ..] => TailwindLeading::parse(rest, arbitrary)?.boxed(),
            ["list", rest @ ..] => TailwindList::parse(rest, arbitrary)?,
            // https://tailwindcss.com/docs/text-decoration
            ["underline"] => TailwindDecorationLine::from("underline").boxed(),
            ["overline"] => TailwindDecorationLine::from("overline").boxed(),
            ["line", "through"] => TailwindDecorationLine::from("line-through").boxed(),
            ["no", "underline"] => TailwindDecorationLine::from("none").boxed(),
            // https://tailwindcss.com/docs/text-decoration-color
            ["decoration", rest @ ..] => TailwindDecoration::parse(rest, arbitrary)?,
            ["underline", "offset", rest @ ..] => TailwindUnderlineOffset::parse(rest, arbitrary)?.boxed(),
            // https://tailwindcss.com/docs/text-transform
            ["uppercase"] => TailwindTextTransform::from("uppercase").boxed(),
            ["lowercase"] => TailwindTextTransform::from("lowercase").boxed(),
            ["capitalize"] => TailwindTextTransform::from("capitalize").boxed(),
            ["normal", "case"] => TailwindTextTransform::from("none").boxed(),
            // https://tailwindcss.com/docs/text-overflow
            ["truncate"] => TailwindTextOverflow::Truncate.boxed(),
            ["indent", rest @ ..] => TailwindIndent::parse(rest, arbitrary)?.boxed(),
            ["align", rest @ ..] => TailwindAlign::parse(rest, arbitrary)?.boxed(),
            ["whitespace", rest @ ..] => TailwindWhiteSpace::parse(rest, arbitrary)?.boxed(),
            // break catched
            // content catched
            // Typography System Extension
            ["prose"] => todo!(),
            // Backgrounds System
            ["bg", rest @ ..] => Self::bg_adaptor(rest, arbitrary)?,
            ["from", rest @ ..] => TailwindFrom::parse(rest, arbitrary)?.boxed(),
            ["via", rest @ ..] => TailwindVia::parse(rest, arbitrary)?.boxed(),
            ["to", rest @ ..] => TailwindTo::parse(rest, arbitrary)?.boxed(),
            // Borders System
            ["rounded", rest @ ..] => TailwindRounded::parse(rest, arbitrary)?.boxed(),
            ["border", rest @ ..] => TailwindBorder::parse(rest, arbitrary)?,
            ["divide", rest @ ..] => Self::divide_adaptor(rest, arbitrary)?,
            ["outline", rest @ ..] => outline_adaptor(rest, arbitrary)?,
            ["ring", rest @ ..] => Self::ring_adaptor(rest, arbitrary)?,
            // Effects System
            ["shadow", rest @ ..] => Self::shadow_adaptor(rest, arbitrary)?,
            ["opacity", rest @ ..] => TailwindOpacity::parse(rest, arbitrary, false)?.boxed(),
            ["mix", "blend", rest @ ..] => TailwindBlend::parse(rest, arbitrary)?.boxed(),
            // Filters System
            ["blur", rest @ ..] => TailwindBlur::parse(rest, arbitrary, false)?.boxed(),
            ["brightness", rest @ ..] => TailwindBrightness::parse(rest, arbitrary, false)?.boxed(),
            ["contrast", rest @ ..] => TailwindContrast::parse(rest, arbitrary, false)?.boxed(),
            ["drop", "shadow", rest @ ..] => TailwindShadow::parse(rest, arbitrary, true)?.boxed(),
            ["grayscale", rest @ ..] => TailwindGrayscale::parse(rest, arbitrary, false)?.boxed(),
            ["hue", "rotate", rest @ ..] => TailwindHueRotate::parse(rest, arbitrary, false)?.boxed(),
            ["invert", rest @ ..] => TailwindInvert::parse(rest, arbitrary, false)?.boxed(),
            ["saturate", rest @ ..] => TailwindSaturate::parse(rest, arbitrary, false)?.boxed(),
            ["sepia", rest @ ..] => TailwindSepia::parse(rest, arbitrary, false)?.boxed(),
            ["backdrop", rest @ ..] => Self::backdrop_adaptor(rest, arbitrary)?,
            // Tables System
            ["table", rest @ ..] => Self::table_adaptor(rest, arbitrary)?,
            // Transitions System
            ["transition", rest @ ..] => TailwindTransition::parse(rest, arbitrary)?.boxed(),
            ["duration", rest @ ..] => TailwindDuration::parse(rest, arbitrary)?.boxed(),
            ["ease", rest @ ..] => TailwindEase::parse(rest, arbitrary)?.boxed(),
            ["delay", rest @ ..] => TailwindDelay::parse(rest, arbitrary)?.boxed(),
            ["animate", rest @ ..] => TailwindAnimate::parse(rest, arbitrary)?.boxed(),
            // Transforms System
            ["scale", rest @ ..] => TailwindScale::parse(rest, arbitrary, neg)?.boxed(),
            ["rotate", rest @ ..] => TailwindRotate::parse(rest, arbitrary, neg)?.boxed(),
            ["translate", rest @ ..] => TailwindTranslate::parse(rest, arbitrary, neg)?.boxed(),
            ["skew", rest @ ..] => TailwindSkew::parse(rest, arbitrary, neg)?.boxed(),
            ["origin", rest @ ..] => TailwindOrigin::parse(rest, arbitrary)?.boxed(),
            // Interactivity System
            ["accent", rest @ ..] => TailwindAccentColor::parse(rest, arbitrary)?.boxed(),
            // https://tailwindcss.com/docs/appearance
            ["appearance", rest @ ..] => TailwindAppearance::parse(rest, arbitrary)?.boxed(),
            ["cursor", rest @ ..] => TailwindCursor::parse(rest, arbitrary)?.boxed(),
            ["caret", rest @ ..] => TailwindCaretColor::parse(rest, arbitrary)?.boxed(),
            ["pointer", "events", rest @ ..] => TailwindPointerEvents::parse(rest, arbitrary)?.boxed(),
            ["resize", rest @ ..] => TailwindResize::parse(rest, arbitrary)?.boxed(),
            ["scroll", rest @ ..] => TailwindScroll::parse(rest, arbitrary, neg)?,
            ["snap", rest @ ..] => TailwindSnap::parse(rest, arbitrary)?,
            ["touch", rest @ ..] => TailwindTorch::parse(rest, arbitrary)?.boxed(),
            ["select", rest @ ..] => TailwindSelect::parse(rest, arbitrary)?.boxed(),
            ["will", "change", rest @ ..] => TailwindWillChange::parse(rest, arbitrary)?.boxed(),
            // SVG System
            ["fill", rest @ ..] => TailwindFillColor::parse(rest, arbitrary)?.boxed(),
            ["stroke", rest @ ..] => TailwindStroke::parse(rest, arbitrary)?,
            // Accessibility System
            ["sr", "only"] => TailwindScreenReader::new(true).boxed(),
            ["not", "sr", "only"] => TailwindScreenReader::new(false).boxed(),
            // Form System Extension
            _ => return syntax_error!("Unknown instructions: {} + [{}]", element.join("-"), arbitrary),
        };
        Ok(instance)
    }
    #[inline]
    fn bg_adaptor(str: &[&str], arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
        let out = match str {
            // https://tailwindcss.com/docs/background-attachment
            ["fixed"] => TailwindBackgroundAttachment::Fixed.boxed(),
            ["local"] => TailwindBackgroundAttachment::Local.boxed(),
            ["scroll"] => TailwindBackgroundAttachment::Scroll.boxed(),
            // https://tailwindcss.com/docs/background-clip
            ["clip", "border"] => TailwindBackgroundClip::from("border-box").boxed(),
            ["clip", "padding"] => TailwindBackgroundClip::from("padding-box").boxed(),
            ["clip", "content"] => TailwindBackgroundClip::from("content-box").boxed(),
            ["clip", "text"] => TailwindBackgroundClip::from("text").boxed(),
            // https://tailwindcss.com/docs/background-origin
            ["origin", rest @ ..] => TailwindBackgroundOrigin::parse(rest, arbitrary)?.boxed(),
            // https://tailwindcss.com/docs/background-repeat
            ["no", "repeat"] => TailwindBackgroundRepeat::from("repeat").boxed(),
            ["repeat", rest @ ..] => TailwindBackgroundRepeat::parse(rest, arbitrary)?.boxed(),
            // https://tailwindcss.com/docs/background-size
            ["auto"] => TailwindBackgroundSize::Auto.boxed(),
            ["cover"] => TailwindBackgroundSize::Cover.boxed(),
            ["contain"] => TailwindBackgroundSize::Contain.boxed(),
            // https://tailwindcss.com/docs/background-blend-mode
            ["blend", rest @ ..] => TailwindBackgroundBlend::parse(rest, arbitrary)?.boxed(),
            _ => return syntax_error!("Unknown bg instructions: {}", str.join("-")),
        };
        Ok(out)
    }

    #[inline]
    fn divide_adaptor(str: &[&str], arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
        let out = match str {
            // https://tailwindcss.com/docs/divide-width
            ["x"] => todo!(),
            ["x", _n] => todo!(),
            ["y"] => todo!(),
            ["y", _n] => todo!(),
            // https://tailwindcss.com/docs/divide-style
            [s @ ("solid" | "dashed" | "dotted" | "double" | "none")] => TailwindDivideStyle::from(*s).boxed(),
            // https://tailwindcss.com/docs/divide-color
            _ => return syntax_error!("Unknown divide instructions: {}", str.join("-")),
        };
        Ok(out)
    }
    #[inline]
    fn ring_adaptor(str: &[&str], arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
        let out = match str {
            // https://tailwindcss.com/docs/ring-offset-width
            ["offset", rest @ ..] => TailwindRingOffsetWidth::parse(rest, arbitrary)?.boxed(),
            _ => return syntax_error!("Unknown ring instructions: {}", str.join("-")),
        };
        Ok(out)
    }
    #[inline]
    fn shadow_adaptor(str: &[&str], arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
        let out = match str {
            // https://tailwindcss.com/docs/box-shadow
            [] => todo!(),
            ["sm"] => todo!(),
            // https://tailwindcss.com/docs/box-shadow-color
            _ => TailwindShadow::parse(str, arbitrary, false)?.boxed(),
        };
        Ok(out)
    }
    #[inline]
    fn box_adaptor(str: &[&str], arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
        let out = match str {
            ["decoration", rest @ ..] => TailwindBoxDecoration::parse(rest, arbitrary)?.boxed(),
            ["border"] => TailwindBoxSizing::from("border").boxed(),
            ["content"] => TailwindBoxSizing::from("content").boxed(),
            _ => return syntax_error!("Unknown box instructions: {}", str.join("-")),
        };
        Ok(out)
    }

    #[inline]
    fn justify_adaptor(str: &[&str], arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
        debug_assert!(arbitrary.is_none(), "forbidden arbitrary after justify");
        let out = match str {
            // https://tailwindcss.com/docs/justify-items
            ["items", rest @ ..] => TailwindJustifyItems::parse(rest, arbitrary)?.boxed(),
            // https://tailwindcss.com/docs/justify-self
            ["self", rest @ ..] => TailwindJustifySelf::parse(rest, arbitrary)?.boxed(),
            // https://tailwindcss.com/docs/justify-content
            _ => TailwindJustifyContent::parse(str, arbitrary)?.boxed(),
        };
        Ok(out)
    }

    #[inline]
    fn backdrop_adaptor(str: &[&str], arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
        debug_assert!(arbitrary.is_none(), "forbidden arbitrary after justify");
        let out = match str {
            // https://tailwindcss.com/docs/backdrop-blur
            ["blur", rest @ ..] => TailwindBlur::parse(rest, arbitrary, true)?.boxed(),
            // https://tailwindcss.com/docs/backdrop-brightness
            ["brightness", rest @ ..] => TailwindBrightness::parse(rest, arbitrary, true)?.boxed(),
            // https://tailwindcss.com/docs/backdrop-contrast
            ["contrast", rest @ ..] => TailwindContrast::parse(rest, arbitrary, true)?.boxed(),
            // https://tailwindcss.com/docs/backdrop-grayscale
            ["grayscale", rest @ ..] => TailwindGrayscale::parse(rest, arbitrary, true)?.boxed(),
            // https://tailwindcss.com/docs/backdrop-hue-rotate
            ["hue", "rotate", rest @ ..] => TailwindHueRotate::parse(rest, arbitrary, true)?.boxed(),
            // https://tailwindcss.com/docs/backdrop-invert
            ["invert", rest @ ..] => TailwindInvert::parse(rest, arbitrary, true)?.boxed(),
            // https://tailwindcss.com/docs/backdrop-opacity
            ["opacity", rest @ ..] => TailwindOpacity::parse(rest, arbitrary, true)?.boxed(),
            // https://tailwindcss.com/docs/backdrop-saturate
            ["saturate", rest @ ..] => TailwindSaturate::parse(rest, arbitrary, true)?.boxed(),
            // https://tailwindcss.com/docs/backdrop-sepia
            ["sepia", rest @ ..] => TailwindSepia::parse(rest, arbitrary, true)?.boxed(),
            _ => return syntax_error!("Unknown backdrop instructions: {}", str.join("-")),
        };
        Ok(out)
    }
    #[inline]
    fn table_adaptor(str: &[&str], arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
        let out = match str {
            // https://tailwindcss.com/docs/display#flex
            // `[]` => This won't happen
            ["caption"] => TailwindTableLayout::Auto.boxed(),
            ["right"] => TailwindTableLayout::Auto.boxed(),
            ["none"] => TailwindTableLayout::Auto.boxed(),
            // https://tailwindcss.com/docs/table-layout
            ["auto"] => TailwindTableLayout::Auto.boxed(),
            ["fixed"] => TailwindTableLayout::Fixed.boxed(),
            _ => return syntax_error!("Unknown table instructions: {}", str.join("-")),
        };
        Ok(out)
    }
    #[inline]
    fn object_adaptor(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
        let out = match pattern {
            // https://tailwindcss.com/docs/object-fit
            [s @ ("contain" | "cover" | "fill" | "none")] => TailwindObjectFit::from(*s).boxed(),
            ["scale", "down"] => TailwindObjectFit::from("scale-down").boxed(),
            // https://tailwindcss.com/docs/object-position
            _ => TailwindObjectPosition::parse(pattern, arbitrary)?.boxed(),
        };
        Ok(out)
    }
    #[inline]
    fn overflow_adaptor(str: &[&str], arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
        let out = match str {
            // https://tailwindcss.com/docs/overflow
            ["x", pattern @ ..] => TailwindOverflow::parse(pattern, arbitrary, Some(true))?.boxed(),
            ["y", pattern @ ..] => TailwindOverflow::parse(pattern, arbitrary, Some(false))?.boxed(),
            _ => TailwindOverflow::parse(str, arbitrary, None)?.boxed(),
        };
        Ok(out)
    }
    #[inline]
    fn overscroll_adaptor(str: &[&str], arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
        let out = match str {
            // https://tailwindcss.com/docs/overscroll-behavior
            ["x", pattern @ ..] => TailwindOverscroll::parse(pattern, arbitrary, Some(true))?.boxed(),
            ["y", pattern @ ..] => TailwindOverscroll::parse(pattern, arbitrary, Some(true))?.boxed(),
            _ => TailwindOverscroll::parse(str, arbitrary, None)?.boxed(),
        };
        Ok(out)
    }
}
