use super::*;

pub(crate) mod content_align;

#[doc=include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindContent {
    kind: StandardValue,
}

crate::macros::sealed::keyword_instance!(TailwindContent => "content");

impl Display for TailwindContent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.kind.fmt(f)
    }
}

impl TailwindContent {
    /// https://tailwindcss.com/docs/align-content
    pub fn adapt(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
        let instance = match pattern {
            // https://tailwindcss.com/docs/content#arbitrary-values
            [] if arbitrary.is_some() => TailwindContent::parse_arbitrary(arbitrary)?.boxed(),
            // https://tailwindcss.com/docs/align-content
            [s @ ("center" | "start" | "end" | "between" | "around" | "evenly")] => TailwindContentAlign::from(*s).boxed(),
            ["align", rest @ ..] => TailwindContentAlign::parse(rest, arbitrary)?.boxed(),
            // https://tailwindcss.com/docs/content
            _ => {
                let s = pattern.join("-");
                debug_assert!(Self::check_valid(&s));
                TailwindContent::from(s).boxed()
            },
        };
        Ok(instance)
    }
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: StandardValue::Arbitrary(arbitrary.to_owned()) })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/content#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
            "close-quote",
            "inherit",
            "initial",
            "no-close-quote",
            "none",
            "no-open-quote",
            "normal",
            "open-quote",
            "revert",
            "unset",
        ]);
        set.contains(mode)
    }
}
