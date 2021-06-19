use super::*;
use tailwind_error::nom::{bytes::complete::tag, combinator::opt, sequence::tuple};

impl TailwindBorderStyle {
    pub fn into_instance(self) -> Box<dyn TailwindInstance> {
        Box::new(self)
    }
}

impl TailwindRingOffsetWidth {
    pub fn parse(input: &[&str], arbitrary: &str) -> Result<Self> {
        match input {
            [] => Self::parse_arbitrary(arbitrary),
            [n] => Self::parse_arbitrary(n),
            _ => syntax_error!("unknown aspect-ratio elements"),
        }
    }
    pub fn parse_arbitrary(arbitrary: &str) -> Result<Self> {
        let (width, _) = tuple((parse_integer, opt(tag("px"))))(arbitrary)?.1;
        Ok(Self { width })
    }
}
