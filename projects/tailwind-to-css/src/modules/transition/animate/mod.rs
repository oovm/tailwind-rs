use super::*;
use std::fmt::Write;

#[doc = include_str!("readme.md")]
#[derive(Copy, Clone, Debug)]
pub struct TailwindAnimate {}

impl Display for TailwindAnimate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindAnimate {
    fn write_css(&self, f: &mut (dyn Write), ctx: &TailwindBuilder) -> Result<()> {
        todo!()
    }
}

impl TailwindAnimate {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        todo!()
    }
}
