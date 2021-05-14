use super::*;
use crate::systems::spaces::TailwindSpacing;
use nom::character::complete::multispace0;

mod utils;

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

fn parse_text(input: &str) -> IResult<&str, (&str, &str)> {
    match tuple((tag("text"), tag("-"), alpha1, tag("-"), digit1))(input) {
        Ok(o) => Ok((o.0, (&o.1.2, &o.1.4))),
        Err(e) => Err(e),
    }
}

fn parse_width(input: &str) -> IResult<&str, (&str, &str)> {
    match tuple((tag("w"), tag("-"), alpha1, tag("-"), digit1))(input) {
        Ok(o) => Ok((o.0, (&o.1.2, &o.1.4))),
        Err(e) => Err(e),
    }
}

fn parse_height(input: &str) -> IResult<&str, (&str, &str)> {
    match tuple((tag("h"), tag("-"), alpha1, tag("-"), digit1))(input) {
        Ok(o) => Ok((o.0, (&o.1.2, &o.1.4))),
        Err(e) => Err(e),
    }
}
/// https://tailwindcss.com/docs/adding-custom-styles#using-arbitrary-values
///
/// ```regexp
/// \[[^\]\]
/// ```
pub fn parse_arbitrary() {}

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
    Custom(),
}

pub enum AstElement {
    /// `&`
    SelfReference,
    /// `[.]`
    Arbitrary,
    ///
    Normal,
}

impl AstElement {
    pub fn parser<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, T> {
        move |input| match opt() {}
    }

    pub fn as_usize(&self) -> Option<usize> {}
    pub fn as_fraction(&self) -> Option<(usize, usize)> {}
}

pub struct AstStyle {}

/// a(& b())
pub struct AstGroup {}

pub struct AstVariant<'a> {
    not: bool,
    names: Vec<&'a str>,
}

impl AstVariant {
    pub fn parser() {}
}
