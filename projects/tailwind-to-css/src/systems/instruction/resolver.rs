use super::*;

// noinspection SpellCheckingInspection
impl TailwindInstruction {
    #[inline(never)]
    pub fn get_instance(&self) -> Result<Box<dyn TailwindInstance>> {
        let element = self.view_elements();
        let arbitrary = self.view_arbitrary();
        let neg = self.negative;
        let instance = match element.as_slice() {
            // Layout System
            ["aspect", rest @ ..] => TailwindAspect::parse(rest, arbitrary)?.boxed(),
            ["container"] => TailwindContainer {}.boxed(),
            ["columns", rest @ ..] => TailwindColumns::parse(rest, arbitrary)?.boxed(),
            ["break", rest @ ..] => Self::break_adaptor(rest, arbitrary)?,
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
            ["isolate"] => TailwindIsolation::Isolate.boxed(),
            ["isolation", "auto"] => TailwindIsolation::Auto.boxed(),
            ["object", rest @ ..] => object_adaptor(rest, arbitrary)?,
            ["overflow", rest @ ..] => Self::overflow_adaptor(rest, arbitrary)?,
            ["overscroll", rest @ ..] => Self::overscroll_adaptor(rest, arbitrary)?,
            // https://tailwindcss.com/docs/position#header
            [s @ ("static" | "fixed" | "absolute" | "relative" | "sticky")] => TailwindPosition::from(*s).boxed(),
            // https://tailwindcss.com/docs/top-right-bottom-left
            ["inset", _rest @ ..] => todo!(),
            ["top", _rest @ ..] => todo!(),
            ["right", _rest @ ..] => todo!(),
            ["buttom", _rest @ ..] => todo!(),
            ["left", _rest @ ..] => todo!(),
            // https://tailwindcss.com/docs/visibility
            ["visible"] => TailwindVisibility::Visible.boxed(),
            ["invisible"] => TailwindVisibility::Invisible.boxed(),
            // https://tailwindcss.com/docs/z-index
            ["z", rest @ ..] => TailWindZIndex::parse(rest, arbitrary, self.negative)?.boxed(),
            // Flexbox & Grid
            ["basis", rest @ ..] => TailwindBasis::parse(rest, arbitrary)?.boxed(),
            ["flex", rest @ ..] => Self::flex_adaptor(rest, arbitrary)?,
            ["grow", rest @ ..] => TailWindGrow::parse(rest, arbitrary)?.boxed(),
            ["shrink", rest @ ..] => TailWindShrink::parse(rest, arbitrary)?.boxed(),
            ["order", rest @ ..] => TailWindOrder::parse(rest, arbitrary, self.negative)?.boxed(),
            ["grid", rest @ ..] => Self::grid_adaptor(rest, arbitrary)?,
            // https://tailwindcss.com/docs/grid-column
            ["col", rest @ ..] => TailwindColumn::parse(rest, arbitrary)?.boxed(),
            ["row", rest @ ..] => TailwindRow::parse(rest, arbitrary)?.boxed(),
            ["auto", rest @ ..] => TailwindGridAuto::parse(rest, arbitrary)?.boxed(),
            ["gap", "x", rest @ ..] => TailwindGap::parse_x(rest, arbitrary)?.boxed(),
            ["gap", "y", rest @ ..] => TailwindGap::parse_y(rest, arbitrary)?.boxed(),
            ["gap", rest @ ..] => TailwindGap::parse_xy(rest, arbitrary)?.boxed(),
            ["justify", rest @ ..] => Self::justify_adaptor(rest, arbitrary)?,
            ["content", rest @ ..] => Self::content_adaptor(rest, arbitrary)?,
            ["items", rest @ ..] => TailwindItems::parse(rest, arbitrary)?.boxed(),
            ["self", rest @ ..] => TailwindSelf::parse(rest, arbitrary)?.boxed(),
            ["place", rest @ ..] => Self::place_adaptor(rest, arbitrary)?,
            // justify catched
            // Spacing System
            [p @ ("p" | "pl" | "pr" | "pm" | "pt" | "px" | "py"), rest @ ..] =>
                TailwindSpacing::parse_padding(rest, p, arbitrary)?.boxed(),
            [m @ ("m" | "ml" | "mr" | "mm" | "mt" | "mx" | "my"), rest @ ..] =>
                TailwindSpacing::parse_margin(rest, m, arbitrary)?.boxed(),
            ["space", "x", rest @ ..] => TailwindSpacing::parse_space(rest, 'x', arbitrary)?.boxed(),
            ["space", "y", rest @ ..] => TailwindSpacing::parse_space(rest, 'y', arbitrary)?.boxed(),
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
            ["list", rest @ ..] => Self::list_adaptor(rest, arbitrary)?,
            // https://tailwindcss.com/docs/text-decoration
            ["underline"] => TailwindDecorationLine::Underline.boxed(),
            ["overline"] => TailwindDecorationLine::Overline.boxed(),
            ["line", "through"] => TailwindDecorationLine::ThroughLine.boxed(),
            ["no", "underline"] => TailwindDecorationLine::None.boxed(),
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
            // FIXME: https://tailwindcss.com/docs/background-blend-mode
            ["bg", rest @ ..] => Self::bg_adaptor(rest, arbitrary)?,
            ["from", _rest @ ..] => todo!(),
            ["via", _rest @ ..] => todo!(),
            ["to", _rest @ ..] => todo!(),
            // Borders System
            ["rounded", rest @ ..] => TailwindRounded::parse(rest, arbitrary)?.boxed(),
            ["border", rest @ ..] => TailwindBorder::parse(rest, arbitrary)?,
            ["divide", rest @ ..] => Self::divide_adaptor(rest, arbitrary)?,
            ["outline", rest @ ..] => outline_adaptor(rest, arbitrary)?,
            ["ring", rest @ ..] => Self::ring_adaptor(rest, arbitrary)?,
            // Effects System
            ["shadow", rest @ ..] => Self::shadow_adaptor(rest, arbitrary)?,
            ["opacity", rest @ ..] => TailwindOpacity::parse(rest, arbitrary)?.boxed(),
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
            ["transition", _rest @ ..] => todo!(),
            ["duration", rest @ ..] => TailwindDuration::parse(rest, arbitrary)?.boxed(),
            ["ease", _rest @ ..] => todo!(),
            ["delay", rest @ ..] => TailwindDelay::parse(rest, arbitrary)?.boxed(),
            ["animate", _rest @ ..] => todo!(),
            // Transforms System
            ["scale", "x", rest @ ..] => TailwindScale::parse(rest, arbitrary, Some(true), neg)?.boxed(),
            ["scale", "y", rest @ ..] => TailwindScale::parse(rest, arbitrary, Some(false), neg)?.boxed(),
            ["scale", rest @ ..] => TailwindScale::parse(rest, arbitrary, None, neg)?.boxed(),
            ["rotate", rest @ ..] => TailwindRotate::parse(rest, arbitrary, neg)?.boxed(),
            ["translate", "x", _rest @ ..] => todo!(),
            ["translate", "y", _rest @ ..] => todo!(),
            ["skew", "x", rest @ ..] => TailwindSkew::parse(rest, arbitrary, true, neg)?.boxed(),
            ["skew", "y", rest @ ..] => TailwindSkew::parse(rest, arbitrary, false, neg)?.boxed(),
            ["origin", rest @ ..] => TailwindOrigin::parse(rest, arbitrary)?.boxed(),
            // Interactivity System
            ["accent", rest @ ..] => TailwindAccentColor::parse(rest, arbitrary)?.boxed(),
            // https://tailwindcss.com/docs/appearance
            ["appearance", "none"] => TailwindAppearance::None.boxed(),
            ["cursor", rest @ ..] => TailwindCursor::parse(rest, arbitrary)?.boxed(),
            ["caret", rest @ ..] => TailwindCaretColor::parse(rest, arbitrary)?.boxed(),
            ["pointer", "events", rest @ ..] => TailwindPointerEvents::parse(rest, arbitrary)?.boxed(),
            ["resize", rest @ ..] => TailwindResize::parse(rest, arbitrary)?.boxed(),
            ["scroll", rest @ ..] => TailwindScroll::parse(rest, arbitrary)?.boxed(),
            ["snap", rest @ ..] => TailwindSnap::parse(rest, arbitrary)?.boxed(),
            ["touch", rest @ ..] => TailwindTorch::parse(rest, arbitrary)?.boxed(),
            ["select", rest @ ..] => TailwindSelect::parse(rest, arbitrary)?.boxed(),
            ["will", "change", rest @ ..] => TailwindWillChange::parse(rest, arbitrary)?.boxed(),
            // SVG System
            ["fill", _rest @ ..] => todo!(),
            ["stroke", _rest @ ..] => todo!(),
            // Accessibility System
            ["sr", "only"] => TailwindScreenReader::new(true).boxed(),
            ["not", "sr", "only"] => TailwindScreenReader::new(false).boxed(),
            // Form System Extension
            _ => return syntax_error!("Unknown instructions: {} + [{}]", element.join("-"), arbitrary),
        };
        Ok(instance)
    }
    #[inline]
    fn break_adaptor(str: &[&str], arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
        let out = match str {
            // https://tailwindcss.com/docs/border-style
            ["normal"] => TailwindBreak::Normal.boxed(),
            ["words"] => TailwindBreak::Words.boxed(),
            ["all"] => TailwindBreak::All.boxed(),
            // https://tailwindcss.com/docs/break-before
            ["before", rest @ ..] => TailwindBreakBefore::parse(rest, arbitrary)?.boxed(),
            // https://tailwindcss.com/docs/break-inside
            ["inside", rest @ ..] => TailwindBreakInside::parse(rest, arbitrary)?.boxed(),
            // https://tailwindcss.com/docs/break-after
            ["after", rest @ ..] => TailwindBreakAfter::parse(rest, arbitrary)?.boxed(),
            _ => return syntax_error!("Unknown break instructions: {}", str.join("-")),
        };
        Ok(out)
    }
    #[inline]
    fn bg_adaptor(str: &[&str], arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
        let out = match str {
            // https://tailwindcss.com/docs/background-attachment
            ["fixed"] => TailwindBackgroundAttachment::Fixed.boxed(),
            ["local"] => TailwindBackgroundAttachment::Local.boxed(),
            ["scroll"] => TailwindBackgroundAttachment::Scroll.boxed(),
            // https://tailwindcss.com/docs/background-clip
            ["clip", "border"] => TailwindBackgroundClip::Border.boxed(),
            ["clip", "padding"] => TailwindBackgroundClip::Padding.boxed(),
            ["clip", "content"] => TailwindBackgroundClip::Content.boxed(),
            ["clip", "text"] => TailwindBackgroundClip::Text.boxed(),
            // https://tailwindcss.com/docs/background-origin
            ["origin", "padding"] => TailwindBackgroundOrigin::Padding.boxed(),
            ["origin", "content"] => TailwindBackgroundOrigin::Content.boxed(),
            ["origin", "border"] => TailwindBackgroundOrigin::Border.boxed(),
            // https://tailwindcss.com/docs/background-repeat
            ["repeat"] => TailwindBackgroundRepeat::Repeat.boxed(),
            ["no", "repeat"] | ["repeat", "none"] => TailwindBackgroundRepeat::None.boxed(),
            ["repeat", "x"] => TailwindBackgroundRepeat::RepeatX.boxed(),
            ["repeat", "y"] => TailwindBackgroundRepeat::RepeatY.boxed(),
            ["repeat", "round"] => TailwindBackgroundRepeat::Round.boxed(),
            ["repeat", "space"] => TailwindBackgroundRepeat::Space.boxed(),
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
    fn divide_adaptor(str: &[&str], _arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
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
    fn box_adaptor(str: &[&str], _arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
        let out = match str {
            ["decoration", "clone"] => TailwindBoxDecoration::Clone.boxed(),
            ["decoration", "slice"] => TailwindBoxDecoration::Slice.boxed(),
            ["border"] => TailwindBoxSizing::Border.boxed(),
            ["content"] => TailwindBoxSizing::Content.boxed(),
            _ => return syntax_error!("Unknown box instructions: {}", str.join("-")),
        };
        Ok(out)
    }
    #[inline]
    fn flex_adaptor(str: &[&str], arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
        let out = match str {
            // https://tailwindcss.com/docs/display#flex
            [] if arbitrary.is_none() => TailwindDisplay::Flex.boxed(),
            // https://tailwindcss.com/docs/flex#arbitrary-values
            [] => TailwindFlex::parse_arbitrary(arbitrary)?.boxed(),
            // https://tailwindcss.com/docs/flex-direction
            ["row"] => TailwindFlexDirection::Row.boxed(),
            ["row", "reverse"] => TailwindFlexDirection::RowReverse.boxed(),
            ["col"] => TailwindFlexDirection::Column.boxed(),
            ["col", "reverse"] => TailwindFlexDirection::ColumnReverse.boxed(),
            // https://tailwindcss.com/docs/flex-wrap
            ["wrap"] => TailwindFlexWrap::Wrap.boxed(),
            ["wrap", "reverse"] => TailwindFlexWrap::WrapReverse.boxed(),
            ["nowrap"] => TailwindFlexWrap::NoWrap.boxed(),
            // https://tailwindcss.com/docs/flex
            ["auto"] => TailwindFlex::Inherit.boxed(),
            ["initial"] => TailwindFlex::Inherit.boxed(),
            ["none"] => TailwindFlex::None.boxed(),
            [n] => TailwindFlex::parse(n)?.boxed(),
            _ => return syntax_error!("Unknown box instructions: {}", str.join("-")),
        };
        Ok(out)
    }
    #[inline]
    fn grid_adaptor(str: &[&str], arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
        debug_assert!(arbitrary.is_none(), "forbidden arbitrary after place");
        let out = match str {
            // https://tailwindcss.com/docs/grid-template-rows
            ["rows", _rest @ ..] => TailwindListStyle::None.boxed(),
            // https://tailwindcss.com/docs/grid-auto-flow
            ["flow", _rest @ ..] => TailwindListStyle::None.boxed(),
            // https://tailwindcss.com/docs/place-self
            ["self", _rest @ ..] => TailwindListStyle::None.boxed(),
            _ => return syntax_error!("Unknown list instructions: {}", str.join("-")),
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
    fn content_adaptor(str: &[&str], arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
        let out = match str {
            // https://tailwindcss.com/docs/align-content
            ["center"] => TailwindContent::Center.boxed(),
            ["start"] => TailwindContent::Start.boxed(),
            ["end"] => TailwindContent::End.boxed(),
            ["between"] => TailwindContent::Between.boxed(),
            ["around"] => TailwindContent::Around.boxed(),
            ["evenly"] => TailwindContent::Evenly.boxed(),
            // https://tailwindcss.com/docs/content
            ["none"] => TailwindContentElement::None.boxed(),
            // https://tailwindcss.com/docs/content#arbitrary-values
            [] => TailwindContentElement::parse_arbitrary(arbitrary)?.boxed(),
            _ => return syntax_error!("Unknown content instructions: {}", str.join("-")),
        };
        Ok(out)
    }
    #[inline]
    fn place_adaptor(str: &[&str], arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
        let out = match str {
            // https://tailwindcss.com/docs/place-content
            ["content", rest @ ..] => TailwindPlaceContent::parse(rest, arbitrary)?.boxed(),
            // https://tailwindcss.com/docs/place-items
            ["items", rest @ ..] => TailwindPlaceItems::parse(rest, arbitrary)?.boxed(),
            // https://tailwindcss.com/docs/place-self
            ["self", rest @ ..] => TailwindPlaceSelf::parse(rest, arbitrary)?.boxed(),
            _ => return syntax_error!("Unknown place instructions: {}", str.join("-")),
        };
        Ok(out)
    }
    #[inline]
    fn list_adaptor(str: &[&str], arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
        let out = match str {
            // https://tailwindcss.com/docs/list-style-type
            ["none"] => TailwindListStyle::None.boxed(),
            ["disc"] => TailwindListStyle::Disc.boxed(),
            ["decimal"] => TailwindListStyle::Decimal.boxed(),
            // https://tailwindcss.com/docs/list-style-position
            ["inside"] => TailwindListStylePosition::Inside.boxed(),
            ["outside"] => TailwindListStylePosition::Outside.boxed(),
            // https://tailwindcss.com/docs/list-style-type#arbitrary-values
            [] => TailwindListStyle::parse_arbitrary(arbitrary)?.boxed(),
            _ => return syntax_error!("Unknown list instructions: {}", str.join("-")),
        };
        Ok(out)
    }

    #[inline]
    fn backdrop_adaptor(str: &[&str], arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
        debug_assert!(arbitrary.is_none(), "forbidden arbitrary after justify");
        let out = match str {
            // https://tailwindcss.com/docs/justify-content
            ["content", _rest @ ..] => TailwindListStyle::None.boxed(),
            // https://tailwindcss.com/docs/justify-items
            ["items", _rest @ ..] => TailwindListStyle::None.boxed(),
            // https://tailwindcss.com/docs/justify-self
            ["self", _rest @ ..] => TailwindListStyle::None.boxed(),
            _ => return syntax_error!("Unknown justify instructions: {}", str.join("-")),
        };
        Ok(out)
    }
    #[inline]
    fn table_adaptor(str: &[&str], _arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
        let out = match str {
            ["caption"] => TailwindTableLayout::Auto.boxed(),
            ["right"] => TailwindTableLayout::Auto.boxed(),
            ["none"] => TailwindTableLayout::Auto.boxed(),
            // https://tailwindcss.com/docs/table-layout
            ["auto"] => TailwindTableLayout::Auto.boxed(),
            ["fixed"] => TailwindTableLayout::Fixed.boxed(),
            // https://tailwindcss.com/docs/display#table
            [] => TailwindDisplay::Table.boxed(),
            _ => return syntax_error!("Unknown table instructions: {}", str.join("-")),
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
