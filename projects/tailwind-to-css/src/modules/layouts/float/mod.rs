use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindFloat {
    kind: String,
}

impl Display for TailwindFloat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "float-{}", self.kind)
    }
}

impl TailwindInstance for TailwindFloat {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        css_attributes! {
            "float" => self.kind
        }
    }
}

impl TailwindFloat {
    /// https://tailwindcss.com/docs/float
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        debug_assert!(arbitrary.is_none(), "forbidden arbitrary after float");
        let kind = pattern.join("-");
        debug_assert!(Self::check_valid(&kind));
        Ok(Self { kind })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/float#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
            // Keyword values
            "left",
            "right",
            "none",
            "inline-start",
            "inline-end",
            // Global values
            "inherit",
            "initial",
            "revert",
            "unset",
        ]);
        set.contains(mode)
    }
}
