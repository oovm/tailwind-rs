use super::*;

#[doc = include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindFontStyle {
    kind: KeywordOnly,
}

crate::macros::sealed::keyword_instance!(TailwindFontStyle => "font-style");

impl Display for TailwindFontStyle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            KeywordOnly::Standard(s) => match s.as_str() {
                "normal" => write!(f, "not-italic"),
                "italic" => write!(f, "italic"),
                _ => write!(f, "font-style-{}", s),
            },
            KeywordOnly::Arbitrary(s) => write!(f, "font-style-[{}]", s),
        }
    }
}

impl TailwindFontStyle {
    /// <https://tailwindcss.com/docs/font-style>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: KeywordOnly::parser("font-style", &Self::check_valid)(pattern, arbitrary)? })
    }
    /// dispatch to [font-style](https://developer.mozilla.org/en-US/docs/Web/CSS/font-style)
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: KeywordOnly::parse_arbitrary(arbitrary)? })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/font-style#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec!["inherit", "initial", "italic", "normal", "oblique", "revert", "unset"]);
        set.contains(mode)
    }
}
