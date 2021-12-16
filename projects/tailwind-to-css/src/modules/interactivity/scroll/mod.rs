
use self::scroll_behavior::TailwindScrollBehavior;
use super::*;
pub(crate) mod scroll_behavior;

#[doc=include_str!("readme.md")]
#[derive(Debug, Copy, Clone)]
pub struct TailwindScroll {}

impl TailwindScroll {
    /// <https://tailwindcss.com/docs/scroll-behavior>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary, negative: Negative) -> Result<Box<dyn TailwindInstance>> {
        let kind = match pattern {
            ["p" | "pl" | "pr" | "pb" | "pt" | "px" | "py", ..] =>
                TailwindScrollPadding::parse(pattern, arbitrary, negative)?.boxed(),
            ["m" | "ml" | "mr" | "mb" | "mt" | "mx" | "my", ..] =>
                TailwindScrollMargin::parse(pattern, arbitrary, negative)?.boxed(),
            _ => TailwindScrollBehavior::parse(pattern, arbitrary)?.boxed(),
        };
        Ok(kind)
    }
}
