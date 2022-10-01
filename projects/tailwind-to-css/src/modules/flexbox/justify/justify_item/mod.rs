use super::*;

#[doc=include_str!("readme.md")]
#[derive(Copy, Clone, Debug, Default)]
pub struct TailwindJustifyItems {}

impl TailwindProcessor for TailwindJustifyItems {
    fn on_catch(&self) -> &'static [&'static str] {
        &["items"]
    }
    /// <https://tailwindcss.com/docs/justify-items>
    fn on_progress(&self, pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
        let mut kw = KeywordInstance::default();
        kw.parse(Self::PREFIX, pattern, Self::KEYWORDS, arbitrary)?;
        Ok(kw.boxed())
    }
}

impl TailwindJustifyItems {
    pub const PREFIX: &'static str = "justify-items";
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/justify-items#syntax>
    pub const KEYWORDS: KeywordMap = &[
        ("baseline", None),
        ("center", None),
        ("end", None),
        ("flex-end", None),
        ("flex-start", None),
        ("inherit", None),
        ("initial", None),
        ("left", None),
        ("normal", None),
        ("revert", None),
        ("right", None),
        ("self-end", None),
        ("self-start", None),
        ("start", None),
        ("stretch", None),
        ("unset", None),
    ];
}
