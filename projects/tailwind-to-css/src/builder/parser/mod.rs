use crate::{systems::typography::FontSmoothing, TailwindAspect, TailwindBorderCollapse, TailwindBreak, TailwindBuilder, TailwindInstance, ParsedItem, TailwindTableLayout};
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1},
    sequence::tuple,
    IResult,
};
use std::collections::{BTreeSet, HashSet};
use nom::branch::alt;
use nom::character::complete::space0;
use nom::combinator::opt;
use nom::multi::many0;
use crate::systems::ParsedList;

pub mod typography;

impl TailwindBuilder {
    /// `(item (WS item)*)?`
    pub(crate) fn parse(&self, input: &str) -> BTreeSet<Box<dyn TailwindInstance>> {
        let mut out = BTreeSet::new();
        // FIXME: stupid code !!!
        let item0 = alt((
            self.maybe_layout_system(),
            self.maybe_table_system(),
        ));
        let item = alt((
            self.maybe_layout_system(),
            self.maybe_table_system(),
        ));
        match opt(tuple((item0, many0(tuple((space0, item))))))(input) {
            Ok((_, Some((head, rest)))) => {
                out.extend(head.into_iter());
                for (_, items) in rest {
                    out.extend(items.into_iter());
                }
            }
            Ok((_, None)) => {}
            Err(e) => todo!()
        };
        return out;
    }
    fn parse_text(&self, _input: &str) -> HashSet<Box<dyn TailwindInstance>> {
        todo!()
    }

    fn maybe_layout_system<'a>(&self) -> impl FnMut(&'a str) -> ParsedList<'a> {
        alt((TailwindAspect::parser(), TailwindBreak::parser()))
    }


    fn maybe_table_system<'a>(&self) -> impl FnMut(&'a str) -> ParsedList<'a> {
        alt((
            TailwindBorderCollapse::parser(),
            TailwindTableLayout::parser(),
        ))
    }
}

fn parse_text(input: &str) -> IResult<&str, (&str, &str)> {
    match tuple((tag("text"), tag("-"), alpha1, tag("-"), digit1))(input) {
        Ok(o) => Ok((o.0, (&o.1.2, &o.1.4))),
        Err(e) => Err(e),
    }
}

fn parse_width(input: &str) -> IResult<&str, (&str, &str)> {
    match tuple((tag("w"), tag("-"), alpha1, tag("-"), digit1))(input) {
        Ok(o) => Ok((o.0, (&o.1.2, &o.1.4))),
        Err(e) => Err(e),
    }
}

fn parse_height(input: &str) -> IResult<&str, (&str, &str)> {
    match tuple((tag("h"), tag("-"), alpha1, tag("-"), digit1))(input) {
        Ok(o) => Ok((o.0, (&o.1.2, &o.1.4))),
        Err(e) => Err(e),
    }
}