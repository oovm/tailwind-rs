use crate::modules::flexbox::*;

#[derive(Debug, Copy, Clone)]
enum JustifyItems {
    Start,
    End,
    Center,
    Stretch,
    Global(CssBehavior),
}

#[doc=include_str!("readme.md")]
#[derive(Debug, Copy, Clone)]
pub struct TailwindJustifyItems {
    kind: JustifyItems,
}

impl Display for JustifyItems {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Start => write!(f, "start"),
            Self::End => write!(f, "end"),
            Self::Center => write!(f, "center"),
            Self::Stretch => write!(f, "stretch"),
            Self::Global(g) => write!(f, "{}", g),
        }
    }
}

impl Display for TailwindJustifyItems {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "justify-items-{}", self.kind)
    }
}

impl TailwindInstance for TailwindJustifyItems {}

impl TailwindJustifyItems {
    /// https://tailwindcss.com/docs/justify-items
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        debug_assert!(arbitrary.is_none(), "forbidden arbitrary after justify-items");
        let kind = match pattern {
            ["start"] => JustifyItems::Start,
            ["end"] => JustifyItems::End,
            ["center"] => JustifyItems::Center,
            ["stretch"] => JustifyItems::Stretch,
            _ => return syntax_error!("Unknown justify-items instructions: {}", pattern.join("-")),
        };
        Ok(Self { kind })
    }
}
