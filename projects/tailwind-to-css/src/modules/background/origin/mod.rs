use super::*;

#[doc=include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindBackgroundOrigin {
    kind: StandardValue,
}

crate::macros::sealed::keyword_instance!(TailwindBackgroundOrigin => "background-origin");

// noinspection DuplicatedCode
impl Display for TailwindBackgroundOrigin {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "bg-origin-")?;
        match &self.kind {
            StandardValue::Keyword(s) => match s.as_str() {
                "border-box" => write!(f, "border"),
                "padding-box" => write!(f, "padding"),
                "content-box" => write!(f, "content"),
                _ => write!(f, "{}", self.kind),
            },
            StandardValue::Arbitrary(s) => write!(f, "{}", s),
        }
    }
}

// noinspection DuplicatedCode
impl TailwindBackgroundOrigin {
    /// https://tailwindcss.com/docs/background-origin
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let kind = match pattern {
            ["border"] => StandardValue::from("border-box"),
            ["padding"] => StandardValue::from("padding-box"),
            ["content"] => StandardValue::from("content-box"),
            _ => StandardValue::parser("bg-origin", &Self::check_valid)(pattern, arbitrary)?,
        };
        Ok(Self { kind })
    }
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/background-origin#syntax>
    pub fn check_valid(mode: &str) -> bool {
        let set =
            BTreeSet::from_iter(vec!["border-box", "content-box", "inherit", "initial", "padding-box", "revert", "unset"]);
        set.contains(mode)
    }
}
