use crate::{AxisXY, StandardValue};

use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindOverflow {
    kind: StandardValue,
    axis: AxisXY,
}

impl Display for TailwindOverflow {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.axis.write_xyn(f, "overflow", &self.kind)
    }
}

impl TailwindInstance for TailwindOverflow {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        let class = self.axis.format_xyn("overflow");
        css_attributes! {
            class => self.kind.get_properties()
        }
    }
}

impl TailwindOverflow {
    /// <https://tailwindcss.com/docs/overflow>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let (axis, rest) = AxisXY::split_xyn(pattern);
        let kind = StandardValue::parser("overflow", &Self::check_valid)(rest, arbitrary)?;
        Ok(Self { kind, axis })
    }
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/overflow#syntax>
    pub fn check_valid(mode: &str) -> bool {
        ["auto", "clip", "hidden", "inherit", "initial", "revert", "scroll", "unset", "visible"].contains(&mode)
    }
}
