use self::scroll_behavior::TailwindScrollBehavior;
use super::*;
pub(crate) mod scroll_behavior;

#[doc=include_str!("readme.md")]
#[derive(Debug, Copy, Clone)]
pub struct TailwindScroll {}

impl TailwindScroll {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary, negative: bool) -> Result<Box<dyn TailwindInstance>> {
        let kind = match pattern {
            ["p" | "pl" | "pr" | "pm" | "pt" | "px" | "py", ..] =>
                TailwindScrollPadding::parse(pattern, arbitrary, negative)?.boxed(),
            ["m" | "ml" | "mr" | "mm" | "mt" | "mx" | "my", ..] =>
                TailwindScrollMargin::parse(pattern, arbitrary, negative)?.boxed(),
            _ => TailwindScrollBehavior::parse(pattern, arbitrary)?.boxed(),
        };
        Ok(kind)
    }
}
