use super::*;

#[derive(Debug, Copy, Clone)]
pub enum JustifySelf {
    Auto,
    Start,
    End,
    Center,
    Stretch,
    Global(CssBehavior),
}

#[doc=include_str!("readme.md")]
#[derive(Debug, Copy, Clone)]
pub struct TailwindJustifySelf {
    kind: JustifySelf,
}

impl Display for JustifySelf {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            JustifySelf::Auto => write!(f, "auto"),
            JustifySelf::Start => write!(f, "start"),
            JustifySelf::End => write!(f, "end"),
            JustifySelf::Center => write!(f, "center"),
            JustifySelf::Stretch => write!(f, "stretch"),
            JustifySelf::Global(g) => write!(f, "{}", g),
        }
    }
}

impl Display for TailwindJustifySelf {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "justify-self-{}", self.kind)
    }
}

impl TailwindInstance for TailwindJustifySelf {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        css_attributes! {
            "justify-self" => self.kind
        }
    }
}

impl TailwindJustifySelf {
    /// https://tailwindcss.com/docs/justify-self
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        debug_assert!(arbitrary.is_none(), "forbidden arbitrary after justify-self");
        let kind = match pattern {
            ["auto"] => JustifySelf::Auto,
            ["start"] => JustifySelf::Start,
            ["end"] => JustifySelf::End,
            ["center"] => JustifySelf::Center,
            ["stretch"] => JustifySelf::Stretch,
            _ => return syntax_error!("Unknown contrast instructions: {}", pattern.join("-")),
        };
        Ok(Self { kind })
    }
}
