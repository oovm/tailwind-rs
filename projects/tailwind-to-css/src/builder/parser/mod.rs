use super::*;
use crate::systems::spaces::TailwindSpacing;
use nom::{
    character::complete::{alphanumeric1, multispace0},
    error::ErrorKind,
    multi::many1,
    AsChar, InputTakeAtPosition,
};

mod utils;

pub use self::utils::*;

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

#[derive(Debug)]
pub enum AstAtom<'a> {
    /// `&`
    SelfReference,
    /// `[.]`
    Arbitrary(&'a str),
    ///
    Normal(&'a str),
}

#[derive(Debug)]
pub struct AstElement<'a> {
    variants: Vec<AstVariant<'a>>,
    atoms: Vec<&'a str>,
}

#[derive(Debug)]
pub struct AstVariant<'a> {
    not: bool,
    names: &'a str,
}

pub struct AstGroup<'a> {
    variants: Vec<AstVariant<'a>>,
    inner: Vec<AstElement<'a>>,
}

impl<'a> AstVariant<'a> {
    pub fn parse(input: &'a str) -> IResult<&'a str, Self> {
        // alt((Self::parse_self, Self::parse_arbitrary, Self::parse_normal))(input)
        todo!()
    }
}

impl<'a> AstAtom<'a> {
    pub fn parse(input: &'a str) -> IResult<&'a str, Self> {
        alt((Self::parse_self, Self::parse_arbitrary, Self::parse_normal))(input)
    }
    fn parse_self(input: &'a str) -> IResult<&'a str, Self> {
        let (rest, _) = tag("&")(input)?;
        Ok((rest, Self::SelfReference))
    }
    fn parse_arbitrary(input: &'a str) -> IResult<&'a str, Self> {
        let (rest, _) = tag("&")(input)?;
        Ok((rest, Self::SelfReference))
    }
    fn parse_normal(input: &'a str) -> IResult<&'a str, Self> {
        fn check_char(c: char) -> bool {
            c.is_alphanum() || c == '/'
        }
        let (rest, s) = input.split_at_position1_complete(|item| !check_char(item), ErrorKind::RegexpMatch)?;
        Ok((rest, Self::Normal(s)))
    }
}

impl<'a> AstAtom<'a> {
    pub fn as_usize(&self) -> Option<usize> {
        match self {
            Self::Arbitrary(_) | Self::SelfReference => None,
            Self::Normal(s) => parse_integer(s).ok().map(|(_, o)| o),
        }
    }
    pub fn as_fraction(&self) -> Option<(usize, usize)> {
        match self {
            Self::Arbitrary(_) | Self::SelfReference => None,
            Self::Normal(s) => parse_fraction(s).ok().map(|(_, o)| o),
        }
    }
}

#[test]
fn test() {
    println!("{:?}", AstAtom::parse("200"))
}

// pub struct AstStyle {}
//
// /// a(& b())
// pub struct AstGroup {}
//

// impl AstVariant {
//     pub fn parser() {}
// }
