use std::fmt::Write;

use crate::{TailwindBreakAfter, TailwindBreakBefore, TailwindBreakInside};

use super::*;

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
                _ => write!(f, "{}", s),
            },
        }
    }
}

impl TailwindInstance for TailwindBreak {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        match &self.kind {
            WordBreak::Normal => css_attributes! {
                "overflow-wrap" => "normal",
                "word-break" => "normal"
            },
            WordBreak::Words => css_attributes! {
                "overflow-wrap" => "break-word"
            },
            WordBreak::Standard(s) => css_attributes! {
                "word-break" => s
            },
        }
    }
}

impl TailwindBreak {
    /// https://tailwindcss.com/docs/word-break
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
        let kind = match pattern {
            // https://tailwindcss.com/docs/break-before
            ["before", rest @ ..] => TailwindBreakBefore::parse(rest, arbitrary)?.boxed(),
            // https://tailwindcss.com/docs/break-inside
            ["inside", rest @ ..] => TailwindBreakInside::parse(rest, arbitrary)?.boxed(),
            // https://tailwindcss.com/docs/break-after
            ["after", rest @ ..] => TailwindBreakAfter::parse(rest, arbitrary)?.boxed(),
            // https://tailwindcss.com/docs/word-break
            _ => Self::parse_self(pattern, arbitrary)?.boxed(),
        };
        Ok(kind)
    }
    fn parse_self(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: WordBreak::parse(pattern, arbitrary)? })
    }
}

impl WordBreak {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        debug_assert!(arbitrary.is_none(), "forbidden arbitrary after break");
        let kind = match pattern {
            ["normal"] => Self::Normal,
            ["words"] => Self::Words,
            ["all"] => Self::Standard("break-all".to_string()),
            _ => {
                let kind = pattern.join("-");
                debug_assert!(Self::check_valid(&kind));
                Self::Standard(kind)
            },
        };
        Ok(kind)
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/word-break#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec!["break-all", "inherit", "initial", "keep-all", "normal", "revert", "unset"]);
        set.contains(mode)
    }
}
