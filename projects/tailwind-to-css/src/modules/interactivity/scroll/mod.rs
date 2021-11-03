use crate::TailwindSizing;

use super::*;

pub(crate) mod scroll_padding;

#[doc=include_str!("readme.md")]
#[derive(Debug, Copy, Clone)]
pub struct TailwindScroll {}

impl Display for TailwindScroll {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindScroll {}

impl TailwindScroll {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
        todo!()
    }
    pub fn check_valid(mode: &str) -> bool {
        todo!()
    }
}
