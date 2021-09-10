use super::*;

#[derive(Debug, Copy, Clone)]
enum JustifyContent {
    Start,
    End,
    Center,
    Between,
    Around,
    Evenly,
    Global(CssBehavior),
}

#[doc=include_str!("readme.md")]
#[derive(Debug, Copy, Clone)]
pub struct TailwindJustifyContent {
    kind: JustifyContent,
}

impl Display for JustifyContent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Start => write!(f, "start"),
            Self::End => write!(f, "end"),
            Self::Center => write!(f, "center"),
            Self::Between => write!(f, "between"),
            Self::Around => write!(f, "around"),
            Self::Evenly => write!(f, "evenly"),
            Self::Global(g) => write!(f, "{}", g),
        }
    }
}

impl Display for TailwindJustifyContent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "justify-{}", self.kind)
    }
}

impl TailwindInstance for TailwindJustifyContent {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let content = match self.kind {
            JustifyContent::Start => "flex-start".to_string(),
            JustifyContent::End => "flex-end".to_string(),
            JustifyContent::Center => "center".to_string(),
            JustifyContent::Between => "space-between".to_string(),
            JustifyContent::Around => "space-around".to_string(),
            JustifyContent::Evenly => "space-evenly".to_string(),
            JustifyContent::Global(g) => g.to_string(),
        };
        css_attributes! {
            "justify-content" => content
        }
    }
}

impl TailwindJustifyContent {
    /// https://tailwindcss.com/docs/justify-content
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        debug_assert!(arbitrary.is_none(), "forbidden arbitrary after justify");
        let kind = match pattern {
            ["start"] => JustifyContent::Start,
            ["end"] => JustifyContent::End,
            ["center"] => JustifyContent::Center,
            ["between"] => JustifyContent::Between,
            ["around"] => JustifyContent::Around,
            ["evenly"] => JustifyContent::Evenly,
            _ => return syntax_error!("Unknown justify instructions: {}", pattern.join("-")),
        };
        Ok(Self { kind })
    }
}
