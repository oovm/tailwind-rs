use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindLeft {
    kind: UnitValue,
}

impl Display for TailwindLeft {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.kind.write_negative(f)?;
        self.kind.write_class(f, "left-")
    }
}

impl TailwindInstance for TailwindLeft {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        css_attributes! {
            "left" => self.kind.get_properties_rem()
        }
    }
}

impl TailwindLeft {
    /// <https://tailwindcss.com/docs/top-right-bottom-left>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary, negative: Negative) -> Result<Self> {
        let kind = UnitValue::negative_parser("left", Self::check_valid, true, false, true)(pattern, arbitrary, negative)?;
        Ok(Self { kind })
    }
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/left#syntax>
    pub fn check_valid(mode: &str) -> bool {
        ["auto", "inherit", "initial", "revert", "unset"].contains(&mode)
    }
}
