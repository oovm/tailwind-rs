use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindRight {
    kind: UnitValue,
}

impl Display for TailwindRight {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.kind.write_negative(f)?;
        self.kind.write_class(f, "right-")
    }
}

impl TailwindInstance for TailwindRight {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        css_attributes! {
            "right" => self.kind.get_properties_rem()
        }
    }
}

impl TailwindRight {
    /// <https://tailwindcss.com/docs/top-right-bottom-left>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary, negative: Negative) -> Result<Self> {
        let kind = UnitValue::negative_parser("right", Self::check_valid, true, false, true)(pattern, arbitrary, negative)?;
        Ok(Self { kind })
    }
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/right#syntax>
    pub fn check_valid(mode: &str) -> bool {
        ["auto", "inherit", "initial", "revert", "unset"].contains(&mode)
    }
}
