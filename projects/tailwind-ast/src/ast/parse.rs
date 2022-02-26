use super::*;
use nom::error::{ErrorKind, ParseError};

impl<'a> AstGroup<'a> {
    /// `v:a?(a(a b))`
    #[inline]
    pub fn parse(input: &'a str) -> IResult<&'a str, Self> {
        let (rest, (head, children)) = tuple((AstStyle::parse, Self::parse_pair))(input)?;
        Ok((rest, Self { head, children }))
    }
    #[inline]
    fn parse_pair(input: &'a str) -> IResult<&'a str, Vec<AstGroupItem>> {
        let (rest, paired) = delimited_paired('(', ')')(input)?;
        Ok((rest, AstGroupItem::parse_many(paired.trim())?.1))
    }
}

impl<'a> AstGroupItem<'a> {
    /// [`AstGroup`] or [`AstStyle`]
    #[inline]
    pub fn parse(input: &'a str) -> IResult<&'a str, Self> {
        let (rest, (head, group)) = tuple((AstStyle::parse, opt(AstGroup::parse_pair)))(input)?;
        let out = match group {
            None => Self::Styled(head),
            Some(children) => Self::Grouped(AstGroup { head, children }),
        };
        Ok((rest, out))
    }
    #[inline]
    fn parse_many(input: &'a str) -> IResult<&'a str, Vec<Self>> {
        let head = AstGroupItem::parse;
        let rest = many0(tuple((multispace1, AstGroupItem::parse)));
        let (rest, (first, other)) = tuple((head, rest))(input)?;
        let mut out = vec![first];
        out.extend(other.into_iter().map(|s| s.1));
        Ok((rest, out))
    }
    // #[inline]
    // fn parse_style(input: &'a str) -> IResult<&'a str, Self> {
    //     AstStyle::parse(input).map(|(rest, ok)| (rest, Self::Styled(ok)))
    // }
    // #[inline]
    // fn parse_nested(input: &'a str) -> IResult<&'a str, Self> {
    //     AstGroup::parse(input).map(|(rest, ok)| (rest, Self::Grouped(ok)))
    // }
    // #[inline]
    // fn parse_self(input: &'a str) -> IResult<&'a str, Self> {
    //     AstReference::parse(input).map(|(rest, ok)| (rest, Self::SelfReference(ok)))
    // }
}

impl<'a> AstStyle<'a> {
    /// `v:v::-?a-a-a-[A]`
    #[inline]
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
    #[inline]
    pub fn parse(input: &'a str) -> IResult<&'a str, Self> {
        let (rest, (first, other)) = tuple((Self::parse_head, many0(Self::parserest)))(input)?;
        let mut out = vec![first];
        out.extend(other.into_iter());
        Ok((rest, Self { elements: out }))
    }
    #[inline]
    fn parse_head(input: &'a str) -> IResult<&'a str, &'a str> {
        let stop = |c: char| -> bool {
            // space
            matches!(c, ' ' | '\n' | '\r' | '-' | '[' | ']' | '(' | ')')
        };
        take_till1(stop)(input)
    }
    #[inline]
    fn parserest(input: &'a str) -> IResult<&'a str, &'a str> {
        let (rest, (_, out)) = tuple((char('-'), Self::parse_head))(input)?;
        Ok((rest, out))
    }
}

impl<'a> ASTVariant<'a> {
    /// `(not-)?variant:pseudo::`
    ///
    /// ## Reference
    /// -
    #[inline]
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
    #[inline]
    fn parse_one(input: &'a str) -> IResult<&'a str, Self> {
        let not = opt(tuple((tag("not"), tag("-"))));
        let vs = separated_list0(tag("-"), alphanumeric1);
        let (rest, (not, names)) = tuple((not, vs))(input)?;
        Ok((rest, Self { not: not.is_some(), pseudo: false, names }))
    }
    #[rustfmt::skip]
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/Pseudo-elements#index
    #[inline]    fn check_pseudo(names: &[&str]) -> bool {
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
    #[inline]
    pub fn parse(input: &'a str) -> IResult<&'a str, Self> {
        let pair = delimited(char('['), take_till1(|c| c == ']'), char(']'));
        let (rest, (_, arbitrary)) = tuple((char('-'), pair))(input)?;
        Ok((rest, Self { arbitrary }))
    }
}

impl AstReference {
    /// `&`
    #[inline]
    pub fn parse(input: &str) -> IResult<&str, Self> {
        let (rest, _) = char('&')(input)?;
        Ok((rest, Self {}))
    }
}

fn delimited_paired(opening: char, closing: char) -> impl Fn(&str) -> IResult<&str, &str> {
    move |input: &str| {
        delimited(char(opening), take_until_unbalanced(opening, closing), char(closing))(input)
    }
}

/// https://stackoverflow.com/questions/70630556/parse-allowing-nested-parentheses-in-nom
fn take_until_unbalanced(
    opening_bracket: char,
    closing_bracket: char,
) -> impl Fn(&str) -> IResult<&str, &str> {
    move |i: &str| {
        let mut index = 0;
        let mut bracket_counter = 0;
        while let Some(n) = &i[index..].find(&[opening_bracket, closing_bracket, '\\'][..]) {
            index += n;
            let mut it = i[index..].chars();
            match it.next().unwrap_or_default() {
                c if c == '\\' => {
                    // Skip the escape char `\`.
                    index += '\\'.len_utf8();
                    // Skip also the following char.
                    if let Some(c) = it.next() {
                        index += c.len_utf8();
                    }
                }
                c if c == opening_bracket => {
                    bracket_counter += 1;
                    index += opening_bracket.len_utf8();
                }
                c if c == closing_bracket => {
                    // Closing bracket.
                    bracket_counter -= 1;
                    index += closing_bracket.len_utf8();
                }
                // Can not happen.
                _ => unreachable!(),
            };
            // We found the unmatched closing bracket.
            if bracket_counter == -1 {
                // We do not consume it.
                index -= closing_bracket.len_utf8();
                return Ok((&i[index..], &i[0..index]));
            };
        }

        if bracket_counter == 0 {
            Ok(("", i))
        }
        else {
            Err(Err::Error(Error::from_error_kind(i, ErrorKind::TakeUntil)))
        }
    }
}
