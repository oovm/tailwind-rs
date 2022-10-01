use super::*;
use crate::KeywordInstance;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug, Default)]
pub struct TailwindJustifyItems {}

impl TailwindProcessor for TailwindJustifyItems {
    fn on_catch<'a, 'i>(&'a self, pattern: &'i [&'i str]) -> Option<&'i [&'i str]> {
        match pattern {
            ["items", rest @ ..] => Some(rest),
            _ => None,
        }
    }
    /// <https://tailwindcss.com/docs/justify-items>
    fn on_progress(&self, pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
        let kw = KeywordInstance { pattern: Self::PREFIX, kind: StandardValue::parser(Self::PREFIX, &Self::check_valid)(pattern, arbitrary)? };
        Ok(kw.boxed())
    }
}

impl TailwindJustifyItems {
    const PREFIX: &'static str = "justify-items";
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/justify-items#syntax>
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
            "baseline",
            "center",
            "end",
            "flex-end",
            "flex-start",
            "inherit",
            "initial",
            "left",
            "normal",
            "revert",
            "right",
            "self-end",
            "self-start",
            "start",
            "stretch",
            "unset",
        ]);
        set.contains(mode)
    }
}
