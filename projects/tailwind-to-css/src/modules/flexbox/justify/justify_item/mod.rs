use super::*;
use crate::KeywordInstance;
use std::ops::ControlFlow;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug, Default)]
pub struct TailwindJustifyItems {}

impl TailwindProcessor for TailwindJustifyItems {
    fn on_catch(&self) -> &'static [&'static str] {
        &["items"]
    }
    /// <https://tailwindcss.com/docs/justify-items>
    fn on_progress(&self, pattern: &[&str], arbitrary: &TailwindArbitrary) -> ControlFlow<Result<Box<dyn TailwindInstance>>, ()> {
        let kw = KeywordInstance::parse(Self::PREFIX, pattern, 0, arbitrary);

        ControlFlow::Break()
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
