use super::*;
use nom::{branch::alt, IResult};

impl TailwindAspect {
    #[inline]
    pub fn new(kind: &'static str, ratio: &'static str) -> Box<dyn TailwindInstance> {
        Box::new(Self { kind, ratio })
    }
    pub fn parser<'a>(
        kind: &'static str,
        ratio: &'static str,
    ) -> impl Fn(&'a str) -> IResult<&'a str, Box<dyn TailwindInstance>> {
        let id = format!("aspect-{}", kind);
        move |input| match tag(id.as_str())(input) {
            Ok(o) => Ok((o.0, Self::new(kind, ratio))),
            Err(e) => Err(e),
        }
    }
}

impl TailwindBreak {
    pub fn parser<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, Box<dyn TailwindInstance>> {
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

    pub fn parser_before<'a>(kind: &'static str) -> impl Fn(&'a str) -> IResult<&'a str, Box<dyn TailwindInstance>> {
        let id = format!("break-before-{}", kind);
        move |input| match tag(id.as_str())(input) {
            Ok(o) => Ok((o.0, Box::new(Self::Before(kind)))),
            Err(e) => Err(e),
        }
    }
    pub fn parser_after<'a>(kind: &'static str) -> impl Fn(&'a str) -> IResult<&'a str, Box<dyn TailwindInstance>> {
        let id = format!("break-after-{}", kind);
        move |input| match tag(id.as_str())(input) {
            Ok(o) => Ok((o.0, Box::new(Self::After(kind)))),
            Err(e) => Err(e),
        }
    }
    pub fn parser_inside<'a>(kind: &'static str) -> impl Fn(&'a str) -> IResult<&'a str, Box<dyn TailwindInstance>> {
        let id = format!("break-inside-{}", kind);
        move |input| match tag(id.as_str())(input) {
            Ok(o) => Ok((o.0, Box::new(Self::Inside(kind)))),
            Err(e) => Err(e),
        }
    }
}
