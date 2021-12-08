use super::*;
mod length;
use self::length::OutlineOffset;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindOutlineOffset {
    kind: OutlineOffset,
}

impl<T> From<T> for TailwindOutlineOffset
where
    T: Into<String>,
{
    fn from(kind: T) -> Self {
        Self { kind: OutlineOffset::Standard(kind.into()) }
    }
}

impl Display for TailwindOutlineOffset {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "outline-offset-{}", self.kind)
    }
}

impl TailwindInstance for TailwindOutlineOffset {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let width = match &self.kind {
            OutlineOffset::Unit(n) => format!("{}px", n),
            OutlineOffset::Standard(n) => n.to_string(),
            OutlineOffset::Length(n) => format!("{}", n),
        };
        css_attributes! {
            "outline-width" => width
        }
    }
}

impl TailwindOutlineOffset {
    /// https://tailwindcss.com/docs/outline-offset
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: OutlineOffset::parse(pattern, arbitrary)? })
    }
}
