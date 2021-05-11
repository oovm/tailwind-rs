use super::*;

impl TailwindAspect {
    #[inline]
    fn instance(kind: &'static str, ratio: &'static str) -> Box<dyn TailwindInstance> {
        Box::new(Self { kind, ratio })
    }
    pub fn parser<'a>() -> impl FnMut(&'a str) -> ParsedList<'a> {
        move |input| {
            as_list(alt((
                // https://tailwindcss.com/docs/aspect-ratio
                Self::parser_kind("auto", "auto"),
                Self::parser_kind("square", "1 / 1"),
                Self::parser_kind("video", "16 / 9"),
            ))(input))
        }
    }
    fn parser_kind<'a>(kind: &'static str, ratio: &'static str) -> impl FnMut(&'a str) -> ParsedItem<'a> {
        let id = format!("aspect-{}", kind);
        move |input| match tag(id.as_str())(input) {
            Ok(o) => Ok((o.0, Self::instance(kind, ratio))),
            Err(e) => Err(e),
        }
    }
}

impl TailwindBreak {
    pub fn parser<'a>() -> impl FnMut(&'a str) -> ParsedList<'a> {
        move |input| {
            as_list(alt((
                // https://tailwindcss.com/docs/break-before
                TailwindBreak::parser_before("auto"),
                TailwindBreak::parser_before("avoid"),
                TailwindBreak::parser_before("all"),
                TailwindBreak::parser_before("avoid-page"),
                TailwindBreak::parser_before("page"),
                TailwindBreak::parser_before("left"),
                TailwindBreak::parser_before("right"),
                TailwindBreak::parser_before("column"),
                // https://tailwindcss.com/docs/break-after
                TailwindBreak::parser_after("auto"),
                TailwindBreak::parser_after("avoid"),
                TailwindBreak::parser_after("all"),
                TailwindBreak::parser_after("avoid-page"),
                TailwindBreak::parser_after("page"),
                TailwindBreak::parser_after("left"),
                TailwindBreak::parser_after("right"),
                TailwindBreak::parser_after("column"),
                // https://tailwindcss.com/docs/break-inside
                TailwindBreak::parser_inside("auto"),
                TailwindBreak::parser_inside("avoid"),
                TailwindBreak::parser_inside("avoid-page"),
                TailwindBreak::parser_inside("avoid-column"),
            ))(input))
        }
    }

    fn parser_before<'a>(kind: &'static str) -> impl FnMut(&'a str) -> ParsedItem {
        let id = format!("break-before-{}", kind);
        move |input| match tag(id.as_str())(input) {
            Ok(o) => Ok((o.0, Box::new(Self::Before(kind)))),
            Err(e) => Err(e),
        }
    }
    fn parser_after<'a>(kind: &'static str) -> impl FnMut(&'a str) -> ParsedItem {
        let id = format!("break-after-{}", kind);
        move |input| match tag(id.as_str())(input) {
            Ok(o) => Ok((o.0, Box::new(Self::After(kind)))),
            Err(e) => Err(e),
        }
    }
    fn parser_inside<'a>(kind: &'static str) -> impl FnMut(&'a str) -> ParsedItem {
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

    pub fn parser<'a>() -> impl FnMut(&'a str) -> ParsedList {
        move |input| as_list(alt((Self::parse_number, Self::parse_auto))(input))
    }

    fn parse_number(input: &str) -> ParsedItem {
        match tuple((opt(tag("-")), tag("z"), tag("-"), map_res(recognize(digit1), str::parse)))(input) {
            Ok((s, (Some(_), _, _, n))) => Ok((s, Self::number(n, true))),
            Ok((s, (None, _, _, n))) => Ok((s, Self::number(n, false))),
            Err(e) => Err(e),
        }
    }
    fn parse_auto(input: &str) -> ParsedItem {
        match tuple((tag("z"), tag("-"), tag("auto")))(input) {
            Ok((s, _)) => Ok((s, Self::auto())),
            Err(e) => Err(e),
        }
    }
}
