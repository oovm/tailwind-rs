use super::*;

#[derive(Copy, Clone, Debug)]
enum ClearKind {
    Left,
    Right,
    Both,
    None,
}

#[doc = include_str!("clear.md")]
#[derive(Copy, Clone, Debug)]
pub struct TailwindClear {
    kind: ClearKind,
}

impl Display for ClearKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Left => f.write_str("left"),
            Self::Right => f.write_str("right"),
            Self::Both => f.write_str("both"),
            Self::None => f.write_str("none"),
        }
    }
}

impl Display for TailwindClear {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "clear-{}", self.kind)
    }
}

impl TailwindInstance for TailwindClear {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        css_attributes! {
            "clear" => self.kind
        }
    }
}

impl TailwindClear {
    /// https://tailwindcss.com/docs/clear
    pub fn parse(kind: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        debug_assert!(arbitrary.is_none(), "forbidden arbitrary after brightness");
        let out = match kind {
            ["left"] => Self { kind: ClearKind::Left },
            ["right"] => Self { kind: ClearKind::Right },
            ["both"] => Self { kind: ClearKind::Both },
            ["none"] => Self { kind: ClearKind::None },
            _ => return syntax_error!("Unknown clear elements: {}", kind.join("-")),
        };
        Ok(out)
    }
}
