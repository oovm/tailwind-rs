use super::*;
use nom::{character::complete::multispace1, multi::separated_list1};
use std::mem::swap;

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
        let stop = |c: char| -> bool { matches!(c, ':' | '|' | '[' | ']' | '(' | ')' | ' ') };
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
    pub fn parse_list(input: &str) -> IResult<&str, Vec<Self>> {
        separated_list0(multispace1, Self::parse)(input)
    }
}

impl AstStyle {
    pub fn view_elements(&self) -> Vec<&str> {
        self.elements.iter().map(|s| s.0.as_str()).collect()
    }
}

impl AstGroup {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        alt((Self::parse_grouped, Self::parse_standalone))(input)
    }
    pub fn parse_list(input: &str) -> IResult<&str, Vec<Self>> {
        separated_list0(multispace1, Self::parse)(input)
    }
    #[inline]
    fn parse_standalone(input: &str) -> IResult<&str, Self> {
        let (rest, inner) = AstStyle::parse(input)?;
        Ok((rest, Self::Standalone { inner }))
    }
    #[inline]
    fn parse_grouped(input: &str) -> IResult<&str, Self> {
        let lhs = tuple((AstVariant::parse_many, opt(AstElement::parse)));
        let rhs = delimited(char('('), AstStyle::parse_list, char(')'));
        let (rest, ((variants, elements), inner)) = tuple((lhs, rhs))(input)?;
        Ok((rest, Self::Grouped { variants, elements, inner }))
    }
    pub fn expand(s: Self, buffer: &mut Vec<AstStyle>) {
        match s {
            Self::Standalone { inner } => buffer.push(inner),
            Self::Grouped { variants, elements, inner } => {
                for mut i in inner {
                    i.variants.extend(variants.iter().cloned());
                    if let Some(e) = elements.clone() {
                        let mut elements = vec![e];
                        swap(&mut elements, &mut i.elements);
                        i.elements.extend(elements.into_iter());
                    }
                    buffer.push(i)
                }
            }
        }
    }
    pub fn expand_list(v: Vec<Self>) -> Vec<AstStyle> {
        let mut out = vec![];
        for i in v {
            Self::expand(i, &mut out)
        }
        out
    }
}

#[test]
fn test_style() {
    // w-full sm:w-auto text-lg uppercase text-gray-100 bg-purple-800 hover:bg-purple-700 focus:bg-purple-700 focus-visible:ring-4 ring-purple-400 px-6
    println!("{:#?}", AstStyle::parse("not-hover:sm:text-red-200").unwrap().1);
    println!("{:#?}", AstStyle::parse_list("w-full sm:w-auto").unwrap().1);
}

#[test]
fn test_group() {
    println!("{:#?}", AstGroup::parse_list("w(full sm:auto)").unwrap().1);
    println!("{:#?}", AstGroup::parse_list("not-hover:sm:text-red-200").unwrap().1);
}

#[test]
fn test_group_expand() {
    let g = AstGroup::parse_list("w(full sm:auto)").unwrap().1;
    println!("{:#?}", AstGroup::expand_list(g));
}
