use super::*;
use tailwind_error::nom::{branch::alt, sequence::tuple};

impl TailwindBorderCollapse {
    pub fn parser<'a>() -> impl FnMut(&'a str) -> ParsedList<'a> {
        move |input| {
            as_list(alt((Self::parser_kind("collapse", Self::Collapse), Self::parser_kind("separate", Self::Collapse)))(input))
        }
    }
    pub fn parser_kind<'a>(kind: &'static str, item: Self) -> impl Fn(&'a str) -> ParsedItem {
        move |input| match tuple((
            //
            tag("border"),
            tag("-"),
            tag(kind),
        ))(input)
        {
            Ok((s, _)) => Ok((s, Box::new(item))),
            Err(e) => Err(e),
        }
    }
}

impl TailwindTableLayout {
    pub fn parse(input: &[&str]) -> Box<dyn TailwindInstance> {
        let out = match input {
            ["auto"] => Self::Auto,
            ["fixed"] => Self::Fixed,
            _ => {
                panic!("TODO")
            }
        };
        Box::new(out)
    }
}
