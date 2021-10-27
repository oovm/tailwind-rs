use super::*;
mod length;
use self::length::OutlineOffset;

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
        match &self.kind {
            OutlineOffset::Unit(s) => write!(f, "outline-{}", s),
            OutlineOffset::Standard(s) => write!(f, "outline-offset-{}", s),
            OutlineOffset::Length(s) => write!(f, "outline-offset-{}", s),
        }
    }
}

impl TailwindInstance for TailwindOutlineOffset {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let width = match &self.kind {
            OutlineOffset::Unit(n) => format!("{}px", n),
            OutlineOffset::Standard(n) => format!("{}", n),
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
