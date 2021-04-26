use crate::{systems::typography::FontSmoothing, TailwindBuilder, TailwindInstance};
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1},
    sequence::tuple,
    IResult,
};
use std::collections::HashSet;

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
