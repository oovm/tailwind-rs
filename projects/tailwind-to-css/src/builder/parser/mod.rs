mod display;
mod methods;
mod utils;

pub use self::utils::*;
use super::*;
use crate::{
    syntax_error, systems::borders::TailwindBorderStyle, TailwindBorderCollapse, TailwindBoxDecorationBreak, TailwindBoxSizing,
    TailwindClear, TailwindColumns, TailwindContainer, TailwindDisplay, TailwindFloat, TailwindFontSmoothing, TailwindHeight,
    TailwindIsolation, TailwindPosition, TailwindScreenReader, TailwindSpacing, TailwindVisibility, TailwindWidth,
};
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

impl TailwindInstance for AstStyle {
    fn id(&self) -> String {
        todo!()
    }
    fn selectors(&self, ctx: &TailwindBuilder) -> String {
        todo!()
    }
    fn attributes(&self, ctx: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let mut out = BTreeSet::default();
        out
    }
}

impl AstStyle {
    pub fn get_instance(&self) -> Result<Box<dyn TailwindInstance>> {
        let instance = match self.view_elements().as_slice() {
            // ["w", rest @ ..] => {TailW}
            // Layout System
            ["aspect", rest @ ..] => TailwindAspect::parse(rest)?.boxed(),
            ["container"] => TailwindContainer {}.boxed(),
            ["columns", rest @ ..] => TailwindColumns::parse(rest)?.boxed(),
            ["break", "before", rest @ ..] => TailwindBreak::parse_before(rest)?.boxed(),
            ["break", "inside", rest @ ..] => TailwindBreak::parse_inside(rest)?.boxed(),
            ["break", "after", rest @ ..] => TailwindBreak::parse_after(rest)?.boxed(),
            ["box", rest @ ..] => Self::box_adaptor(rest)?,
            ["block"] => TailwindDisplay::Block.boxed(),
            ["flex"] => TailwindDisplay::Flex.boxed(),
            // begin https://tailwindcss.com/docs/display
            ["inline", "flex"] => TailwindDisplay::InlineFlex.boxed(),
            ["inline", "block"] => TailwindDisplay::InlineBlock.boxed(),
            // end https://tailwindcss.com/docs/display
            ["float", rest @ ..] => Self::float_adaptor(rest)?,
            ["clear", rest @ ..] => TailwindClear::parse(rest)?.boxed(),

            ["isolate"] => TailwindIsolation::Isolate.boxed(),
            ["isolation", "auto"] => TailwindIsolation::Auto.boxed(),
            ["object", rest @ ..] => Self::object_adaptor(rest)?,
            ["overflow", rest @ ..] => Self::overflow_adaptor(rest)?,
            ["overscroll", rest @ ..] => Self::overscroll_adaptor(rest)?,
            // begin https://tailwindcss.com/docs/position
            ["static"] => TailwindPosition::InlineFlex.boxed(),
            ["fixed"] => TailwindPosition::InlineFlex.boxed(),
            ["absolute"] => TailwindPosition::InlineFlex.boxed(),
            ["relative"] => TailwindPosition::InlineFlex.boxed(),
            ["sticky"] => TailwindPosition::InlineFlex.boxed(),
            // end https://tailwindcss.com/docs/position
            ["visible"] => TailwindVisibility::Visible.boxed(),
            ["invisible"] => TailwindVisibility::Invisible.boxed(),

            ["inset", rest @ ..] => TailWindZIndex::parse(rest, self.negative),

            ["s", rest @ ..] => TailWindZIndex::parse(rest, self.negative),
            // Spacing System
            [p @ ("p" | "pl" | "pr" | "pm" | "pt" | "px" | "py"), rest @ ..] => TailwindSpacing::parse_padding(rest, p),
            [m @ ("m" | "ml" | "mr" | "mm" | "mt" | "mx" | "my"), rest @ ..] => TailwindSpacing::parse_margin(rest, m),
            ["space", "x", rest @ ..] => TailwindSpacing::parse_space(rest, 'x'),
            ["space", "y", rest @ ..] => TailwindSpacing::parse_space(rest, 'y'),
            // Sizing System
            ["w", rest @ ..] => TailwindWidth::parse(rest, "normal"),
            ["min", "w", rest @ ..] => TailwindWidth::parse(rest, "min"),
            ["max", "w", rest @ ..] => TailwindWidth::parse(rest, "max"),
            ["h", rest @ ..] => TailwindHeight::parse(rest, "normal"),
            ["min", "h", rest @ ..] => TailwindHeight::parse(rest, "min"),
            ["max", "h", rest @ ..] => TailwindHeight::parse(rest, "max"),
            // Typography System
            ["font", family @ ("sans" | "serif" | "mono")] => todo!(),
            ["text", size] => todo!(),
            ["text", size] => todo!(),
            ["antialiased"] => TailwindFontSmoothing::new(false).boxed(),
            ["subpixel", "antialiased"] => TailwindFontSmoothing::new(true).boxed(),
            ["italic"] => todo!(),
            ["not", "italic"] => todo!(),
            ["font", weight @ ("thin" | "extralight" | "light")] => todo!(),
            // TODO:https://tailwindcss.com/docs/font-variant-numeric
            ["tracking", rest @ ..] => todo!(),
            ["leading", rest @ ..] => todo!(),
            ["list", rest @ ..] => todo!(),
            ["font", alignment @ ("text" | "serif" | "mono")] => todo!(),
            // TODO:https://tailwindcss.com/docs/font-variant-numeric
            ["underline", rest @ ..] => todo!(),
            ["decoration", rest @ ..] => todo!(),
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
            ["divide", rest @ ..] => Self::divide_adaptor(rest)?,
            ["outline", rest @ ..] => Self::outline_adaptor(rest)?,
            ["ring", rest @ ..] => Self::ring_adaptor(rest)?,
            // Effects System
            ["shadow", rest @ ..] => Self::shadow_adaptor(rest)?,
            ["opacity", rest @ ..] => todo!(),
            ["mix", "blend", rest @ ..] => todo!(),
            // Filters System
            ["blur", rest @ ..] => todo!(),
            ["brightness", rest @ ..] => todo!(),
            ["contrast", rest @ ..] => todo!(),

            ["drop", "shadow", rest @ ..] => todo!(),

            ["grayscale", rest @ ..] => todo!(),
            ["backdrop", rest @ ..] => todo!(),
            // Tables System
            ["table", rest @ ..] => Self::table_adaptor(rest)?,
            // Transitions System
            ["transition", rest @ ..] => todo!(),
            ["duration", rest @ ..] => todo!(),
            ["ease", rest @ ..] => todo!(),
            ["delay", rest @ ..] => todo!(),

            ["animate", rest @ ..] => todo!(),
            ["ease", rest @ ..] => todo!(),
            ["delay", rest @ ..] => todo!(),
            // Transforms System
            ["scale", "x", rest @ ..] => todo!(),
            ["scale", "y", rest @ ..] => todo!(),
            ["scale", rest @ ..] => todo!(),
            ["rotate", rest @ ..] => todo!(),
            ["translate", rest @ ..] => todo!(),
            ["rotate", rest @ ..] => todo!(),
            ["skew", "x", rest @ ..] => todo!(),
            ["skew", "y", rest @ ..] => todo!(),
            ["origin", rest @ ..] => todo!(),
            // Interactivity System
            ["accent", rest @ ..] => todo!(),

            // SVG System
            ["svg", rest @ ..] => todo!(),
            // Accessibility System
            ["sr", "only"] => TailwindScreenReader::new(true).boxed(),
            ["not", "sr", "only"] => TailwindScreenReader::new(false).boxed(),
            // Form System Extension
            _ => panic!("Unknown tailwind system"),
        };
        Ok(instance)
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

            _ => return syntax_error!(""),
        };
        Ok(out)
    }
    #[inline]
    fn divide_adaptor(str: &[&str]) -> Result<Box<dyn TailwindInstance>> {
        let out = match str {
            // https://tailwindcss.com/docs/float
            _ => return syntax_error!(""),
        };
        Ok(out)
    }
    #[inline]
    fn outline_adaptor(str: &[&str]) -> Result<Box<dyn TailwindInstance>> {
        let out = match str {
            // https://tailwindcss.com/docs/float
            _ => return syntax_error!(""),
        };
        Ok(out)
    }
    #[inline]
    fn ring_adaptor(str: &[&str]) -> Result<Box<dyn TailwindInstance>> {
        let out = match str {
            // https://tailwindcss.com/docs/float
            _ => return syntax_error!(""),
        };
        Ok(out)
    }
    #[inline]
    fn shadow_adaptor(str: &[&str]) -> Result<Box<dyn TailwindInstance>> {
        let out = match str {
            // https://tailwindcss.com/docs/float
            _ => return syntax_error!(""),
        };
        Ok(out)
    }
    #[inline]
    fn box_adaptor(str: &[&str]) -> Result<Box<dyn TailwindInstance>> {
        let out = match str {
            ["decoration", "clone"] => TailwindBoxDecorationBreak::Clone.boxed(),
            ["decoration", "slice"] => TailwindBoxDecorationBreak::Clone.boxed(),
            ["border"] => TailwindBoxSizing::Border.boxed(),
            ["content"] => TailwindBoxSizing::Content.boxed(),
            _ => return syntax_error!(""),
        };
        Ok(out)
    }
    #[inline]
    fn table_adaptor(str: &[&str]) -> Result<Box<dyn TailwindInstance>> {
        let out = match str {
            // https://tailwindcss.com/docs/display#table
            [] => TailwindDisplay::Table.boxed(),
            ["caption"] => TailwindFloat::Left.boxed(),
            ["right"] => TailwindFloat::Right.boxed(),
            ["none"] => TailwindFloat::None.boxed(),
            // https://tailwindcss.com/docs/table-layout
            ["auto"] => TailwindTableLayout::Auto.boxed(),
            ["fixed"] => TailwindTableLayout::Fixed.boxed(),

            _ => return syntax_error!(""),
        };
        Ok(out)
    }
    #[inline]
    fn float_adaptor(str: &[&str]) -> Result<Box<dyn TailwindInstance>> {
        let out = match str {
            // https://tailwindcss.com/docs/float
            ["left"] => TailwindFloat::Left.boxed(),
            ["right"] => TailwindFloat::Right.boxed(),
            ["none"] => TailwindFloat::None.boxed(),
            _ => return syntax_error!(""),
        };
        Ok(out)
    }

    #[inline]
    fn object_adaptor(str: &[&str]) -> Result<Box<dyn TailwindInstance>> {
        let out = match str {
            // https://tailwindcss.com/docs/float
            _ => return syntax_error!(""),
        };
        Ok(out)
    }
    #[inline]
    fn overflow_adaptor(str: &[&str]) -> Result<Box<dyn TailwindInstance>> {
        let out = match str {
            // https://tailwindcss.com/docs/float
            _ => return syntax_error!(""),
        };
        Ok(out)
    }
    #[inline]
    fn overscroll_adaptor(str: &[&str]) -> Result<Box<dyn TailwindInstance>> {
        let out = match str {
            // https://tailwindcss.com/docs/float
            _ => return syntax_error!(""),
        };
        Ok(out)
    }
}

impl TailwindBuilder {
    /// `(item (WS/NL item)*)?`
    pub(crate) fn parse(&self, input: &str) -> Result<BTreeSet<Box<dyn TailwindInstance>>> {
        todo!()
    }
}
