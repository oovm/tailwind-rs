use super::*;

#[doc=include_str!("readme.md")]
#[derive(Copy, Clone, Debug, Default)]
pub struct TailwindJustifySelf {}

impl TailwindProcessor for TailwindJustifySelf {
    fn on_catch(&self) -> &'static [&'static str] {
        &["items"]
    }
    /// <https://tailwindcss.com/docs/justify-self>
    fn on_progress(&self, pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
        let mut kw = KeywordInstance::default();
        kw.parse("justify-items", pattern, Self::KEYWORDS, arbitrary)?;
        Ok(kw.boxed())
    }
}

impl TailwindJustifySelf {
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/justify-self#syntax>
    pub const KEYWORDS: KeywordMap = &[
        "auto",
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
    ];
}
