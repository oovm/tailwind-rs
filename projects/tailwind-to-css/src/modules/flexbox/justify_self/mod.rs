use super::*;
#[doc=include_str!("readme.md")]
#[derive(Debug, Copy, Clone)]
pub enum TailwindJustifySelf {
    Auto,
    Start,
    End,
    Center,
    Stretch,
}
impl Display for TailwindJustifySelf {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindJustifySelf {}

impl TailwindJustifySelf {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        todo!()
    }
}
