use super::*;

#[derive(Copy, Clone, Debug)]
pub enum FloatKind {
    Left,
    Right,
    None,
}

#[doc = include_str!("readme.md")]
#[derive(Copy, Clone, Debug)]
pub struct TailwindFloat {
    kind: FloatKind,
}

impl Display for FloatKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Left => f.write_str("left"),
            Self::Right => f.write_str("right"),
            Self::None => f.write_str("none"),
        }
    }
}

impl Display for TailwindFloat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "float-{}", self.kind)
    }
}

impl TailwindInstance for TailwindFloat {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        css_attributes! {
            "float" => self.kind
        }
    }
}

impl TailwindFloat {
    /// https://tailwindcss.com/docs/float
    pub fn parse(kind: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        debug_assert!(arbitrary.is_none(), "forbidden arbitrary after float");
        let out = match kind {
            ["left"] => Self { kind: FloatKind::Left },
            ["right"] => Self { kind: FloatKind::Right },
            ["none"] => Self { kind: FloatKind::None },
            _ => return syntax_error!("Unknown float elements: {}", kind.join("-")),
        };
        Ok(out)
    }
}
