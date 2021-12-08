use super::*;
mod length;
use self::length::OutlineWidth;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindOutlineWidth {
    kind: OutlineWidth,
}

impl<T> From<T> for TailwindOutlineWidth
where
    T: Into<String>,
{
    fn from(kind: T) -> Self {
        Self { kind: OutlineWidth::Standard(kind.into()) }
    }
}

impl Display for TailwindOutlineWidth {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            OutlineWidth::Unit(s) => write!(f, "outline-{}", s),
            OutlineWidth::Standard(s) => write!(f, "outline-width-{}", s),
            OutlineWidth::Length(s) => write!(f, "outline-width-{}", s),
        }
    }
}

impl TailwindInstance for TailwindOutlineWidth {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let width = match &self.kind {
            OutlineWidth::Unit(n) => format!("{}px", n),
            OutlineWidth::Standard(n) => n.to_string(),
            OutlineWidth::Length(n) => format!("{}", n),
        };
        css_attributes! {
            "outline-width" => width
        }
    }
}

impl TailwindOutlineWidth {
    /// https://tailwindcss.com/docs/outline-offset
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: OutlineWidth::parse(pattern, arbitrary)? })
    }
}
