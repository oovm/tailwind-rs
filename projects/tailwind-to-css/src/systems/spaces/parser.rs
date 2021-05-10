use nom::character::complete::one_of;
use super::*;

impl TailwindSpacing {
    pub fn auto(kind: TailwindSpacingKind) -> Self {
        Self {
            kind,
            size: TailwindSpacingSize::Auto,
        }
    }
    pub fn reversed(kind: TailwindSpacingKind) -> Self {
        Self {
            kind,
            size: TailwindSpacingSize::Reversed,
        }
    }
    pub fn px(n: usize, kind: TailwindSpacingKind) -> Self {
        Self {
            kind,
            size: TailwindSpacingSize::Px(n),
        }
    }
    pub fn unit(n: usize, kind: TailwindSpacingKind) -> Self {
        Self {
            kind,
            size: TailwindSpacingSize::Number(n),
        }
    }
}

impl TailwindSpacing {
    pub fn parser<'a>() -> impl FnMut(&'a str) -> ParsedList<'a> {
        // move |input| {
        //     as_list(alt((
        //         // https://tailwindcss.com/docs/aspect-ratio
        //         Self::parser_kind("auto", "auto"),
        //         Self::parser_kind("square", "1 / 1"),
        //         Self::parser_kind("video", "16 / 9"),
        //     ))(input))
        // }
        todo!()
    }
    /// (p|m)(t|r|b|l|x|y)-(px|n|auto)

    fn parse_pm<'a>(input: &str) -> ParsedItem<'a> {
        match tuple((//
                     alt((tag("p"), tag("m"))),
                     opt(one_of("trblxy")),
                     tag("-"),
                     Self::parser_size()
        ))(input) {
            Ok((rest, all)) => Ok((rest, all)),
            Err(e) => Err(e),
        }
    }

    fn parser_pm<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, TailwindSpacingKind> {
        move |input| match alt((//
                                tuple((
                                    one_of("pm"),
                                    opt(one_of("trblxy")),
                                )),
                                tuple((
                                    tag("space-"),
                                    one_of("xy"),
                                ))
        ))(input) {
            Ok((rest, all)) => Ok((rest, all)),
            Err(e) => Err(e),
        }
    }

    fn parser_size<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, TailwindSpacingSize> {
        let reverse = tag("reverse");
        let auto = tag("auto");
        let px = tag("px");
        let n = tag("n");
        move |input| match alt((reverse, auto, px, n))(input) {
            Ok((rest, all)) => Ok((rest, all)),
            Err(e) => Err(e),
        }
    }

    /// (space)(x|y)-(px|n|reverse)
    fn parse_space() {
        let space = tag("space");
        let xy = one_of("xy");
        todo!()
    }
}