use super::*;
use crate::systems::spaces::TailwindSpacing;
use nom::{
    bytes::complete::{take_till, take_till1},
    character::complete::{alphanumeric1, char, multispace0},
    multi::separated_list0,
    sequence::delimited,
};

mod display;
mod methods;
mod utils;

use self::utils::*;

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

/// `v:v:-a-a-[A]`
// #[derive(Clone)]
pub struct AstStyle {
    negative: bool,
    variants: Vec<AstVariant>,
    elements: Vec<AstElement>,
    arbitrary: Option<AstArbitrary>,
}

/// https://tailwindcss.com/docs/adding-custom-styles#using-arbitrary-values
///
/// `[.]`
#[derive(Debug)]
pub struct AstArbitrary(String);

#[derive(Debug)]
pub struct AstElement(String);

#[derive(Debug)]
pub struct AstVariant {
    not: bool,
    pseudo: bool,
    names: Vec<String>,
}

#[derive(Debug)]
pub enum AstGroup {
    Standalone { inner: AstStyle },
    Grouped { variants: Vec<AstVariant>, elements: Option<AstElement>, inner: Vec<AstStyle> },
}
