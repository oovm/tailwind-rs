use super::*;

pub(crate) mod content_align;

#[doc = include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindContent {
    kind: Content,
}

impl<T> From<T> for TailwindContent
where
    T: Into<String>,
{
    fn from(kind: T) -> Self {
        Self { kind: Content::Standard(kind.into()) }
    }
}

#[derive(Debug, Clone)]
enum Content {
    Standard(String),
    Arbitrary(String),
}

impl Display for TailwindContent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            Content::Standard(s) => write!(f, "{}", s),
            Content::Arbitrary(s) => write!(f, "[{}]", s),
        }
    }
}

impl TailwindInstance for TailwindContent {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let content = match &self.kind {
            Content::Standard(s) => s,
            Content::Arbitrary(s) => s,
        };
        css_attributes! {
            "content" => content
        }
    }
}

impl TailwindContent {
    /// https://tailwindcss.com/docs/align-content
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
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
        Ok(Self { kind: Content::Arbitrary(arbitrary.to_string()) })
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
