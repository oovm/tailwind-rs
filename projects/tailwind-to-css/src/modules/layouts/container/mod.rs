use super::*;

#[doc = include_str!("readme.md")]
#[derive(Copy, Clone, Debug)]
pub struct TailwindContainer {}

impl TailwindContainer {
    pub fn new() -> Self {
        Self {}
    }
}
impl Display for TailwindContainer {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindContainer {}
