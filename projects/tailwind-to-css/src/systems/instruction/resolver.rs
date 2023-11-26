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
            // skip [flex, table]
            ["block"] => TailwindDisplay::from("block").boxed(),
            ["inline", "block"] => TailwindDisplay::from("inline-block").boxed(),
            ["inline"] => TailwindDisplay::from("inline").boxed(),
            ["inline", "flex"] => TailwindDisplay::from("inline-flex").boxed(),
            ["inline", "table"] => TailwindDisplay::from("inline-table").boxed(),
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
            ["object", rest @ ..] => object_adaptor(rest, arbitrary)?,
            ["overflow", rest @ ..] => TailwindOverflow::parse(rest, arbitrary)?.boxed(),
            ["overscroll", rest @ ..] => TailwindOverscroll::parse(rest, arbitrary)?.boxed(),
            // https://tailwindcss.com/docs/position#header
            [s @ ("static" | "fixed" | "absolute" | "relative" | "sticky")] => TailwindPosition::from(*s).boxed(),
            ["position", rest @ ..] => TailwindPosition::parse(rest, arbitrary)?.boxed(),
            // https://tailwindcss.com/docs/top-right-bottom-left
            ["inset", rest @ ..] => TailwindInset::parse(rest, arbitrary, neg)?.boxed(),
            ["top", rest @ ..] => TailwindTop::parse(rest, arbitrary, neg)?.boxed(),
            ["right", rest @ ..] => TailwindRight::parse(rest, arbitrary, neg)?.boxed(),
            ["bottom", rest @ ..] => TailwindBottom::parse(rest, arbitrary, neg)?.boxed(),
            ["left", rest @ ..] => TailwindLeft::parse(rest, arbitrary, neg)?.boxed(),
            // https://tailwindcss.com/docs/visibility
            ["invisible"] => TailwindVisibility::from("hidden").boxed(),
            ["visible" | "visibility", rest @ ..] => TailwindVisibility::parse(rest, arbitrary)?.boxed(),
            // https://tailwindcss.com/docs/z-index
            ["z", rest @ ..] => TailwindZIndex::parse(rest, arbitrary, neg)?.boxed(),
            // Flexbox & Grid
            ["basis", rest @ ..] => TailwindBasis::parse(rest, arbitrary)?.boxed(),
            ["flex", rest @ ..] => TailwindFlex::adapt(rest, arbitrary)?,
            ["grow", rest @ ..] => TailWindGrow::parse(rest, arbitrary)?.boxed(),
            ["shrink", rest @ ..] => TailWindShrink::parse(rest, arbitrary)?.boxed(),
            ["order", rest @ ..] => TailWindOrder::parse(rest, arbitrary, neg)?.boxed(),
            ["grid", rest @ ..] => TailwindGrid::adapt(rest, arbitrary)?,
            // https://tailwindcss.com/docs/grid-column
            ["col", rest @ ..] => TailwindColumn::parse(rest, arbitrary)?.boxed(),
            ["row", rest @ ..] => TailwindRow::parse(rest, arbitrary)?.boxed(),
            ["auto", rest @ ..] => TailwindGridAuto::parse(rest, arbitrary)?.boxed(),
            ["gap", rest @ ..] => TailwindGap::parse(rest, arbitrary)?.boxed(),
            ["justify", rest @ ..] => justify_adaptor(rest, arbitrary)?,
            ["content", rest @ ..] => TailwindContent::adapt(rest, arbitrary)?,
            ["items", rest @ ..] => TailwindItems::parse(rest, arbitrary)?.boxed(),
            ["self", rest @ ..] => TailwindSelf::parse(rest, arbitrary)?.boxed(),
            ["place", rest @ ..] => TailwindPlace::adapt(rest, arbitrary)?,
            // justify catched
            // Spacing System
            ["p" | "pl" | "pr" | "pb" | "pt" | "px" | "py", ..] => TailwindPadding::parse(pattern, arbitrary, neg)?.boxed(),
            ["m" | "ml" | "mr" | "mb" | "mt" | "mx" | "my", ..] => TailwindMargin::parse(pattern, arbitrary, neg)?.boxed(),
            ["space", rest @ ..] => TailwindSpace::parse(rest, arbitrary, neg)?,
            // Sizing System
            ["w", rest @ ..] => TailwindSizing::parse_width(rest, arbitrary)?.boxed(),
            ["min", "w", rest @ ..] => TailwindSizing::parse_width_min(rest, arbitrary)?.boxed(),
            ["max", "w", rest @ ..] => TailwindSizing::parse_width_max(rest, arbitrary)?.boxed(),
            ["h", rest @ ..] => TailwindSizing::parse_height(rest, arbitrary)?.boxed(),
            ["min", "h", rest @ ..] => TailwindSizing::parse_height_min(rest, arbitrary)?.boxed(),
            ["max", "h", rest @ ..] => TailwindSizing::parse_height_max(rest, arbitrary)?.boxed(),
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
            ["list", rest @ ..] => list_adaptor(rest, arbitrary)?,
            // https://tailwindcss.com/docs/text-decoration
            ["underline"] => TailwindDecorationLine::from("underline").boxed(),
            ["overline"] => TailwindDecorationLine::from("overline").boxed(),
            ["line", "through"] => TailwindDecorationLine::from("line-through").boxed(),
            ["no", "underline"] => TailwindDecorationLine::from("none").boxed(),
            // https://tailwindcss.com/docs/text-decoration-color
            ["decoration", rest @ ..] => TailwindDecoration::adapt(rest, arbitrary)?,
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
            ["border", rest @ ..] => Self::border_adaptor(rest, arbitrary)?,
            ["divide", rest @ ..] => TailwindDivide::adapt(rest, arbitrary)?,
            ["outline", rest @ ..] => outline_adaptor(rest, arbitrary)?,
            ["ring", rest @ ..] => TailwindRing::adapt(rest, arbitrary)?,
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
            ["hue", "rotate", rest @ ..] => TailwindHueRotate::parse(rest, arbitrary, false, neg)?.boxed(),
            ["invert", rest @ ..] => TailwindInvert::parse(rest, arbitrary, false)?.boxed(),
            ["saturate", rest @ ..] => TailwindSaturate::parse(rest, arbitrary, false)?.boxed(),
            ["sepia", rest @ ..] => TailwindSepia::parse(rest, arbitrary, false)?.boxed(),
            ["backdrop", rest @ ..] => Self::backdrop_adaptor(rest, arbitrary, neg)?,
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
            ["scroll", rest @ ..] => scroll_adaptor(rest, arbitrary, neg)?,
            ["snap", rest @ ..] => snap_adaptor(rest, arbitrary)?,
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
            _ => return syntax_error!("Unknown instructions: {} + {}", element.join("-"), arbitrary.get_class()),
        };
        Ok(instance)
    }
    #[inline]
    fn bg_adaptor(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
        let out = match pattern {
            // https://tailwindcss.com/docs/background-attachment
            [s @ ("fixed" | "local" | "scroll")] => TailwindBackgroundAttachment::from(*s).boxed(),
            ["attach", rest @ ..] => TailwindBackgroundAttachment::parse(rest, arbitrary)?.boxed(),
            // https://tailwindcss.com/docs/background-clip
            ["clip", rest @ ..] => TailwindBackgroundClip::parse(rest, arbitrary)?.boxed(),
            // https://tailwindcss.com/docs/background-origin
            ["origin", rest @ ..] => TailwindBackgroundOrigin::parse(rest, arbitrary)?.boxed(),
            // https://tailwindcss.com/docs/background-repeat
            ["no", "repeat"] => TailwindBackgroundRepeat::from("no-repeat").boxed(),
            ["repeat", rest @ ..] => TailwindBackgroundRepeat::parse(rest, arbitrary)?.boxed(),
            // https://tailwindcss.com/docs/background-size
            [s @ ("auto" | "cover" | "contain")] => TailwindBackgroundSize::from(*s).boxed(),
            ["size", rest @ ..] => TailwindBackgroundSize::parse(rest, arbitrary)?.boxed(),
            // https://tailwindcss.com/docs/background-blend-mode
            ["blend", rest @ ..] => TailwindBackgroundBlend::parse(rest, arbitrary)?.boxed(),
            _ => TailwindBackgroundColor::parse(pattern, arbitrary)?.boxed(),
        };
        Ok(out)
    }
    #[inline]
    fn border_adaptor(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
        let color = |color| TailwindBorderColor::from(color).boxed();
        let out = match pattern {
            // https://tailwindcss.com/docs/border-style
            [s @ ("solid" | "dashed" | "dotted" | "double" | "hidden" | "none")] => TailwindBorderStyle::from(*s).boxed(),
            // https://tailwindcss.com/docs/border-collapse
            ["separate"] => TailwindBorderCollapse::from("separate").boxed(),
            ["collapse"] if arbitrary.is_none() => TailwindBorderCollapse::from("collapse").boxed(),
            ["collapse", rest @ ..] => TailwindBorderCollapse::parse(rest, arbitrary)?.boxed(),
            // https://tailwindcss.com/docs/border-width
            [] => TailwindBorderWidth::parse(pattern, arbitrary)?.boxed(), // e.g. border-[3px]
            ["0" | "2" | "4" | "8", ..] if arbitrary.is_none() => TailwindBorderWidth::parse(pattern, arbitrary)?.boxed(), // e.g. border-4
            ["x" | "y" | "t" | "r" | "b" | "l", ..] => TailwindBorderWidth::parse(pattern, arbitrary)?.boxed(), // e.g. border-x-2
            // https://tailwindcss.com/docs/border-color
            ["black"] => color(TailwindColor::Black),
            ["white"] => color(TailwindColor::White),
            _ => TailwindBorderColor::parse(pattern, arbitrary)?.boxed(),
        };
        Ok(out)
    }
    #[inline]
    fn shadow_adaptor(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
        let out = match pattern {
            // https://tailwindcss.com/docs/box-shadow
            ["black" | "white" | "current" | "transparent"] => TailwindShadowColor::parse(pattern, arbitrary)?.boxed(),
            ["color", rest @ ..] => TailwindShadowColor::parse(rest, arbitrary)?.boxed(),
            // https://tailwindcss.com/docs/box-shadow-color
            _ => TailwindShadow::parse(pattern, arbitrary, false)?.boxed(),
        };
        Ok(out)
    }
    #[inline]
    fn box_adaptor(str: &[&str], arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
        let out = match str {
            // https://tailwindcss.com/docs/box-decoration-break
            ["decoration" | "break", rest @ ..] => TailwindBoxDecoration::parse(rest, arbitrary)?.boxed(),
            ["clone"] => TailwindBoxDecoration::from("clone").boxed(),
            ["slice"] => TailwindBoxDecoration::from("slice").boxed(),
            // https://tailwindcss.com/docs/box-sizing
            ["border"] => TailwindBoxSizing::from("border-box").boxed(),
            ["content"] => TailwindBoxSizing::from("content-box").boxed(),
            ["sizing", rest @ ..] => TailwindBoxSizing::parse(rest, arbitrary)?.boxed(),
            _ => return syntax_error!("Unknown box instructions: {}", str.join("-")),
        };
        Ok(out)
    }

    #[inline]
    fn backdrop_adaptor(str: &[&str], arbitrary: &TailwindArbitrary, negative: Negative) -> Result<Box<dyn TailwindInstance>> {
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
            ["hue", "rotate", rest @ ..] => TailwindHueRotate::parse(rest, arbitrary, true, negative)?.boxed(),
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
    fn table_adaptor(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
        let out = match pattern {
            // https://tailwindcss.com/docs/display#flex
            [] if arbitrary.is_none() => TailwindDisplay::from("table").boxed(),
            ["caption"] => TailwindDisplay::from("table-caption").boxed(),
            ["cell"] => TailwindDisplay::from("table-cell").boxed(),
            ["column"] => TailwindDisplay::from("table-column").boxed(),
            ["column", "group"] => TailwindDisplay::from("table-column-group").boxed(),
            ["footer", "group"] => TailwindDisplay::from("table-footer-group").boxed(),
            ["header", "group"] => TailwindDisplay::from("table-header-group").boxed(),
            ["row", "group"] => TailwindDisplay::from("table-row-group").boxed(),
            ["row"] => TailwindDisplay::from("table-row").boxed(),
            // https://tailwindcss.com/docs/table-layout
            _ => TailwindTableLayout::parse(pattern, arbitrary)?.boxed(),
        };
        Ok(out)
    }
}
