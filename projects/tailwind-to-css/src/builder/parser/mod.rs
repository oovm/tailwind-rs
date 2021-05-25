mod display;
mod methods;
mod utils;

use self::utils::*;
use super::*;
use crate::systems::spaces::TailwindSpacing;
use std::{
    fmt::{Display, Formatter, Write},
    mem::swap,
    str::FromStr,
};
use tailwind_error::nom::{
    bytes::complete::{take_till, take_till1},
    character::complete::{alphanumeric1, char, multispace0, multispace1},
    combinator::{map_res, recognize},
    multi::{separated_list0, separated_list1},
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
    Standalone { inner: AstStyle },
    Grouped { variants: Vec<AstVariant>, elements: Option<AstElement>, inner: Vec<AstStyle> },
}

/// https://github.com/tw-in-js/twind/blob/main/src/twind/variants.ts
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
        match self.view_elements().as_slice() {
            ["w", rest @ ..] => {}
            _ => panic!("unknown tailwind style"),
        }
        out
    }
}

impl TailwindBuilder {
    /// `(item (WS/NL item)*)?`
    pub(crate) fn parse(&self, input: &str) -> Result<BTreeSet<Box<dyn TailwindInstance>>> {
        let mut out = BTreeSet::new();
        // FIXME: stupid code !!!
        let item0 = alt((
            //
            self.maybe_layout_system(),
            self.maybe_spacing_system(),
            self.maybe_table_system(),
        ));
        let item = alt((
            //
            self.maybe_layout_system(),
            self.maybe_spacing_system(),
            self.maybe_table_system(),
        ));
        match opt(tuple((item0, many0(tuple((multispace0, item))))))(input.trim()) {
            Ok((_, None)) => {}
            Ok((_, Some((head, rest)))) => {
                out.extend(head.into_iter());
                for (_, items) in rest {
                    out.extend(items.into_iter());
                }
            }
            Err(_) => todo!(),
        };
        return Ok(out);
    }

    fn maybe_layout_system<'a>(&self) -> impl FnMut(&'a str) -> ParsedList<'a> {
        alt((
            //
            TailwindAspect::parser(),
            TailwindBreak::parser(),
            TailWindZIndex::parser(),
        ))
    }

    fn maybe_table_system<'a>(&self) -> impl FnMut(&'a str) -> ParsedList<'a> {
        alt((TailwindBorderCollapse::parser(), TailwindTableLayout::parser()))
    }
    fn maybe_spacing_system<'a>(&self) -> impl FnMut(&'a str) -> ParsedList<'a> {
        TailwindSpacing::parser()
    }
}
