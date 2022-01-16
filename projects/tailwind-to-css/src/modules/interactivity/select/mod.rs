use super::*;

#[doc=include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindSelect {
    kind: StandardValue,
}

crate::macros::sealed::keyword_instance!(TailwindSelect => "user-select");

impl Display for TailwindSelect {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "select-{}", self.kind)
    }
}

impl TailwindSelect {
    /// <https://tailwindcss.com/docs/user-select>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let kind = StandardValue::parser("select", &Self::check_valid)(pattern, arbitrary)?;
        Ok(Self { kind })
    }
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/user-select#syntax>
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec!["all", "auto", "contain", "inherit", "initial", "none", "revert", "text", "unset"]);
        set.contains(mode)
    }
}
