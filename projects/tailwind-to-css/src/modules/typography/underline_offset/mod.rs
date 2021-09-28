use std::fmt::Write;

use super::*;

#[doc = include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindUnderlineOffset {
    kind: UnderlineOffset,
}

#[derive(Debug, Clone)]
enum UnderlineOffset {
    Auto,
    Unit(usize),
    Length(LengthUnit),
}

impl Display for UnderlineOffset {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Auto => write!(f, "auto"),
            Self::Unit(n) => write!(f, "{}", n),
            Self::Length(n) => write!(f, "{}", n.get_class_arbitrary()),
        }
    }
}

impl Display for TailwindUnderlineOffset {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "underline-offset-{}", self)
    }
}

impl TailwindInstance for TailwindUnderlineOffset {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let offset = match self.kind {
            UnderlineOffset::Auto => "auto".to_string(),
            UnderlineOffset::Unit(n) => format!("{}px", n),
            UnderlineOffset::Length(n) => n.get_properties(),
        };
        css_attributes! {
            "text-underline-offset" => offset
        }
    }
}

impl TailwindUnderlineOffset {
    pub fn parse(_input: &[&str], _arbitrary: &TailwindArbitrary) -> Result<Self> {
        todo!()
    }
    pub fn parse_arbitrary(_arbitrary: &TailwindArbitrary) -> Result<Self> {
        todo!()
    }
}
