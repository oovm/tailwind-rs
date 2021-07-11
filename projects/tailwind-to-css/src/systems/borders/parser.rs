use super::*;
use crate::parse_i_px_maybe;
use tailwind_error::nom::{combinator::opt, sequence::tuple, IResult};

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
        Ok(Self { width: parse_i_px_maybe(arbitrary)?.1 })
    }
}
