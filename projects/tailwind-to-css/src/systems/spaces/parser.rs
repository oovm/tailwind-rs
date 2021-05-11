use std::fs::read_to_string;
use super::*;
use nom::character::complete::one_of;

impl TailwindSpacing {
    pub fn auto(kind: TailwindSpacingKind) -> Self {
        Self { kind, size: TailwindSpacingSize::Auto }
    }
    pub fn reversed(kind: TailwindSpacingKind) -> Self {
        Self { kind, size: TailwindSpacingSize::Reversed }
    }
    pub fn px(n: usize, kind: TailwindSpacingKind) -> Self {
        Self { kind, size: TailwindSpacingSize::Px(n) }
    }
    pub fn unit(n: usize, kind: TailwindSpacingKind) -> Self {
        Self { kind, size: TailwindSpacingSize::Number(n) }
    }
}

impl TailwindSpacing {
    /// `(p|m)(t|r|b|l|x|y)?-(px|n|auto)`
    ///
    /// `(space)(x|y)-(px|n|reverse)`
    pub fn parser<'a>(input: &str) -> impl FnMut(&'a str) -> ParsedList<'a> {
        move |input| as_list(match tuple((Self::parser_kind(), tag("-"), Self::parser_size()))(input) {
            Ok((rest, (kind, _, size))) => Ok((rest, Box::new(Self { kind, size }))),
            Err(e) => Err(e),
        })
    }

    fn parser_kind<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, TailwindSpacingKind> {
        let pm = move |input: &'a str| match tuple((one_of("pm"), opt(one_of("trblxy"))))(input) {
            Ok((rest, (a, b))) => {
                Ok((rest, TailwindSpacingKind::parse_pm(a, b).unwrap()))
            }
            Err(e) => Err(e),
        };
        let space = move |input: &'a str| match tuple((tag("space-"), one_of("xy")))(input) {
            Ok((rest, (a, b))) => Ok((rest, TailwindSpacingKind::parse_space(a, b).unwrap())),
            Err(e) => Err(e),
        };
        alt((pm, space))
    }

    fn parser_size<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, TailwindSpacingSize> {
        move |input| match alt((tag("reverse"), tag("auto"), tag("px"), tag("n")))(input) {
            Ok((rest, all)) => {
                let size = match all {
                    "auto" => { TailwindSpacingSize::Auto }
                    _ => todo!()
                };
                Ok((rest, size))
            }
            Err(e) => Err(e),
        }
    }
}

impl TailwindSpacingKind {
    /// `(p|m)(t|r|b|l|x|y)?`
    fn parse_pm(a: char, b: Option<char>) -> Option<TailwindSpacingKind> {
        let kind = match (a, b) {
            ('p', None) => Self::Padding,
            ('p', Some('t')) => Self::PaddingT,
            ('p', Some('r')) => Self::PaddingR,
            ('p', Some('b')) => Self::PaddingB,
            ('p', Some('l')) => Self::PaddingL,
            ('p', Some('x')) => Self::PaddingX,
            ('p', Some('y')) => Self::PaddingY,
            ('m', None) => Self::Margin,
            ('m', Some('t')) => Self::MarginT,
            ('m', Some('r')) => Self::MarginR,
            ('m', Some('b')) => Self::MarginB,
            ('m', Some('l')) => Self::MarginL,
            ('m', Some('x')) => Self::MarginX,
            ('m', Some('y')) => Self::MarginY,
            _ => return None
        };
        return Some(kind);
    }
    /// `(space)(x|y)`
    fn parse_space(_: &str, b: char) -> Option<TailwindSpacingKind> {
        match b {
            'x' => Some(Self::SpaceBetweenX),
            'y' => Some(Self::SpaceBetweenY),
            _ => None
        }
    }
}