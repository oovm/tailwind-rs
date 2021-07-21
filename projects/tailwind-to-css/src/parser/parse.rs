use super::*;


impl<'a> ASTVariant<'a> {
    
}

impl TailwindVariant {
    /// `(not-)?(ALPHA)(-ALPHA)*`
    ///
    /// eg:
    /// - `not-focus`
    /// - `not-last-child`
    pub fn parse(input: &str) -> IResult<&str, Self> {
        let not = opt(tuple((tag("not"), tag("-"))));
        let vs = separated_list0(tag("-"), alphanumeric1);
        let (rest, (not, r)) = tuple((not, vs))(input)?;
        let names: Vec<_> = r.into_iter().map(|f| f.to_string()).collect();
        let pseudo = Self::check_pseudo(&names.iter().map(<_>::as_ref).collect::<Vec<_>>());
        Ok((rest, Self { not: not.is_some(), pseudo, names }))
    }
    /// `var:var::`
    pub fn parse_many(input: &str) -> IResult<&str, Vec<Self>> {
        let (rest, out) = many0(tuple((Self::parse, alt((tag("::"), tag(":"))))))(input)?;
        let mut v = vec![];
        for (a, s) in out {
            let mut a = a;
            if s == "::" {
                a.pseudo = true
            }
            v.push(a)
        }
        Ok((rest, v))
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