use crate::StandardValue;

use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindOverscroll {
    kind: StandardValue,
    axis: AxisXY,
}

impl Display for TailwindOverscroll {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.axis.write_xyn(f, "overscroll", &self.kind)
    }
}

impl TailwindInstance for TailwindOverscroll {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        let class = self.axis.format_xyn("overscroll-behavior");
        css_attributes! {
            class => self.kind.get_properties()
        }
    }
}

impl TailwindOverscroll {
    /// <https://tailwindcss.com/docs/overscroll-behavior>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let (axis, rest) = AxisXY::split_xyn(pattern);
        let kind = StandardValue::parser("overscroll", &Self::check_valid)(rest, arbitrary)?;
        Ok(Self { kind, axis })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/overscroll-behavior#syntax
    pub fn check_valid(mode: &str) -> bool {
        ["auto", "contain", "inherit", "initial", "none", "revert", "unset"].contains(&mode)
    }
}
