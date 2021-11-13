use super::*;

#[doc=include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindBackgroundRepeat {
    kind: BackgroundRepeat,
}

#[derive(Debug, Clone)]
enum BackgroundRepeat {
    Standard(String),
    Arbitrary(String),
}

impl<T> From<T> for TailwindBackgroundRepeat
where
    T: Into<String>,
{
    fn from(kind: T) -> Self {
        Self { kind: BackgroundRepeat::Standard(kind.into()) }
    }
}

impl Display for TailwindBackgroundRepeat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            BackgroundRepeat::Standard(s) => match s.as_str() {
                "repeat" => write!(f, "bg-repeat"),
                "no-repeat" => write!(f, "bg-no-repeat"),
                "repeat-x" => write!(f, "bg-repeat-x"),
                "repeat-y" => write!(f, "bg-repeat-y"),
                _ => write!(f, "bg-repeat-{}", s),
            },
            BackgroundRepeat::Arbitrary(s) => write!(f, "bg-repeat-[{}]", s),
        }
    }
}

impl TailwindInstance for TailwindBackgroundRepeat {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let cursor = match &self.kind {
            BackgroundRepeat::Standard(s) => s,
            BackgroundRepeat::Arbitrary(s) => s,
        };
        css_attributes! {
            "background-repeat" => cursor
        }
    }
}

impl TailwindBackgroundRepeat {
    /// https://tailwindcss.com/docs/background-repeat
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: BackgroundRepeat::parse(pattern, arbitrary)? })
    }
    /// https://tailwindcss.com/docs/background-repeat
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: BackgroundRepeat::parse_arbitrary(arbitrary)? })
    }
}

impl BackgroundRepeat {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let out = match pattern {
            [] if arbitrary.is_none() => Self::Standard("repeat".to_string()),
            [] => Self::parse_arbitrary(arbitrary)?,
            ["none"] => Self::Standard("no-repeat".to_string()),
            ["x"] => Self::Standard("repeat-x".to_string()),
            ["y"] => Self::Standard("repeat-y".to_string()),
            _ => {
                let s = pattern.join("-");
                debug_assert!(Self::check_valid(&s));
                Self::Standard(s)
            },
        };
        Ok(out)
    }
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        debug_assert!(arbitrary.is_some());
        Ok(Self::Arbitrary(arbitrary.to_string()))
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/background-repeat#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
            "inherit",
            "initial",
            "no-repeat",
            "repeat",
            "repeat-x",
            "repeat-y",
            "revert",
            "round",
            "space",
            "unset",
        ]);
        set.contains(mode)
    }
}
