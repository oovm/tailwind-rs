use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindBackgroundAttachment {
    kind: StandardValue,
}

crate::macros::sealed::keyword_instance!(TailwindBackgroundAttachment => "background-attachment");

impl Display for TailwindBackgroundAttachment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "bg-")?;
        match &self.kind {
            StandardValue::Keyword(s) => match s.as_str() {
                s @ ("fixed" | "local" | "scroll") => write!(f, "{}", s),
                _ => write!(f, "attach-{}", s),
            },
            StandardValue::Arbitrary(s) => s.write_class(f, "attach-"),
        }
    }
}

impl TailwindBackgroundAttachment {
    /// <https://tailwindcss.com/docs/background-attachment>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: StandardValue::parser("bg-attach", &Self::check_valid)(pattern, arbitrary)? })
    }
    /// <https://tailwindcss.com/docs/background-attachment>
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        StandardValue::parse_arbitrary(arbitrary).map(|kind| Self { kind })
    }
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/background-attachment#syntax>
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec!["fixed", "inherit", "initial", "local", "revert", "scroll", "unset"]);
        set.contains(mode)
    }
}
