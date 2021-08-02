use super::*;
use nom::{
    branch::alt,
    bytes::complete::take_till1,
    character::complete::{alphanumeric1, multispace0, multispace1},
    multi::{many0, separated_list0},
    sequence::delimited,
};

impl<'a> AstGroup<'a> {
    /// `v:a?(a(a b))`
    pub fn parse(input: &'a str) -> IResult<&'a str, Self> {
        let (rest, (head, children)) = tuple((AstStyle::parse, Self::parse_many))(input)?;
        Ok((rest, Self { head, children }))
    }
    #[inline]
    fn parse_many(input: &'a str) -> IResult<&'a str, Vec<AstGroupItem>> {
        let head = AstGroupItem::parse;
        let rest = many0(tuple((multispace1, AstGroupItem::parse)));
        let many = tuple((multispace0, head, rest, multispace0));
        let (rest, (_, first, other, _)) = delimited(char('('), many, char(')'))(input)?;
        let mut out = vec![first];
        out.extend(other.into_iter().map(|s| s.1));
        Ok((rest, out))
    }
}

impl<'a> AstGroupItem<'a> {
    /// [`AstGroup`] or [`AstStyle`]
    pub fn parse(input: &'a str) -> IResult<&'a str, Self> {
        alt((Self::parse_nested, Self::parse_style))(input)
    }
    #[inline]
    fn parse_style(input: &'a str) -> IResult<&'a str, Self> {
        AstStyle::parse(input).map(|(rest, ok)| (rest, Self::Styled(ok)))
    }
    // #[inline]
    // fn parse_self(input: &'a str) -> IResult<&'a str, Self> {
    //     AstReference::parse(input).map(|(rest, ok)| (rest, Self::SelfReference(ok)))
    // }
    #[inline]
    fn parse_nested(input: &'a str) -> IResult<&'a str, Self> {
        AstGroup::parse(input).map(|(rest, ok)| (rest, Self::Grouped(ok)))
    }
}

impl<'a> AstStyle<'a> {
    /// `v:v::-?a-a-a-[A]`
    pub fn parse(input: &'a str) -> IResult<&'a str, Self> {
        let (rest, (variants, negative, elements, arbitrary)) = tuple((
            many0(ASTVariant::parse),
            opt(char('-')),
            opt(AstElements::parse),
            opt(AstArbitrary::parse),
        ))(input)?;

        Ok((
            rest,
            Self {
                negative: negative.is_some(),
                variants,
                elements: elements.unwrap_or_default().elements,
                arbitrary: arbitrary.map(|s| s.arbitrary),
            },
        ))
    }
}

impl<'a> AstElements<'a> {
    /// a(-a)*
    pub fn parse(input: &'a str) -> IResult<&'a str, Self> {
        let (rest, (first, other)) = tuple((Self::parse_head, many0(Self::parse_rest)))(input)?;
        let mut out = vec![first];
        out.extend(other.into_iter());
        Ok((rest, Self { elements: out }))
    }
    #[inline]
    fn parse_head(input: &'a str) -> IResult<&'a str, &'a str> {
        let stop = |c: char| -> bool { matches!(c, ' ' | '-' | '[' | ']' | '(' | ')') };
        take_till1(stop)(input)
    }
    #[inline]
    fn parse_rest(input: &'a str) -> IResult<&'a str, &'a str> {
        let (rest, (_, out)) = tuple((char('-'), Self::parse_head))(input)?;
        Ok((rest, out))
    }
}

impl<'a> ASTVariant<'a> {
    /// `(not-)?variant:pseudo::`
    ///
    /// ## Reference
    /// -
    pub fn parse(input: &'a str) -> IResult<&'a str, Self> {
        let (rest, (mut v, s)) = tuple((Self::parse_one, alt((tag("::"), tag(":")))))(input)?;
        if s == "::" {
            v.pseudo = true
        }
        else {
            v.pseudo = Self::check_pseudo(&v.names.iter().map(<_>::as_ref).collect::<Vec<_>>());
        }
        Ok((rest, v))
    }
    /// `(not-)?(ALPHA)(-ALPHA)*`
    ///
    /// eg:
    /// - `not-focus`
    /// - `not-last-child`
    fn parse_one(input: &'a str) -> IResult<&'a str, Self> {
        let not = opt(tuple((tag("not"), tag("-"))));
        let vs = separated_list0(tag("-"), alphanumeric1);
        let (rest, (not, names)) = tuple((not, vs))(input)?;
        Ok((rest, Self { not: not.is_some(), pseudo: false, names }))
    }
    #[rustfmt::skip]
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/Pseudo-elements#index
    fn check_pseudo(names: &[&str]) -> bool {
        matches!(names
            , ["after"]
            | ["before"]
            | ["backdrop"]
            | ["marker"]
            | ["placeholder"]
            | ["selection"]
            | ["first", "line"]
            | ["first", "litter"]
            | ["first", "selector", "button"]
            | ["target", "text"]
        )
    }
}

impl<'a> AstArbitrary<'a> {
    /// `-[ANY+]`
    pub fn parse(input: &'a str) -> IResult<&'a str, Self> {
        let pair = delimited(char('['), take_till1(|c| c == ']'), char(']'));
        let (rest, (_, arbitrary)) = tuple((char('-'), pair))(input)?;
        Ok((rest, Self { arbitrary }))
    }
}

impl AstReference {
    /// `&`
    pub fn parse(input: &str) -> IResult<&str, Self> {
        let (rest, _) = char('&')(input)?;
        Ok((rest, Self {}))
    }
}
