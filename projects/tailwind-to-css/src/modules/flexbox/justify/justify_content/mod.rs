use super::*;

#[doc=include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindJustifyContent {
    kind: String,
}

enum JustifyContent {}

impl<T> From<T> for TailwindJustifyContent
where
    T: Into<String>,
{
    fn from(kind: T) -> Self {
        Self { kind: kind.into() }
    }
}

impl Display for TailwindJustifyContent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = self.kind.as_str();
        match s {
            "flex-start" => write!(f, "justify-start"),
            "flex-end" => write!(f, "justify-end"),
            "center" => write!(f, "justify-center"),
            "space-between" => write!(f, "justify-between"),
            "space-around" => write!(f, "justify-around"),
            "space-evenly" => write!(f, "justify-evenly"),
            _ => write!(f, "justify-content-{}", self.kind),
        }
    }
}

impl TailwindInstance for TailwindJustifyContent {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        css_attributes! {
            "justify-content" => self.kind
        }
    }
}

impl TailwindJustifyContent {
    /// https://tailwindcss.com/docs/justify-content
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        debug_assert!(arbitrary.is_none(), "forbidden arbitrary after justify-content");
        let out = match pattern {
            ["safe", "center"] => Self::from("safe center"),
            ["unsafe", "center"] => Self::from("unsafe center"),
            _ => {
                let kind = pattern.join("-");
                debug_assert!(Self::check_valid(&kind));
                Self { kind }
            },
        };
        Ok(out)
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/justify-content#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
            "all",
            "auto",
            "fill",
            "inherit",
            "initial",
            "none",
            "painted",
            "revert",
            "stroke",
            "unset",
            "visible",
            "visibleFill",
            "visiblePainted",
            "visibleStroke",
        ]);
        set.contains(mode)
    }
}
