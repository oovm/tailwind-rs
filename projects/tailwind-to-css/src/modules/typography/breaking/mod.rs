use super::*;
use std::fmt::Write;

#[doc=include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindBreak {
    kind: WordBreak,
}

#[derive(Debug, Clone)]
enum WordBreak {
    Normal,
    Words,
    Standard(String),
}

impl<T> From<T> for TailwindBreak
where
    T: Into<String>,
{
    fn from(kind: T) -> Self {
        Self { kind: WordBreak::Standard(kind.into()) }
    }
}

impl Display for TailwindBreak {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            WordBreak::Normal => write!(f, "break-normal"),
            WordBreak::Words => write!(f, "break-words"),
            WordBreak::Standard(s) => match s.as_str() {
                "break-all" => write!(f, "break-all"),
                _ => write!(f, "break-all"),
            },
        }
    }
}

impl TailwindInstance for TailwindBreak {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        css_attributes! {}
    }
}

impl TailwindBreak {
    /// https://tailwindcss.com/docs/word-break
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        debug_assert!(arbitrary.is_none(), "forbidden arbitrary after resize");
        match pattern {
            // https://tailwindcss.com/docs/word-break
            ["normal"] => Ok(Self::from("both")),
            ["words"] => Ok(Self::from("horizontal")),
            ["all"] => Ok(Self::from("vertical")),
            _ => {
                let kind = pattern.join("-");
                debug_assert!(Self::check_valid(&kind));
                Ok(Self { kind })
            },
        }
    }
}

impl WordBreak {
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/word-break#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec!["break-all", "inherit", "initial", "keep-all", "normal", "revert", "unset"]);
        set.contains(mode)
    }
}
