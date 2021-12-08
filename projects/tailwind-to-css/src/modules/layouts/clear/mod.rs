use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindClear {
    kind: String,
}

impl Display for TailwindClear {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "float-{}", self.kind)
    }
}

impl TailwindInstance for TailwindClear {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        css_attributes! {
            "float" => self.kind
        }
    }
}

impl TailwindClear {
    /// https://tailwindcss.com/docs/clear
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        debug_assert!(arbitrary.is_none(), "forbidden arbitrary after clear");
        let kind = pattern.join("-");
        debug_assert!(Self::check_valid(&kind));
        Ok(Self { kind })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/clear#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
            // Keyword values
            "left",
            "right",
            "none",
            "both",
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
