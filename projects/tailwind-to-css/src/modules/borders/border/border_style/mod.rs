use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindBorderStyle {
    kind: String,
}

impl<T> From<T> for TailwindBorderStyle
where
    T: Into<String>,
{
    fn from(kind: T) -> Self {
        Self { kind: kind.into() }
    }
}

impl Display for TailwindBorderStyle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "border-{}", self.kind)
    }
}

impl TailwindInstance for TailwindBorderStyle {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        css_attributes! {
            "border-style" => self.kind
        }
    }
}

impl TailwindBorderStyle {
    /// https://tailwindcss.com/docs/object-fit
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        debug_assert!(arbitrary.is_none(), "forbidden arbitrary after object");
        let kind = pattern.join("-");
        debug_assert!(Self::check_valid(&kind));
        Ok(Self { kind })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/border-style#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
            "none", "hidden", "dotted", "dashed", "solid", "double", "groove", "ridge", "ridge", "inset", "outset", "inherit",
            "initial", "revert", "unset",
        ]);
        set.contains(mode)
    }
}
