use crate::MaybeArbitrary;

use super::*;

#[doc=include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindBackgroundRepeat {
    kind: MaybeArbitrary,
}

impl<T> From<T> for TailwindBackgroundRepeat
where
    T: Into<String>,
{
    fn from(kind: T) -> Self {
        Self { kind: MaybeArbitrary::Standard(kind.into()) }
    }
}

impl Display for TailwindBackgroundRepeat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            MaybeArbitrary::Standard(s) => match s.as_str() {
                "repeat" => write!(f, "bg-repeat"),
                "no-repeat" => write!(f, "bg-no-repeat"),
                "repeat-x" => write!(f, "bg-repeat-x"),
                "repeat-y" => write!(f, "bg-repeat-y"),
                _ => write!(f, "bg-repeat-{}", s),
            },
            MaybeArbitrary::Arbitrary(s) => write!(f, "bg-repeat-[{}]", s),
        }
    }
}

impl TailwindInstance for TailwindBackgroundRepeat {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        css_attributes! {
            "background-repeat" => self.kind.get_properties()
        }
    }
}

impl TailwindBackgroundRepeat {
    /// https://tailwindcss.com/docs/background-repeat
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: parser(pattern, arbitrary)? })
    }
    /// https://tailwindcss.com/docs/background-repeat
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: MaybeArbitrary::parse_arbitrary(arbitrary)? })
    }
}

pub fn parser(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<MaybeArbitrary> {
    let out = match pattern {
        [] if arbitrary.is_none() => MaybeArbitrary::Standard("repeat".to_string()),
        [] => MaybeArbitrary::parse_arbitrary(arbitrary)?,
        ["none"] => MaybeArbitrary::Standard("no-repeat".to_string()),
        ["x"] => MaybeArbitrary::Standard("repeat-x".to_string()),
        ["y"] => MaybeArbitrary::Standard("repeat-y".to_string()),
        _ => MaybeArbitrary::parse_standard(pattern, "bg-repeat", &check_valid)?,
    };
    Ok(out)
}

/// https://developer.mozilla.org/en-US/docs/Web/CSS/background-repeat#syntax
fn check_valid(mode: &str) -> bool {
    let set = BTreeSet::from_iter(vec![
        "inherit",
        "initial",
        "no-repeat",
        "repeat",
        "repeat-x",
        "repeat-y",
        "revert",
        "round",
        "space",
        "unset",
    ]);
    set.contains(mode)
}
