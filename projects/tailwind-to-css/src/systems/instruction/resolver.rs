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
            ["block"] => TailwindDisplay::Block.boxed(),
            ["inline", "block"] => TailwindDisplay::InlineBlock.boxed(),
            ["inline"] => TailwindDisplay::InlineFlex.boxed(),
            // flex catched
            ["inline", "flex"] => TailwindDisplay::InlineFlex.boxed(),
            // table catched
            ["inline", "table"] => TailwindDisplay::InlineFlex.boxed(),
            // table catched
            ["flow", "root"] => TailwindDisplay::InlineFlex.boxed(),
            // grid catched
            ["inline", "grid"] => TailwindDisplay::InlineFlex.boxed(),
            ["contents"] => TailwindDisplay::InlineFlex.boxed(),
            // list catched
            ["hidden"] => TailwindDisplay::InlineFlex.boxed(),
            // https://tailwindcss.com/docs/float
            ["float", rest @ ..] => TailwindFloat::parse(rest, arbitrary)?.boxed(),
            ["clear", rest @ ..] => TailwindClear::parse(rest, arbitrary)?.boxed(),
            ["isolate"] => TailwindIsolation::Isolate.boxed(),
            ["isolation", "auto"] => TailwindIsolation::Auto.boxed(),
            ["object", rest @ ..] => Self::object_adaptor(rest, arbitrary)?,
            ["overflow", rest @ ..] => Self::overflow_adaptor(rest, arbitrary)?,
            ["overscroll", rest @ ..] => Self::overscroll_adaptor(rest, arbitrary)?,
            // begin https://tailwindcss.com/docs/position
            ["static"] => TailwindPosition::Static.boxed(),
            ["fixed"] => TailwindPosition::Fixed.boxed(),
            ["absolute"] => TailwindPosition::Absolute.boxed(),
            ["relative"] => TailwindPosition::Relative.boxed(),
            ["sticky"] => TailwindPosition::Sticky.boxed(),
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
            ["font", rest @ ..] => Self::font_adaptor(rest, arbitrary)?,
            ["text", rest @ ..] => Self::text_adaptor(rest, arbitrary)?,
            // begin https://tailwindcss.com/docs/font-variant-numeric
            ["antialiased"] => TailwindFontSmoothing::new(false).boxed(),
            ["subpixel", "antialiased"] => TailwindFontSmoothing::new(true).boxed(),
            ["italic"] => TailwindFontStyle::Italic.boxed(),
            ["not", "italic"] => TailwindFontStyle::Normal.boxed(),
            // https://tailwindcss.com/docs/font-variant-numeric
            ["normal", "nums"] => TailwindFontVariantNumeric::Normal.boxed(),
            ["ordinal"] => TailwindFontVariantNumeric::Ordinal.boxed(),
            ["slashed", "zero"] => TailwindFontVariantNumeric::SlashedZero.boxed(),
            ["lining", "nums"] => TailwindFontVariantNumeric::Lining.boxed(),
            ["oldstyle", "nums"] => TailwindFontVariantNumeric::OldStyle.boxed(),
            ["proportional", "nums"] => TailwindFontVariantNumeric::Proportional.boxed(),
            ["tabular", "nums"] => TailwindFontVariantNumeric::Tabular.boxed(),
            ["diagonal", "fractions"] => TailwindFontVariantNumeric::DiagonalFractions.boxed(),
            ["stacked", "fractions"] => TailwindFontVariantNumeric::StackedFractions.boxed(),
            // https://tailwindcss.com/docs/letter-spacing
            ["tracking", rest @ ..] => TailwindTracking::parse(rest, arbitrary)?.boxed(),
            ["leading", rest @ ..] => TailwindLeading::parse(rest, arbitrary)?.boxed(),
            ["list", rest @ ..] => Self::list_adaptor(rest, arbitrary)?,
            // https://tailwindcss.com/docs/text-decoration
            ["underline"] => TailwindTextDecoration::Underline.boxed(),
            ["overline"] => TailwindTextDecoration::Overline.boxed(),
            ["line", "through"] => TailwindTextDecoration::ThroughLine.boxed(),
            ["no", "underline"] => TailwindTextDecoration::None.boxed(),
            // https://tailwindcss.com/docs/text-decoration-color
            ["decoration", rest @ ..] => decoration_adaptor(rest, arbitrary)?,
            ["underline", "offset", rest @ ..] => TailwindUnderlineOffset::parse(rest, arbitrary)?.boxed(),
            // https://tailwindcss.com/docs/text-transform
            ["uppercase"] => TailwindTextTransform::Uppercase.boxed(),
            ["lowercase"] => TailwindTextTransform::Lowercase.boxed(),
            ["capitalize"] => TailwindTextTransform::Capitalize.boxed(),
            ["normal", "case"] => TailwindTextTransform::None.boxed(),
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
            ["border", rest @ ..] => Self::border_adaptor(rest, arbitrary)?,
            ["divide", rest @ ..] => Self::divide_adaptor(rest, arbitrary)?,
            ["outline", rest @ ..] => Self::outline_adaptor(rest, arbitrary)?,
            ["ring", rest @ ..] => Self::ring_adaptor(rest, arbitrary)?,
            // Effects System
            ["shadow", rest @ ..] => Self::shadow_adaptor(rest, arbitrary)?,
            ["opacity", rest @ ..] => TailwindOpacity::parse(rest, arbitrary)?.boxed(),
            ["mix", "blend", rest @ ..] => TailwindBlend::parse(rest, arbitrary)?.boxed(),
            // Filters System
            ["blur", rest @ ..] => TailwindBlur::parse(rest, arbitrary, false)?.boxed(),
            ["brightness", rest @ ..] => TailwindBrightness::parse(rest, arbitrary, false)?.boxed(),
            ["contrast", rest @ ..] => TailwindContrast::parse(rest, arbitrary, false)?.boxed(),
            ["drop", "shadow", rest @ ..] => TailwindShadow::parse_drop(rest, arbitrary)?.boxed(),
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
    fn break_adaptor(str: &[&str], _arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
        let out = match str {
            // https://tailwindcss.com/docs/border-style
            ["normal"] => TailwindBreak::Normal.boxed(),
            ["words"] => TailwindBreak::Words.boxed(),
            ["all"] => TailwindBreak::All.boxed(),
            // https://tailwindcss.com/docs/break-before
            ["before", rest @ ..] => TailwindBreakLayout::parse_before(rest)?.boxed(),
            // https://tailwindcss.com/docs/break-inside
            ["inside", rest @ ..] => TailwindBreakLayout::parse_inside(rest)?.boxed(),
            // https://tailwindcss.com/docs/break-after
            ["after", rest @ ..] => TailwindBreakLayout::parse_after(rest)?.boxed(),
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
    fn border_adaptor(str: &[&str], _arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
        let out = match str {
            // border
            // [] => TailwindBorderWidth::parse(),

            // https://tailwindcss.com/docs/border-style
            ["solid"] => TailwindBorderStyle::Solid.boxed(),
            ["dashed"] => TailwindBorderStyle::Dashed.boxed(),
            ["dotted"] => TailwindBorderStyle::Dotted.boxed(),
            ["double"] => TailwindBorderStyle::Double.boxed(),
            ["hidden"] => TailwindBorderStyle::Hidden.boxed(),
            ["none"] => TailwindBorderStyle::None.boxed(),
            // https://tailwindcss.com/docs/border-collapse
            ["collapse"] => TailwindBorderCollapse::Collapse.into_instance(),
            ["separate"] => TailwindBorderCollapse::Separate.into_instance(),
            // https://tailwindcss.com/docs/border-color
            ["inherit"] => TailwindBorderCollapse::Collapse.into_instance(),
            ["current"] => TailwindBorderCollapse::Separate.into_instance(),
            ["transparent"] => TailwindBorderCollapse::Separate.into_instance(),
            ["black"] => TailwindBorderCollapse::Separate.into_instance(),
            ["white"] => TailwindBorderCollapse::Separate.into_instance(),

            _ => return syntax_error!("Unknown border instructions: {}", str.join("-")),
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
            ["solid"] => TailwindDivideStyle::Solid.boxed(),
            ["dashed"] => TailwindDivideStyle::Dashed.boxed(),
            ["dotted"] => TailwindDivideStyle::Dotted.boxed(),
            ["double"] => TailwindDivideStyle::Double.boxed(),
            ["none"] => TailwindDivideStyle::None.boxed(),
            // https://tailwindcss.com/docs/divide-color
            _ => return syntax_error!("Unknown divide instructions: {}", str.join("-")),
        };
        Ok(out)
    }
    #[inline]
    fn outline_adaptor(str: &[&str], _arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
        let out = match str {
            // https://tailwindcss.com/docs/outline-style
            [] => todo!(),
            ["none"] => TailwindOutlineStyle::None.boxed(),
            ["dashed"] => TailwindOutlineStyle::Dashed.boxed(),
            ["dotted"] => TailwindOutlineStyle::Dotted.boxed(),
            ["double"] => TailwindOutlineStyle::Double.boxed(),
            ["hidden"] => TailwindOutlineStyle::Hidden.boxed(),
            // https://tailwindcss.com/docs/outline-offset
            ["offset", _n] => todo!(),
            // https://tailwindcss.com/docs/outline-width
            [_n] => todo!(),
            _ => return syntax_error!("Unknown outline instructions: {}", str.join("-")),
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
            [] => TailwindShadow::parse_arbitrary(arbitrary)?.boxed(),
            ["sm"] => todo!(),
            // https://tailwindcss.com/docs/box-shadow-color
            _ => return syntax_error!("Unknown shadow instructions: {}", str.join("-")),
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
    fn object_adaptor(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
        let out = match pattern {
            // https://tailwindcss.com/docs/object-fit
            ["contain"] => TailwindObjectFit::Contain.boxed(),
            ["cover"] => TailwindObjectFit::Cover.boxed(),
            ["fill"] => TailwindObjectFit::Fill.boxed(),
            ["none"] => TailwindObjectFit::None.boxed(),
            ["scale", "down"] => TailwindObjectFit::ScaleDown.boxed(),
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

    #[inline]
    fn font_adaptor(str: &[&str], _arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
        let out = match str {
            // https://tailwindcss.com/docs/float
            [s @ ("sans" | "serif" | "mono")] => TailwindFontFamily::new(s).boxed(),
            // https://tailwindcss.com/docs/font-size
            ["xs"] => TailwindFontSize::new(0.75, 1.0).boxed(),
            ["sm"] => TailwindFontSize::new(0.75, 1.0).boxed(),
            ["md"] | ["base"] => TailwindFontSize::new(0.75, 1.0).boxed(),
            ["lg"] => TailwindFontSize::new(0.75, 1.0).boxed(),
            ["xl"] => TailwindFontSize::new(0.75, 1.0).boxed(),
            ["2xl"] => TailwindFontSize::new(0.75, 1.0).boxed(),
            ["3xl"] => TailwindFontSize::new(0.75, 1.0).boxed(),
            ["4xl"] => TailwindFontSize::new(0.75, 1.0).boxed(),
            ["5xl"] => TailwindFontSize::new(0.75, 1.0).boxed(),
            ["6xl"] => TailwindFontSize::new(0.75, 1.0).boxed(),
            ["7xl"] => TailwindFontSize::new(0.75, 1.0).boxed(),
            ["8xl"] => TailwindFontSize::new(0.75, 1.0).boxed(),
            ["9xl"] => TailwindFontSize::new(0.75, 1.0).boxed(),
            // https://tailwindcss.com/docs/float
            ["thin"] => TailwindFontWeight::THIN.boxed(),
            ["extralight"] => TailwindFontWeight::EXTRA_LIGHT.boxed(),
            ["light"] => TailwindFontWeight::LIGHT.boxed(),
            ["normal"] => TailwindFontWeight::NORMAL.boxed(),
            ["medium"] => TailwindFontWeight::MEDIUM.boxed(),
            ["semibold"] => TailwindFontWeight::SEMI_BOLD.boxed(),
            ["bold"] => TailwindFontWeight::BOLD.boxed(),
            ["extrabold"] => TailwindFontWeight::EXTRA_BOLD.boxed(),
            ["black"] => TailwindFontWeight::BLACK.boxed(),
            _ => return syntax_error!("Unknown font instructions: {}", str.join("-")),
        };
        Ok(out)
    }
    #[inline]
    fn text_adaptor(str: &[&str], _arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
        let out = match str {
            // https://tailwindcss.com/docs/text-align
            ["left"] => TailwindTextAlignment::Left.boxed(),
            ["center"] => TailwindTextAlignment::Center.boxed(),
            ["right"] => TailwindTextAlignment::Right.boxed(),
            ["justify"] => TailwindTextAlignment::Justify.boxed(),
            // https://tailwindcss.com/docs/text-color
            // ["inherit"] => TailwindTextColor::INHERIT.boxed(),
            // ["current"] => TailwindTextColor::CURRENT.boxed(),
            // ["transparent"] => TailwindTextColor::TRANSPARENT.boxed(),
            // ["black"] => TailwindTextColor::BLACK.boxed(),
            // ["white"] => TailwindTextColor::WHITE.boxed(),
            // [color, weight] => TailwindTextColor::parse(color, weight)?.boxed(),
            _ => return syntax_error!("Unknown text instructions: {}", str.join("-")),
        };
        Ok(out)
    }
}
