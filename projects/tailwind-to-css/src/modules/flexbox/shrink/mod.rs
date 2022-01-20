use super::*;

#[doc=include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailWindShrink {
    shrink: NumericValue,
}

impl Display for TailWindShrink {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "shrink-{}", self.shrink)
    }
}

impl TailwindInstance for TailWindShrink {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        let shrink = self.shrink.get_properties(|f| (f / 100.0).to_string());
        css_attributes! {
            "flex-shrink" => shrink
        }
    }
}

impl TailWindShrink {
    /// <https://tailwindcss.com/docs/flex-shrink>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let shrink = match pattern {
            [] if arbitrary.is_none() => 100u32.into(),
            _ => NumericValue::positive_parser("shrink", Self::check_valid)(pattern, arbitrary)?,
        };
        Ok(Self { shrink })
    }
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/flex-shrink#syntax>
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec!["inherit", "initial", "revert", "unset"]);
        set.contains(mode)
    }
}
