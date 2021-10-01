use crate::modules::borders::*;

#[derive(Debug, Clone)]
enum OutlineStyle {
    None,
    Default,
    Standard(String),
}

///
#[derive(Clone, Debug)]
pub struct TailwindOutlineStyle {
    kind: OutlineStyle,
}

impl<T> From<T> for TailwindOutlineStyle
where
    T: Into<String>,
{
    fn from(kind: T) -> Self {
        Self { kind: OutlineStyle::Standard(kind.into()) }
    }
}

impl Display for OutlineStyle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "-none"),
            Self::Default => write!(f, ""),
            Self::Standard(s) => match s.as_str() {
                "solid" => write!(f, ""),
                _ => write!(f, "-{}", s),
            },
        }
    }
}

impl Display for TailwindOutlineStyle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "outline{}", self.kind)
    }
}

impl TailwindInstance for TailwindOutlineStyle {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        match &self.kind {
            OutlineStyle::None => css_attributes! {
                "outline" => "2px solid transparent",
                "outline-style" => "2px"
            },
            OutlineStyle::Default => css_attributes! {
                "outline-style" => "solid"
            },
            OutlineStyle::Standard(s) => css_attributes! {
                "outline-style" => s
            },
        }
    }
}

impl TailwindOutlineStyle {
    /// `outline-none`
    pub const None: Self = Self { kind: OutlineStyle::None };
    /// `outline`
    pub const Default: Self = Self { kind: OutlineStyle::Default };
    /// https://tailwindcss.com/docs/outline-style
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        debug_assert!(arbitrary.is_none(), "forbidden arbitrary after object");
        let kind = match pattern {
            ["none"] => OutlineStyle::None,
            [] => OutlineStyle::Standard("solid".to_string()),
            _ => {
                let kind = pattern.join("-");
                debug_assert!(Self::check_valid(&kind));
                OutlineStyle::Standard(kind)
            },
        };
        Ok(Self { kind })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/outline-style#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
            "auto", "none", "dotted", "dashed", "solid", "double", "groove", "ridge", "ridge", "inset", "outset", "inherit",
            "initial", "revert", "unset",
        ]);
        set.contains(mode)
    }
}
