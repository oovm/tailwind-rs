mod display;
mod methods;
mod utils;

pub use self::utils::*;
use super::*;
use crate::{
    systems::{
        accessibility::TailwindScreenReader, borders::TailwindBorderStyle, flexbox::TailwindFlexBasis, spaces::TailwindSpacing,
        typography::TailwindFontSmoothing,
    },
    TailwindBorderCollapse, TailwindHeight, TailwindWidth,
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
        let instance = match self.view_elements().as_slice() {
            // ["w", rest @ ..] => {TailW}
            // Layout System
            ["aspect", rest @ ..] => TailwindAspect::parse(rest),
            ["container"] => todo!(),
            ["columns", rest @ ..] => todo!(),
            ["box", rest @ ..] => Self::box_adaptor(rest),
            ["block"] =>

            ["break", "before", rest @ ..] => TailwindBreak::parse_before(rest),
            ["break", "inside", rest @ ..] => TailwindBreak::parse_inside(rest),
            ["break", "after", rest @ ..] => TailwindBreak::parse_after(rest),
            ["z", rest @ ..] => TailWindZIndex::parse(rest, self.negative),

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
            ["antialiased"] => TailwindFontSmoothing::parse(false),
            ["subpixel", "antialiased"] => TailwindFontSmoothing::parse(true),
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
            ["border", rest @ ..] => Self::border_adaptor(rest),
            ["divide", rest @ ..] => Self::divide_adaptor(rest),
            ["outline", rest @ ..] => Self::outline_adaptor(rest),
            ["ring", rest @ ..] => Self::ring_adaptor(rest),
            // Effects System
            ["shadow", rest @ ..] => Self::shadow_adaptor(rest),
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
            ["table", rest @ ..] => TailwindTableLayout::parse(rest),
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
            ["sr", "only"] => TailwindScreenReader::parse(true),
            ["not", "sr", "only"] => TailwindScreenReader::parse(false),
            // Form System Extension
            _ => panic!("Unknown tailwind system"),
        };
        out
    }
}

impl AstStyle {
    pub fn border_adaptor(str: &[&str]) -> Box<dyn TailwindInstance> {
        match str {
            // border
            [] => TailwindBorderWidth::parse(),

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

            _ => panic!("todo"),
        }
    }
    pub fn divide_adaptor(str: &[&str]) -> Box<dyn TailwindInstance> {
        todo!()
    }

    pub fn outline_adaptor(str: &[&str]) -> Box<dyn TailwindInstance> {
        todo!()
    }

    pub fn ring_adaptor(str: &[&str]) -> Box<dyn TailwindInstance> {
        todo!()
    }

    pub fn shadow_adaptor(str: &[&str]) -> Box<dyn TailwindInstance> {
        todo!()
    }
    pub fn box_adaptor(str: &[&str]) -> Box<dyn TailwindInstance> {
        todo!()
    }
}

impl TailwindBuilder {
    /// `(item (WS/NL item)*)?`
    pub(crate) fn parse(&self, input: &str) -> Result<BTreeSet<Box<dyn TailwindInstance>>> {
        todo!()
    }
}
