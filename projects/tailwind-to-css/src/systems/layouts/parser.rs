use super::*;
use nom::{
    branch::alt,
    character::complete::digit1,
    combinator::{map_res, opt, recognize},
    sequence::tuple,
};

impl TailwindAspect {
    #[inline]
    pub fn new(kind: &'static str, ratio: &'static str) -> Box<dyn TailwindInstance> {
        Box::new(Self { kind, ratio })
    }
    pub fn parser<'a>(kind: &'static str, ratio: &'static str) -> impl Fn(&'a str) -> TailwindParsed {
        let id = format!("aspect-{}", kind);
        move |input| match tag(id.as_str())(input) {
            Ok(o) => Ok((o.0, Self::new(kind, ratio))),
            Err(e) => Err(e),
        }
    }
}

impl TailwindBreak {
    pub fn parser<'a>() -> impl FnMut(&'a str) -> TailwindParsed {
        alt((
            Self::parser_before("auto"),
            Self::parser_before("avoid"),
            Self::parser_before("all"),
            Self::parser_before("avoid-page"),
            Self::parser_before("page"),
            Self::parser_before("left"),
            Self::parser_before("right"),
            Self::parser_before("column"),
            Self::parser_after("auto"),
            Self::parser_after("avoid"),
            Self::parser_after("all"),
            Self::parser_after("avoid-page"),
            Self::parser_after("page"),
            Self::parser_after("left"),
            Self::parser_after("right"),
            Self::parser_after("column"),
            Self::parser_inside("auto"),
            Self::parser_inside("avoid"),
            Self::parser_inside("avoid-page"),
            Self::parser_inside("avoid-column"),
        ))
    }

    pub fn parser_before<'a>(kind: &'static str) -> impl Fn(&'a str) -> TailwindParsed {
        let id = format!("break-before-{}", kind);
        move |input| match tag(id.as_str())(input) {
            Ok(o) => Ok((o.0, Box::new(Self::Before(kind)))),
            Err(e) => Err(e),
        }
    }
    pub fn parser_after<'a>(kind: &'static str) -> impl Fn(&'a str) -> TailwindParsed {
        let id = format!("break-after-{}", kind);
        move |input| match tag(id.as_str())(input) {
            Ok(o) => Ok((o.0, Box::new(Self::After(kind)))),
            Err(e) => Err(e),
        }
    }
    pub fn parser_inside<'a>(kind: &'static str) -> impl Fn(&'a str) -> TailwindParsed {
        let id = format!("break-inside-{}", kind);
        move |input| match tag(id.as_str())(input) {
            Ok(o) => Ok((o.0, Box::new(Self::Inside(kind)))),
            Err(e) => Err(e),
        }
    }
}

impl TailWindZIndex {
    #[inline]
    fn number(n: usize, negative: bool) -> Box<dyn TailwindInstance> {
        match negative {
            true => Box::new(Self::Negative(n)),
            false => Box::new(Self::Positive(n)),
        }
    }
    #[inline]
    fn auto() -> Box<dyn TailwindInstance> {
        Box::new(Self::Auto)
    }
    pub fn parser<'a>() -> impl FnMut(&'a str) -> TailwindParsed {
        alt((Self::parse_number, Self::parse_auto))
    }

    pub fn parse_number(input: &str) -> TailwindParsed {
        match tuple((
            //
            opt(tag("-")),
            tag("z"),
            tag("-"),
            map_res(recognize(digit1), str::parse),
        ))(input)
        {
            Ok((s, (Some(_), _, _, n))) => Ok((s, Self::number(n, true))),
            Ok((s, (None, _, _, n))) => Ok((s, Self::number(n, false))),
            Err(e) => Err(e),
        }
    }
    pub fn parse_auto(input: &str) -> TailwindParsed {
        match tuple((
            //
            tag("z"),
            tag("-"),
            tag("auto"),
        ))(input)
        {
            Ok((s, _)) => Ok((s, Self::auto())),
            Err(e) => Err(e),
        }
    }
}

#[test]
fn test() {
    let p = TailWindZIndex::parser()("-z-1");
    println!("{:#?}", p)
}
