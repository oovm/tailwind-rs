use crate::StandardValue;

use super::*;

#[doc=include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindBackgroundRepeat {
    kind: StandardValue,
}

crate::macros::sealed::keyword_instance!(TailwindBackgroundRepeat => "background-repeat");

impl Display for TailwindBackgroundRepeat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            StandardValue::Keyword(s) => match s.as_str() {
                "repeat" => write!(f, "bg-repeat"),
                "no-repeat" => write!(f, "bg-no-repeat"),
                "repeat-x" => write!(f, "bg-repeat-x"),
                "repeat-y" => write!(f, "bg-repeat-y"),
                _ => write!(f, "bg-repeat-{}", s),
            },
            StandardValue::Arbitrary(s) => write!(f, "bg-repeat-{}", s.get_class()),
        }
    }
}

impl TailwindBackgroundRepeat {
    /// https://tailwindcss.com/docs/background-repeat
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: parse_kind(pattern, arbitrary)? })
    }
    /// https://tailwindcss.com/docs/background-repeat
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        StandardValue::parse_arbitrary(arbitrary).map(|kind| Self { kind })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/background-repeat#syntax
    pub fn check_valid(mode: &str) -> bool {
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
}

fn parse_kind(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<StandardValue> {
    let out = match pattern {
        [] if arbitrary.is_none() => StandardValue::from("repeat"),
        [] => StandardValue::parse_arbitrary(arbitrary)?,
        ["none"] => StandardValue::from("no-repeat"),
        ["x"] => StandardValue::from("repeat-x"),
        ["y"] => StandardValue::from("repeat-y"),
        _ => StandardValue::parse_keyword(pattern, "bg-repeat", &TailwindBackgroundRepeat::check_valid)?,
    };
    Ok(out)
}
