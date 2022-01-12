use super::*;

#[doc=include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindTracking {
    kind: StandardValue,
}

impl Display for TailwindTracking {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "tracking-{}", self.kind)
    }
}

impl TailwindInstance for TailwindTracking {
    fn attributes(&self, ctx: &TailwindBuilder) -> CssAttributes {
        let tracking = match &self.kind {
            StandardValue::Keyword(s) => match s.as_str() {
                s if Self::check_valid(s) => s.to_string(),
                _ => format!("{}em", ctx.fonts.get_tracking(s)),
            },
            StandardValue::Arbitrary(s) => s.get_properties(),
        };
        css_attributes! {
            "letter-spacing" => tracking
        }
    }
}

impl TailwindTracking {
    /// <https://tailwindcss.com/docs/letter-spacing>
    pub fn parse(input: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let kind = StandardValue::parser("tracking", &|_| true)(input, arbitrary)?;
        Ok(Self { kind })
    }
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/letter-spacing#syntax>
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec!["inherit", "initial", "normal", "unset"]);
        set.contains(mode)
    }
}
