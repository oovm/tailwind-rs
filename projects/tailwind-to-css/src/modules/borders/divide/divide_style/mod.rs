use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindDivideStyle {
    kind: String,
}

impl<T> From<T> for TailwindDivideStyle
where
    T: Into<String>,
{
    fn from(kind: T) -> Self {
        Self { kind: kind.into() }
    }
}

impl Display for TailwindDivideStyle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "divide-{}", self.kind)
    }
}

impl TailwindInstance for TailwindDivideStyle {
    fn inlineable(&self) -> bool {
        false
    }
    fn selectors(&self, _: &TailwindBuilder) -> String {
        // format!(".divide-{} > * + *", self.kind)
        format!(".divide-{}>:not([hidden])~:not([hidden])", self.kind)
    }
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        css_attributes! {
            "border-style" => self.kind
        }
    }
}

impl TailwindDivideStyle {
    /// https://tailwindcss.com/docs/divide-style
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        debug_assert!(arbitrary.is_none(), "forbidden arbitrary after object");
        let kind = pattern.join("-");
        debug_assert!(Self::check_valid(&kind));
        Ok(Self { kind })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/border-style#syntax
    pub fn check_valid(mode: &str) -> bool {
        TailwindBorderStyle::check_valid(mode)
    }
}
