use super::*;

#[doc = include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindBorderCollapse {
    kind: String,
}

impl<T> From<T> for TailwindBorderCollapse
where
    T: Into<String>,
{
    fn from(kind: T) -> Self {
        Self { kind: kind.into() }
    }
}

impl Display for TailwindBorderCollapse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.kind.as_str() {
            "" => {},
            _ => {},
        }
        write!(f, "border-{}", self.kind)
    }
}

impl TailwindInstance for TailwindBorderCollapse {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        css_attributes! {
            "border-collapse" => self.kind
        }
    }
}

impl TailwindBorderCollapse {
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/border-style#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec!["collapse", "separate", "initial", "revert", "unset"]);
        set.contains(mode)
    }
}
