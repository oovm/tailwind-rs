pub mod breakpoints;
pub mod colors;
pub mod flexbox;
pub mod fonts;
pub mod layouts;
pub mod preflight;
pub mod sizes;
pub mod spaces;
pub mod typography;

use crate::TailwindInstance;
use css_style::unit::{percent, px, rem, Length};
use nom::{bytes::complete::tag, IResult};
use std::{
    collections::HashMap,
    fmt::{Debug, Display, Formatter, Write},
};

/// Tailwind Parsed Result
pub type TailwindParsed<'a> = IResult<&'a str, Box<dyn TailwindInstance>>;

/// Remove instance from builder
pub(crate) struct SealedRemover(String);

/// Uncategorized tailwind property
#[derive(Debug)]
pub struct TailwindObject {
    id: &'static str,
    attributes: &'static str,
}

impl TailwindObject {
    pub fn new(id: &'static str, css: &'static str) -> Box<dyn TailwindInstance> {
        Box::new(Self { id, attributes: css })
    }
}

impl TailwindInstance for TailwindObject {
    fn id(&self) -> String {
        self.id.to_owned()
    }
    fn attributes(&self) -> Vec<String> {
        self.attributes.trim().lines().map(|s| s.trim().to_owned()).collect()
    }
}

impl TailwindObject {
    pub fn parser<'a>(id: &'static str, css: &'static str) -> impl Fn(&'a str) -> TailwindParsed<'a> {
        move |input| match tag(id)(input) {
            Ok(o) => Ok((o.0, Self::new(id, css))),
            Err(e) => Err(e),
        }
    }
}
