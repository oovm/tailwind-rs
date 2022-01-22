use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindIsolation {
    kind: StandardValue,
}

crate::macros::sealed::keyword_instance!(TailwindIsolation => "isolation");

impl Display for TailwindIsolation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.kind.write_class(f, "isolation-", |f, s| match s {
            "isolate" => write!(f, "isolate"),
            _ => Err(std::fmt::Error),
        })
    }
}

impl TailwindIsolation {
    /// https://tailwindcss.com/docs/isolation
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let kind = StandardValue::parser("isolate", &Self::check_valid)(pattern, arbitrary)?;
        Ok(Self { kind })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/isolation#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
            // Keyword values
            "auto", "isolate", // Global values
            "inherit", "initial", "revert", "unset",
        ]);
        set.contains(mode)
    }
}
