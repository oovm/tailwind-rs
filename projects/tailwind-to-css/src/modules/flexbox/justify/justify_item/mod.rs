use super::*;

#[doc=include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindJustifyItems {
    kind: MaybeArbitrary,
}

impl<T> From<T> for TailwindJustifyItems
where
    T: Into<String>,
{
    fn from(kind: T) -> Self {
        Self { kind: MaybeArbitrary::Standard(kind.into()) }
    }
}

impl Display for TailwindJustifyItems {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "justify-items-{}", self.kind)
    }
}

impl TailwindInstance for TailwindJustifyItems {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        css_attributes! {
            "justify-items" => self.kind.get_properties()
        }
    }
}

impl TailwindJustifyItems {
    /// https://tailwindcss.com/docs/justify-items
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: MaybeArbitrary::parser("justify-self", &check_valid)(pattern, arbitrary)? })
    }
    /// https://tailwindcss.com/docs/justify-items
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: MaybeArbitrary::parse_arbitrary(arbitrary)? })
    }
}

/// https://developer.mozilla.org/en-US/docs/Web/CSS/justify-items#syntax
fn check_valid(mode: &str) -> bool {
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
