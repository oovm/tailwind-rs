use crate::KeywordOnly;

use super::*;

#[doc=include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindBackgroundRepeat {
    kind: KeywordOnly,
}

impl<T> From<T> for TailwindBackgroundRepeat
where
    T: Into<String>,
{
    fn from(kind: T) -> Self {
        Self { kind: KeywordOnly::Standard(kind.into()) }
    }
}

impl Display for TailwindBackgroundRepeat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            KeywordOnly::Standard(s) => match s.as_str() {
                "repeat" => write!(f, "bg-repeat"),
                "no-repeat" => write!(f, "bg-no-repeat"),
                "repeat-x" => write!(f, "bg-repeat-x"),
                "repeat-y" => write!(f, "bg-repeat-y"),
                _ => write!(f, "bg-repeat-{}", s),
            },
            KeywordOnly::Arbitrary(s) => write!(f, "bg-repeat-[{}]", s),
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
        Ok(Self { kind: KeywordOnly::parse_arbitrary(arbitrary)? })
    }
}

pub fn parser(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<KeywordOnly> {
    let out = match pattern {
        [] if arbitrary.is_none() => KeywordOnly::Standard("repeat".to_string()),
        [] => KeywordOnly::parse_arbitrary(arbitrary)?,
        ["none"] => KeywordOnly::Standard("no-repeat".to_string()),
        ["x"] => KeywordOnly::Standard("repeat-x".to_string()),
        ["y"] => KeywordOnly::Standard("repeat-y".to_string()),
        _ => KeywordOnly::parse_standard(pattern, "bg-repeat", &check_valid)?,
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
