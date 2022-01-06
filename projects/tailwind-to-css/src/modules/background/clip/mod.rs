use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindBackgroundClip {
    kind: StandardValue,
}

crate::macros::sealed::keyword_instance!(TailwindBackgroundClip => "background-clip");

// noinspection DuplicatedCode
impl Display for TailwindBackgroundClip {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "bg-clip-")?;
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
impl TailwindBackgroundClip {
    /// <https://tailwindcss.com/docs/background-clip>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let kind = match pattern {
            ["border"] => StandardValue::from("border-box"),
            ["padding"] => StandardValue::from("padding-box"),
            ["content"] => StandardValue::from("content-box"),
            _ => StandardValue::parser("bg-clip", &Self::check_valid)(pattern, arbitrary)?,
        };
        Ok(Self { kind })
    }
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/background-clip#syntax>
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
            "border-box",
            "content-box",
            "inherit",
            "initial",
            "padding-box",
            "revert",
            "text",
            "unset",
        ]);
        set.contains(mode)
    }
}
