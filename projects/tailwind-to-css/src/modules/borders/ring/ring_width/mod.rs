use crate::NumericValue;

use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindRingWidth {
    kind: NumericValue,
}

impl Display for TailwindRingWidth {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ring-offset-{}", self.kind)
    }
}

impl TailwindInstance for TailwindRingWidth {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        css_attributes! {
            "--tw-ring-offset-width" => self.kind,
            "box-shadow" => "0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow)"
        }
    }
}
impl TailwindRingWidth {
    /// <https://tailwindcss.com/docs/ring-width>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let kind = match pattern {
            [] => NumericValue::from(3u32),
            _ => NumericValue::positive_parser("blur", Self::check_valid)(pattern, arbitrary)?,
        };
        Ok(Self { kind })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/object-fit#syntax
    pub fn check_valid(mode: &str) -> bool {
        ["inherit", "initial", "revert", "unset"].contains(&mode)
    }
}
