use super::*;

use self::animation::Animation;

mod animation;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindAnimate {
    kind: Animation,
}

impl Display for TailwindAnimate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "animate-{}", self.kind)
    }
}

impl TailwindInstance for TailwindAnimate {}

impl TailwindAnimate {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: Animation::parse(pattern, arbitrary)? })
    }
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: Animation::parse_arbitrary(arbitrary)? })
    }
}
