use super::*;
use nom::{
    branch::alt,
    bytes::complete::take_till1,
    character::complete::alphanumeric1,
    multi::{many0, separated_list0},
    sequence::delimited,
};

impl<'a> AstStyle<'a> {
    /// `v:v::-?a-a-a-[A]`
    pub fn parse(input: &'a str) -> IResult<&'a str, Self> {
        let (rest, (variants, negative, arbitrary)) =
            tuple((ASTVariant::parse, opt(char('-')), opt(AstArbitrary::parse)))(input)?;

        Ok((rest, Self { negative: negative.is_some(), variants, elements: vec![], arbitrary }))
    }

    #[inline]
    fn parse_elements() {}
}

impl<'a> ASTVariant<'a> {
    /// `(not-)?variant:pseudo::`
    ///
    /// ## Reference
    /// -
    pub fn parse(input: &'a str) -> IResult<&'a str, Vec<Self>> {
        let (rest, out) = many0(tuple((Self::parse_one, alt((tag("::"), tag(":"))))))(input)?;
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
    /// `(not-)?(ALPHA)(-ALPHA)*`
    ///
    /// eg:
    /// - `not-focus`
    /// - `not-last-child`
    fn parse_one(input: &'a str) -> IResult<&'a str, Self> {
        let not = opt(tuple((tag("not"), tag("-"))));
        let vs = separated_list0(tag("-"), alphanumeric1);
        let (rest, (not, names)) = tuple((not, vs))(input)?;
        let pseudo = Self::check_pseudo(&names.iter().map(<_>::as_ref).collect::<Vec<_>>());
        Ok((rest, Self { not: not.is_some(), pseudo, names }))
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

// impl AstElement {
//     fn parse(input: &str) -> IResult<&str, Self> {
//         let stop = |c: char| -> bool { matches!(c, ':' | '|' | '[' | ']' | '(' | ')' | ' ') };
//         let (rest, s) = take_till(stop)(input)?;
//         Ok((rest, Self(s.to_string())))
//     }
// }
//
// impl TailwindInstruction {

// }
//
// impl TailwindInstruction {
//     #[inline]
//     pub fn view_elements(&self) -> Vec<&str> {
//         self.elements.iter().map(|s| s.0.as_str()).collect()
//     }
//     #[inline]
//     pub fn view_arbitrary(&self) -> &str {
//         match &self.arbitrary {
//             None => "",
//             Some(setter) => setter.0.as_str(),
//         }
//     }
//     // TODO
//     pub fn normalization(self) -> Self {
//         self
//     }
// }
//
// impl AstGroup {
//     pub fn parse(input: &str) -> IResult<&str, Self> {
//         alt((Self::parse_grouped, Self::parse_standalone))(input)
//     }
//     pub fn parse_list(input: &str) -> IResult<&str, Vec<Self>> {
//         separated_list0(multispace1, Self::parse)(input)
//     }
//     #[inline]
//     fn parse_standalone(input: &str) -> IResult<&str, Self> {
//         let (rest, inner) = TailwindInstruction::parse(input)?;
//         Ok((rest, Self::Standalone { inner }))
//     }
//     #[inline]
//     fn parse_grouped(input: &str) -> IResult<&str, Self> {
//         let lhs = tuple((TailwindVariant::parse_many, opt(AstElement::parse)));
//         let rhs = delimited(char('('), TailwindInstruction::parse_list, char(')'));
//         let (rest, ((variants, elements), inner)) = tuple((lhs, rhs))(input)?;
//         Ok((rest, Self::Grouped { variants, elements, inner }))
//     }
//
//     pub fn expand(s: Self, buffer: &mut Vec<TailwindInstruction>) {
//         match s {
//             Self::Standalone { inner } => buffer.push(inner),
//             Self::Grouped { variants: vl, elements: el, inner } => {
//                 for TailwindInstruction { negative, variants: vr, elements: er, arbitrary } in inner {
//                     let mut variants = vl.clone();
//                     variants.extend(vr.into_iter());
//                     let mut elements = match el.clone() {
//                         None => vec![],
//                         Some(s) => vec![s],
//                     };
//                     elements.extend(er.into_iter());
//                     let new = TailwindInstruction { negative, variants, elements, arbitrary };
//                     buffer.push(new);
//                 }
//             }
//         }
//     }
//     pub fn expand_list(v: Vec<Self>) -> Vec<TailwindInstruction> {
//         let mut out = vec![];
//         for i in v {
//             Self::expand(i, &mut out)
//         }
//         out
//     }
// }
//
// impl TailwindBuilder {
//     pub fn parse_styles(input: &str) -> Result<Vec<TailwindInstruction>> {
//         let g = AstGroup::parse_list(input.trim())?.1;
//         Ok(AstGroup::expand_list(g))
//     }
// }
