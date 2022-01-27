use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindTop {
    kind: UnitValue,
}

impl Display for TailwindTop {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.kind.write_negative(f)?;
        self.kind.write_class(f, "top-")
    }
}

impl TailwindInstance for TailwindTop {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        css_attributes! {
            "top" => self.kind.get_properties_rem()
        }
    }
}

impl TailwindTop {
    /// <https://tailwindcss.com/docs/top-right-bottom-left>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary, negative: Negative) -> Result<Self> {
        let kind = get_kind_px_full_auto_fact("top", pattern, arbitrary, negative)?;
        Ok(Self { kind })
    }
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/top#syntax>
    pub fn check_valid(mode: &str) -> bool {
        check_valid_auto(mode)
    }
}
