use super::*;

#[doc=include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindJustifySelf {
    kind: JustifySelf,
}

#[derive(Debug, Clone)]
enum JustifySelf {
    Standard(String),
    Arbitrary(String),
}

impl<T> From<T> for TailwindJustifySelf
where
    T: Into<String>,
{
    fn from(kind: T) -> Self {
        Self { kind: JustifySelf::Standard(kind.into()) }
    }
}

impl Display for TailwindJustifySelf {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            JustifySelf::Standard(s) => write!(f, "justify-self-{}", s),
            JustifySelf::Arbitrary(s) => write!(f, "justify-self-[{}]", s),
        }
    }
}

impl TailwindInstance for TailwindJustifySelf {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let cursor = match &self.kind {
            JustifySelf::Standard(s) => s,
            JustifySelf::Arbitrary(s) => s,
        };
        css_attributes! {
            "justify-self" => cursor
        }
    }
}

impl TailwindJustifySelf {
    /// https://tailwindcss.com/docs/justify-self
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        match pattern {
            [] => Self::parse_arbitrary(arbitrary),
            _ => {
                let s = pattern.join("-");
                debug_assert!(Self::check_valid(&s));
                Ok(Self { kind: JustifySelf::Standard(s) })
            },
        }
    }
    /// https://tailwindcss.com/docs/justify-self
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        debug_assert!(arbitrary.is_some());
        Ok(Self { kind: JustifySelf::Arbitrary(arbitrary.to_string()) })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/justify-self#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
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
        ]);
        set.contains(mode)
    }
}
