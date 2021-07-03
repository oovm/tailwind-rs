mod display;
mod methods;
mod utils;

pub use self::utils::*;
use super::*;
use crate::{
    syntax_error,
    systems::{
        borders::TailwindBorderStyle,
        filters::TailwindBrightness,
        flexbox::{TailWindGrow, TailWindOrder, TailWindShrink, TailwindFlex, TailwindFlexDirection, TailwindFlexWrap},
    },
    TailwindBorderCollapse, TailwindBoxDecorationBreak, TailwindBoxSizing, TailwindBreak, TailwindClear, TailwindColumns,
    TailwindContainer, TailwindContent, TailwindDisplay, TailwindDivideStyle, TailwindFloat, TailwindFontFamily,
    TailwindFontSize, TailwindFontSmoothing, TailwindFontStyle, TailwindFontVariantNumeric, TailwindFontWeight,
    TailwindIsolation, TailwindLayoutBreak, TailwindLeading, TailwindListStyle, TailwindListStylePosition, TailwindObjectFit,
    TailwindObjectPosition, TailwindOutlineStyle, TailwindOverflow, TailwindOverscroll, TailwindPosition,
    TailwindRingOffsetWidth, TailwindRotate, TailwindScale, TailwindScreenReader, TailwindShadow, TailwindSizing, TailwindSkew,
    TailwindSpacing, TailwindTextAlignment, TailwindTextColor, TailwindTextDecoration, TailwindTextOverflow,
    TailwindTextTransform, TailwindTracking, TailwindUnderlineOffset, TailwindVisibility,
};
use log::error;
use std::{
    fmt::{Display, Formatter, Write},
    str::FromStr,
};
use tailwind_error::nom::{
    bytes::complete::{take_till, take_till1},
    character::complete::{alphanumeric1, char, multispace1},
    combinator::{map_res, recognize},
    multi::separated_list0,
    sequence::delimited,
};

/// `v:v:-a-a-[A]`
#[derive(Clone)]
pub struct AstStyle {
    negative: bool,
    variants: Vec<AstVariant>,
    elements: Vec<AstElement>,
    arbitrary: Option<AstArbitrary>,
}

#[derive(Debug, Clone)]
pub enum AstGroup {
    Standalone {
        ///
        inner: AstStyle,
    },
    Grouped {
        ///
        variants: Vec<AstVariant>,
        elements: Option<AstElement>,
        inner: Vec<AstStyle>,
    },
}

/// https://github.com/tw-in-js/twind/blob/main/src/twind/variants.ts
#[derive(Debug)]
pub enum AstVariantKind {
    Dark,
    Sticky,
    MotionReduce,
    MotionSafe,
    First,
    Last,
    Even,
    Odd,
    Children,
    Siblings,
    Sibling,
    Override,
}

/// https://tailwindcss.com/docs/adding-custom-styles#using-arbitrary-values
///
/// `[.]`
#[derive(Debug, Clone)]
pub struct AstArbitrary(String);

#[derive(Debug, Clone)]
pub struct AstElement(String);

#[derive(Debug, Clone)]
pub struct AstVariant {
    not: bool,
    pseudo: bool,
    names: Vec<String>,
}

impl Display for AstStyle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for AstStyle {
    fn attributes(&self, ctx: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let mut out = BTreeSet::default();
        match self.get_instance() {
            Ok(o) => out.extend(o.attributes(ctx)),
            Err(e) => {
                #[cfg(debug_assertions)]
                error!("{:?}", e)
            }
        }
        out
    }
}
// noinspection SpellCheckingInspection
impl AstStyle {
    #[inline(never)]
    pub fn get_instance(&self) -> Result<Box<dyn TailwindInstance>> {
        let element = self.view_elements();
        let arbitrary = self.view_arbitrary();
        let instance = match element.as_slice() {
            // Layout System
            ["aspect", rest @ ..] => TailwindAspect::parse(rest, arbitrary)?.boxed(),
            ["container"] => TailwindContainer {}.boxed(),
            ["columns", rest @ ..] => TailwindColumns::parse(rest)?.boxed(),
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
            ["float", rest @ ..] => Self::float_adaptor(rest, arbitrary)?,
            ["clear", rest @ ..] => TailwindClear::parse(rest)?.boxed(),
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
            ["inset", rest @ ..] => TailWindZIndex::parse(rest, self.negative),
            ["top", rest @ ..] => TailWindZIndex::parse(rest, self.negative),
            ["right", rest @ ..] => TailWindZIndex::parse(rest, self.negative),
            ["buttom", rest @ ..] => TailWindZIndex::parse(rest, self.negative),
            ["left", rest @ ..] => TailWindZIndex::parse(rest, self.negative),
            // https://tailwindcss.com/docs/visibility
            ["visible"] => TailwindVisibility::Visible.boxed(),
            ["invisible"] => TailwindVisibility::Invisible.boxed(),
            // https://tailwindcss.com/docs/z-index
            ["z", rest @ ..] => TailWindZIndex::parse(rest, self.negative),
            // Flexbox & Grid
            ["basis", rest @ ..] => todo!(),
            ["flex", rest @ ..] => Self::flex_adaptor(rest, arbitrary)?,
            ["grow", rest @ ..] => TailWindGrow::parse(rest, arbitrary)?.boxed(),
            ["shrink", rest @ ..] => TailWindShrink::parse(rest, arbitrary)?.boxed(),
            ["order", rest @ ..] => TailWindOrder::parse(rest, arbitrary, self.negative)?.boxed(),
            ["grid", rest @ ..] => Self::grid_adaptor(rest, arbitrary)?,
            // https://tailwindcss.com/docs/grid-column
            ["col", rest @ ..] => todo!(),
            ["row", rest @ ..] => todo!(),
            ["auto", rest @ ..] => todo!(),
            ["gap", rest @ ..] => todo!(),
            ["justify", rest @ ..] => Self::justify_adaptor(rest, arbitrary)?,
            ["content", rest @ ..] => todo!(),
            ["items", rest @ ..] => todo!(),
            ["self", rest @ ..] => todo!(),
            ["justify", rest @ ..] => Self::place_adaptor(rest, arbitrary)?,
            // Spacing System
            [p @ ("p" | "pl" | "pr" | "pm" | "pt" | "px" | "py"), rest @ ..] => {
                TailwindSpacing::parse_padding(rest, p, arbitrary)?.boxed()
            }
            [m @ ("m" | "ml" | "mr" | "mm" | "mt" | "mx" | "my"), rest @ ..] => {
                TailwindSpacing::parse_margin(rest, m, arbitrary)?.boxed()
            }
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
            ["decoration", rest @ ..] => Self::decoration_adaptor(rest, arbitrary)?,
            ["underline", "offset", rest @ ..] => TailwindUnderlineOffset::parse(rest, arbitrary)?.boxed(),
            // https://tailwindcss.com/docs/text-transform
            ["uppercase"] => TailwindTextTransform::Uppercase.boxed(),
            ["lowercase"] => TailwindTextTransform::Lowercase.boxed(),
            ["capitalize"] => TailwindTextTransform::Capitalize.boxed(),
            ["normal", "case"] => TailwindTextTransform::None.boxed(),
            // https://tailwindcss.com/docs/text-overflow
            ["truncate"] => TailwindTextOverflow::Truncate.boxed(),
            ["indent", rest @ ..] => todo!(),
            ["align", rest @ ..] => todo!(),
            ["whitespace", rest @ ..] => todo!(),
            ["break", rest @ ..] => todo!(),
            ["content", "none"] => TailwindContent::None.boxed(),
            // Typography System Extension
            ["prose"] => todo!(),
            // Backgrounds System
            // FIXME: https://tailwindcss.com/docs/background-blend-mode
            ["bg", rest @ ..] => todo!(),
            ["from", rest @ ..] => todo!(),
            ["via", rest @ ..] => todo!(),
            ["to", rest @ ..] => todo!(),
            // Borders System
            ["rounded", rest @ ..] => todo!(),
            ["border", rest @ ..] => Self::border_adaptor(rest)?,
            ["divide", rest @ ..] => Self::divide_adaptor(rest, arbitrary)?,
            ["outline", rest @ ..] => Self::outline_adaptor(rest, arbitrary)?,
            ["ring", rest @ ..] => Self::ring_adaptor(rest, arbitrary)?,
            // Effects System
            ["shadow", rest @ ..] => Self::shadow_adaptor(rest, arbitrary)?,
            ["opacity", rest @ ..] => todo!(),
            ["mix", "blend", rest @ ..] => todo!(),
            // Filters System
            ["blur", rest @ ..] => todo!(),
            ["brightness", rest @ ..] => TailwindBrightness::parse(rest, arbitrary)?.boxed(),
            ["contrast", rest @ ..] => todo!(),
            ["drop", "shadow", rest @ ..] => TailwindShadow::parse_drop(rest, arbitrary)?.boxed(),
            ["grayscale", rest @ ..] => todo!(),
            ["hue", "rotate", rest @ ..] => todo!(),
            ["invert", rest @ ..] => todo!(),
            ["saturate", rest @ ..] => todo!(),
            ["sepia", rest @ ..] => todo!(),
            ["backdrop", rest @ ..] => Self::backdrop_adaptor(rest, arbitrary)?,
            // Tables System
            ["table", rest @ ..] => Self::table_adaptor(rest, arbitrary)?,
            // Transitions System
            ["transition", rest @ ..] => todo!(),
            ["duration", rest @ ..] => todo!(),
            ["ease", rest @ ..] => todo!(),
            ["delay", rest @ ..] => todo!(),
            ["animate", rest @ ..] => todo!(),
            // Transforms System
            ["scale", "x", rest @ ..] => TailwindScale::parse(rest, arbitrary, Some(true))?.boxed(),
            ["scale", "y", rest @ ..] => TailwindScale::parse(rest, arbitrary, Some(false))?.boxed(),
            ["scale", rest @ ..] => TailwindScale::parse(rest, arbitrary, None)?.boxed(),
            ["rotate", rest @ ..] => TailwindRotate::parse(rest, arbitrary)?.boxed(),
            ["translate", "x", rest @ ..] => todo!(),
            ["translate", "y", rest @ ..] => todo!(),
            ["skew", "x", rest @ ..] => TailwindSkew::parse(rest, arbitrary, true)?.boxed(),
            ["skew", "y", rest @ ..] => TailwindSkew::parse(rest, arbitrary, false)?.boxed(),
            ["origin", rest @ ..] => todo!(),
            // Interactivity System
            ["accent", rest @ ..] => todo!(),
            // https://tailwindcss.com/docs/appearance
            ["appearance", "none"] => todo!(),
            ["cursor", rest @ ..] => todo!(),
            ["caret", rest @ ..] => todo!(),
            ["pointer", "events", rest @ ..] => todo!(),
            ["resize", rest @ ..] => todo!(),
            ["scroll", rest @ ..] => todo!(),
            ["snap", rest @ ..] => todo!(),
            ["touch", rest @ ..] => todo!(),
            ["select", rest @ ..] => todo!(),
            ["will", "change", rest @ ..] => todo!(),
            // SVG System
            ["fill", rest @ ..] => todo!(),
            ["stroke", rest @ ..] => todo!(),
            // Accessibility System
            ["sr", "only"] => TailwindScreenReader::new(true).boxed(),
            ["not", "sr", "only"] => TailwindScreenReader::new(false).boxed(),
            // Form System Extension
            _ => return syntax_error!("Unknown instructions: {} + [{}]", element.join("-"), arbitrary),
        };
        Ok(instance)
    }
    #[inline]
    fn break_adaptor(str: &[&str], _: &str) -> Result<Box<dyn TailwindInstance>> {
        let out = match str {
            // https://tailwindcss.com/docs/border-style
            ["normal"] => TailwindBreak::Normal.boxed(),
            ["words"] => TailwindBreak::Words.boxed(),
            ["all"] => TailwindBreak::All.boxed(),
            // https://tailwindcss.com/docs/break-before
            ["before", rest @ ..] => TailwindLayoutBreak::parse_before(rest)?.boxed(),
            // https://tailwindcss.com/docs/break-inside
            ["inside", rest @ ..] => TailwindLayoutBreak::parse_inside(rest)?.boxed(),
            // https://tailwindcss.com/docs/break-after
            ["after", rest @ ..] => TailwindLayoutBreak::parse_after(rest)?.boxed(),
            _ => return syntax_error!("Unknown break instructions: {}", str.join("-")),
        };
        Ok(out)
    }
    #[inline]
    fn border_adaptor(str: &[&str]) -> Result<Box<dyn TailwindInstance>> {
        let out = match str {
            // border
            // [] => TailwindBorderWidth::parse(),

            // https://tailwindcss.com/docs/border-style
            ["solid"] => TailwindBorderStyle::Solid.into_instance(),
            ["dashed"] => TailwindBorderStyle::Dashed.into_instance(),
            ["dotted"] => TailwindBorderStyle::Dotted.into_instance(),
            ["double"] => TailwindBorderStyle::Double.into_instance(),
            ["hidden"] => TailwindBorderStyle::Hidden.into_instance(),
            ["none"] => TailwindBorderStyle::None.into_instance(),
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
    fn divide_adaptor(str: &[&str], arbitrary: &str) -> Result<Box<dyn TailwindInstance>> {
        let out = match str {
            // https://tailwindcss.com/docs/divide-width
            ["x"] => todo!(),
            ["x", n] => todo!(),
            ["y"] => todo!(),
            ["y", n] => todo!(),
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
    fn outline_adaptor(str: &[&str], arbitrary: &str) -> Result<Box<dyn TailwindInstance>> {
        let out = match str {
            // https://tailwindcss.com/docs/outline-style
            [] => todo!(),
            ["none"] => TailwindOutlineStyle::None.boxed(),
            ["dashed"] => TailwindOutlineStyle::Dashed.boxed(),
            ["dotted"] => TailwindOutlineStyle::Dotted.boxed(),
            ["double"] => TailwindOutlineStyle::Double.boxed(),
            ["hidden"] => TailwindOutlineStyle::Hidden.boxed(),
            // https://tailwindcss.com/docs/outline-offset
            ["offset", n] => todo!(),
            // https://tailwindcss.com/docs/outline-width
            [n] => todo!(),
            _ => return syntax_error!("Unknown outline instructions: {}", str.join("-")),
        };
        Ok(out)
    }
    #[inline]
    fn ring_adaptor(str: &[&str], arbitrary: &str) -> Result<Box<dyn TailwindInstance>> {
        let out = match str {
            // https://tailwindcss.com/docs/ring-offset-width
            ["offset", rest @ ..] => TailwindRingOffsetWidth::parse(rest, arbitrary)?.boxed(),
            _ => return syntax_error!("Unknown ring instructions: {}", str.join("-")),
        };
        Ok(out)
    }
    #[inline]
    fn shadow_adaptor(str: &[&str], arbitrary: &str) -> Result<Box<dyn TailwindInstance>> {
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
    fn box_adaptor(str: &[&str], arbitrary: &str) -> Result<Box<dyn TailwindInstance>> {
        let out = match str {
            ["decoration", "clone"] => TailwindBoxDecorationBreak::Clone.boxed(),
            ["decoration", "slice"] => TailwindBoxDecorationBreak::Clone.boxed(),
            ["border"] => TailwindBoxSizing::Border.boxed(),
            ["content"] => TailwindBoxSizing::Content.boxed(),
            _ => return syntax_error!("Unknown box instructions: {}", str.join("-")),
        };
        Ok(out)
    }
    #[inline]
    fn flex_adaptor(str: &[&str], arbitrary: &str) -> Result<Box<dyn TailwindInstance>> {
        let out = match str {
            // https://tailwindcss.com/docs/display#flex
            [] if arbitrary.is_empty() => TailwindDisplay::Flex.boxed(),
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
            ["auto"] => TailwindBoxDecorationBreak::Clone.boxed(),
            ["initial"] => TailwindBoxDecorationBreak::Clone.boxed(),
            ["none"] => TailwindBoxDecorationBreak::Clone.boxed(),
            [n] => TailwindFlex::parse(n)?.boxed(),
            _ => return syntax_error!("Unknown box instructions: {}", str.join("-")),
        };
        Ok(out)
    }
    #[inline]
    fn grid_adaptor(str: &[&str], arbitrary: &str) -> Result<Box<dyn TailwindInstance>> {
        todo!()
    }
    #[inline]
    fn justify_adaptor(str: &[&str], arbitrary: &str) -> Result<Box<dyn TailwindInstance>> {
        todo!()
    }
    #[inline]
    fn place_adaptor(str: &[&str], arbitrary: &str) -> Result<Box<dyn TailwindInstance>> {
        todo!()
    }
    #[inline]
    fn list_adaptor(str: &[&str], arbitrary: &str) -> Result<Box<dyn TailwindInstance>> {
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
    fn decoration_adaptor(str: &[&str], arbitrary: &str) -> Result<Box<dyn TailwindInstance>> {
        todo!()
    }
    #[inline]
    fn backdrop_adaptor(str: &[&str], arbitrary: &str) -> Result<Box<dyn TailwindInstance>> {
        todo!()
    }
    #[inline]
    fn table_adaptor(str: &[&str], arbitrary: &str) -> Result<Box<dyn TailwindInstance>> {
        let out = match str {
            // https://tailwindcss.com/docs/display#table
            [] => TailwindDisplay::Table.boxed(),
            ["caption"] => TailwindFloat::Left.boxed(),
            ["right"] => TailwindFloat::Right.boxed(),
            ["none"] => TailwindFloat::None.boxed(),
            // https://tailwindcss.com/docs/table-layout
            ["auto"] => TailwindTableLayout::Auto.boxed(),
            ["fixed"] => TailwindTableLayout::Fixed.boxed(),
            _ => return syntax_error!("Unknown table instructions: {}", str.join("-")),
        };
        Ok(out)
    }
    #[inline]
    fn float_adaptor(str: &[&str], _: &str) -> Result<Box<dyn TailwindInstance>> {
        let out = match str {
            // https://tailwindcss.com/docs/float
            ["left"] => TailwindFloat::Left.boxed(),
            ["right"] => TailwindFloat::Right.boxed(),
            ["none"] => TailwindFloat::None.boxed(),
            _ => return syntax_error!("Unknown float instructions: {}", str.join("-")),
        };
        Ok(out)
    }

    #[inline]
    fn object_adaptor(str: &[&str], arbitrary: &str) -> Result<Box<dyn TailwindInstance>> {
        let out = match str {
            // https://tailwindcss.com/docs/object-fit
            ["contain"] => TailwindObjectFit::Contain.boxed(),
            ["cover"] => TailwindObjectFit::Cover.boxed(),
            ["fill"] => TailwindObjectFit::Fill.boxed(),
            ["none"] => TailwindObjectFit::None.boxed(),
            ["scale", "down"] => TailwindObjectFit::ScaleDown.boxed(),
            // https://tailwindcss.com/docs/object-position
            ["7"] | ["left", "top"] => TailwindObjectPosition::LeftTop.boxed(),
            ["8"] | ["top"] => TailwindObjectPosition::Bottom.boxed(),
            ["9"] | ["right", "top"] => TailwindObjectPosition::Bottom.boxed(),
            ["4"] | ["left"] => TailwindObjectPosition::Bottom.boxed(),
            ["5"] | ["center"] => TailwindObjectPosition::Bottom.boxed(),
            ["6"] | ["right"] => TailwindObjectPosition::Bottom.boxed(),
            ["1"] | ["left", "buttom"] => TailwindObjectPosition::Bottom.boxed(),
            ["2"] | ["buttom"] => TailwindObjectPosition::Bottom.boxed(),
            ["3"] | ["right", "buttom"] => TailwindObjectPosition::Bottom.boxed(),
            [] => TailwindObjectPosition::parse_arbitrary(arbitrary)?.boxed(),
            _ => return syntax_error!("Unknown object instructions: {}", str.join("-")),
        };
        Ok(out)
    }
    #[inline]
    fn overflow_adaptor(str: &[&str], _: &str) -> Result<Box<dyn TailwindInstance>> {
        let out = match str {
            // https://tailwindcss.com/docs/overflow
            ["x", pattern @ ..] => TailwindOverflow::parse_x(pattern)?.boxed(),
            ["y", pattern @ ..] => TailwindOverflow::parse_y(pattern)?.boxed(),
            _ => TailwindOverflow::parse_xy(str)?.boxed(),
        };
        Ok(out)
    }
    #[inline]
    fn overscroll_adaptor(str: &[&str], _: &str) -> Result<Box<dyn TailwindInstance>> {
        let out = match str {
            // https://tailwindcss.com/docs/overscroll-behavior
            ["x", pattern @ ..] => TailwindOverscroll::parse_x(pattern)?.boxed(),
            ["y", pattern @ ..] => TailwindOverscroll::parse_y(pattern)?.boxed(),
            _ => TailwindOverscroll::parse_xy(str)?.boxed(),
        };
        Ok(out)
    }

    #[inline]
    fn font_adaptor(str: &[&str], arbitrary: &str) -> Result<Box<dyn TailwindInstance>> {
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
    fn text_adaptor(str: &[&str], arbitrary: &str) -> Result<Box<dyn TailwindInstance>> {
        let out = match str {
            // https://tailwindcss.com/docs/text-align
            ["left"] => TailwindTextAlignment::Left.boxed(),
            ["center"] => TailwindTextAlignment::Center.boxed(),
            ["right"] => TailwindTextAlignment::Right.boxed(),
            ["justify"] => TailwindTextAlignment::Justify.boxed(),
            // https://tailwindcss.com/docs/text-color
            ["inherit"] => TailwindTextColor::INHERIT.boxed(),
            ["current"] => TailwindTextColor::CURRENT.boxed(),
            ["transparent"] => TailwindTextColor::TRANSPARENT.boxed(),
            ["black"] => TailwindTextColor::BLACK.boxed(),
            ["white"] => TailwindTextColor::WHITE.boxed(),
            [color, weight] => TailwindTextColor::parse(color, weight)?.boxed(),
            _ => return syntax_error!("Unknown text instructions: {}", str.join("-")),
        };
        Ok(out)
    }
}
