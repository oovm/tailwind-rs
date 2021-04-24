use crate::{
    systems::{layouts::TailwindBreak, typography::FontSmoothing},
    TailwindBuilder, TailwindInstance, TailwindObject, TailwindParser,
};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::{alpha0, alpha1, alphanumeric1, digit1},
    sequence::tuple,
    Compare, IResult, InputLength, InputTake,
};
use std::{collections::HashSet, fmt::Write};

pub mod typography;

impl TailwindBuilder {
    pub(crate) fn parse(&self, input: &str) -> HashSet<Box<dyn TailwindInstance>> {
        todo!()
    }
    pub(crate) fn parse_text(&self, input: &str) -> HashSet<Box<dyn TailwindInstance>> {
        todo!()
    }
}

fn parse_text(input: &str) -> IResult<&str, (&str, &str)> {
    match tuple((tag("text"), tag("-"), alpha1, tag("-"), digit1))(input) {
        Ok(o) => Ok((o.0, (&o.1.2, &o.1.4))),
        Err(e) => Err(e),
    }
}

fn layout_parser<'a>(input: &str) -> TailwindParser<'a> {
    alt((
        // aspect:
        TailwindObject::parser("aspect-auto", "aspect-ratio: auto;"),
        TailwindObject::parser("aspect-square", "aspect-ratio: 1 / 1;"),
        TailwindObject::parser("aspect-video", "aspect-ratio: 16 / 9;"),
        // TODO: container https://tailwindcss.com/docs/container
        // TODO: Columns https://tailwindcss.com/docs/container
        TailwindBreak::parser_after("auto"),
        TailwindObject::parser("break-after-avoid", "break-after: avoid;"),
        TailwindObject::parser("break-after-all", "break-after: all;"),
        TailwindObject::parser("break-after-avoid-page", "break-after: avoid-page;"),
        TailwindObject::parser("break-after-page", "break-after: page;"),
        TailwindObject::parser("break-after-left", "break-after: left;"),
        TailwindObject::parser("break-after-right", "break-after: right;"),
        TailwindObject::parser("break-after-column", "break-after: column;"),
    ))(input)
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

#[test]
fn test() {
    println!("{:?}", parse_text("text-blue-100").unwrap())
}
