use super::*;

#[doc=include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindJustifyContent {
    kind: MaybeArbitrary,
}

impl<T> From<T> for TailwindJustifyContent
where
    T: Into<String>,
{
    fn from(kind: T) -> Self {
        Self { kind: MaybeArbitrary::Standard(kind.into()) }
    }
}

impl Display for TailwindJustifyContent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            MaybeArbitrary::Standard(s) => match s.as_str() {
                "flex-start" => write!(f, "justify-start"),
                "flex-end" => write!(f, "justify-end"),
                "center" => write!(f, "justify-center"),
                "space-between" => write!(f, "justify-between"),
                "space-around" => write!(f, "justify-around"),
                "space-evenly" => write!(f, "justify-evenly"),
                _ => write!(f, "justify-content-{}", s),
            },
            MaybeArbitrary::Arbitrary(s) => write!(f, "justify-content-[{}]", s),
        }
    }
}

impl TailwindInstance for TailwindJustifyContent {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        css_attributes! {
            "justify-content" => self.kind.get_properties()
        }
    }
}

impl TailwindJustifyContent {
    /// https://tailwindcss.com/docs/justify-content
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: MaybeArbitrary::parser("ease", &check_valid)(pattern, arbitrary)? })
    }
    /// https://tailwindcss.com/docs/justify-content
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: MaybeArbitrary::parse_arbitrary(arbitrary)? })
    }
}

/// https://developer.mozilla.org/en-US/docs/Web/CSS/cursor#syntax
fn check_valid(mode: &str) -> bool {
    let set = BTreeSet::from_iter(vec![
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
        "space-around",
        "space-between",
        "space-evenly",
        "start",
        "stretch",
        "unset",
    ]);
    set.contains(mode)
}
