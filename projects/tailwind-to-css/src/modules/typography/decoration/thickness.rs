// decoration-0	text-decoration-thickness: 0px;
// decoration-1	text-decoration-thickness: 1px;
// decoration-2	text-decoration-thickness: 2px;
// decoration-4	text-decoration-thickness: 4px;
// decoration-8	text-decoration-thickness: 8px;

use super::*;

#[derive(Debug, Clone)]
enum Thickness {
    Auto,
    FromFont,
    Unit(usize),
    Length(LengthUnit),
    Global(CssBehavior),
}

#[doc = include_str!("text-decoration-thickness.md")]
#[derive(Debug, Clone)]
pub struct TailwindDecorationThickness {
    kind: Thickness,
}

impl Display for Thickness {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Thickness::Auto => write!(f, "auto"),
            Thickness::FromFont => write!(f, "from-font"),
            Thickness::Unit(n) => write!(f, "{}", n),
            Thickness::Length(n) => write!(f, "{}", n.get_class_arbitrary()),
            Thickness::Global(g) => write!(f, "thick-{}", g),
        }
    }
}

impl Display for TailwindDecorationThickness {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "decoration-{}", self.kind)
    }
}

impl TailwindInstance for TailwindDecorationThickness {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let thickness = match self.kind {
            Thickness::Auto => "auto".to_string(),
            Thickness::FromFont => "auto".to_string(),
            Thickness::Unit(n) => format!("{}px", n),
            Thickness::Length(n) => n.get_properties(),
            Thickness::Global(g) => format!("{}", g),
        };
        css_attributes! {
            "text-decoration-thickness" => thickness
        }
    }
}

impl TailwindDecorationThickness {
    /// `decoration-auto`
    pub const Auto: Self = Self { kind: Thickness::Auto };
    /// `decoration-from-font`
    pub const FromFont: Self = Self { kind: Thickness::FromFont };
    /// https://tailwindcss.com/docs/text-decoration-thickness
    pub fn parse(input: &str) -> Result<Self> {
        let a = TailwindArbitrary::from(input);
        let maybe_unit = || -> Result<Self> { Ok(Self { kind: Thickness::Length(LengthUnit::Px(a.as_integer()?)) }) };
        let maybe_length = || -> Result<Self> { Ok(Self { kind: Thickness::Length(a.as_length()?) }) };
        maybe_length().or_else(|_| maybe_unit())
    }
}
