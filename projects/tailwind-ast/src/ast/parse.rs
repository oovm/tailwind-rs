use super::*;

impl<'a> AstGroup<'a> {
    /// `v:a?(a(a b))`
    #[inline]
    pub fn parse(input: &'a str) -> IResult<&'a str, Self> {
        let (rest, (head, children)) = tuple((AstStyle::parse, Self::parse_many))(input)?;
        Ok((rest, Self { head, children }))
    }
    #[inline]
    fn parse_many(input: &'a str) -> IResult<&'a str, Vec<AstGroupItem>> {
        let head = AstGroupItem::parse;
        let rest = many0(tuple((multispace1, AstGroupItem::parse)));
        let many = tuple((head, rest));
        let (rest, (first, other)) = delimited(char('('), many, char(')'))(input.trim())?;
        let mut out = vec![first];
        out.extend(other.into_iter().map(|s| s.1));
        Ok((rest, out))
    }
}

impl<'a> AstGroupItem<'a> {
    /// [`AstGroup`] or [`AstStyle`]
    #[inline]
    pub fn parse(input: &'a str) -> IResult<&'a str, Self> {
        let (rest, (head, group)) = tuple((AstStyle::parse, opt(AstGroup::parse_many)))(input)?;
        let out = match group {
            None => Self::Styled(head),
            Some(children) => Self::Grouped(AstGroup { head, children }),
        };
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
        let (rest, (first, other)) = tuple((Self::parse_head, many0(Self::parse_rest)))(input)?;
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
