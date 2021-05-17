use super::*;

impl AstVariant {
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

impl AstArbitrary {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        let (rest, r) = delimited(char('['), take_till1(|c| c != ']'), char(']'))(input)?;
        Ok((rest, Self(r.to_string())))
    }
    pub fn parse_maybe(input: &str) -> IResult<&str, Option<Self>> {
        let (rest, rhs) = opt(tuple((tag("-"), Self::parse)))(input)?;
        Ok((rest, rhs.map(|(_, e)| e)))
    }
}

impl AstElement {
    fn parse(input: &str) -> IResult<&str, Self> {
        let stop = |c: char| -> bool { c == ':' || c == '|' || c == '[' };
        let (rest, s) = take_till(stop)(input)?;
        Ok((rest, Self(s.to_string())))
    }
}

impl AstElement {
    #[inline]
    pub fn is_self_reference(&self) -> bool {
        self.0 == "&"
    }

    pub fn as_usize(&self) -> Option<usize> {
        parse_integer(&self.0).ok().map(|(_, o)| o)
    }
    pub fn as_fraction(&self) -> Option<(usize, usize)> {
        parse_fraction(&self.0).ok().map(|(_, o)| o)
    }
}

impl AstStyle {
    /// `v:v::-a-a-a-[A]`
    pub fn parse(input: &str) -> IResult<&str, Self> {
        let es = separated_list0(tag("-"), AstElement::parse);
        let (rest, rhs) = tuple((AstVariant::parse_many, opt(tag("-")), es, AstArbitrary::parse_maybe))(input)?;
        let (variants, neg, atoms, arbitrary) = rhs;
        Ok((rest, Self { negative: neg.is_some(), variants, elements: atoms, arbitrary }))
    }
}

impl AstStyle {
    pub fn view_elements(&self) -> Vec<&str> {
        self.elements.iter().map(|s| s.0.as_str()).collect()
    }
}

#[test]
fn test() {
    println!("{:#?}", AstStyle::parse("not-hover:sm:text-200").unwrap().1);
}
