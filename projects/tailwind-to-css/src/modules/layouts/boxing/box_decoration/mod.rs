use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindBoxDecoration {
    kind: String,
}

impl<T> From<T> for TailwindBoxDecoration
where
    T: Into<String>,
{
    fn from(kind: T) -> Self {
        Self { kind: kind.into() }
    }
}

impl Display for TailwindBoxDecoration {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.kind.as_str() {
            "clone" => write!(f, "box-clone"),
            "slice" => write!(f, "box-slice"),
            _ => write!(f, "box-decoration-{}", self.kind),
        }
    }
}

impl TailwindInstance for TailwindBoxDecoration {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        css_attributes! {
            "box-decoration-break" => self.kind
        }
    }
}

impl TailwindBoxDecoration {
    /// https://tailwindcss.com/docs/box-decoration-break
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        debug_assert!(arbitrary.is_none(), "forbidden arbitrary after box-decoration");
        let kind = pattern.join("-");
        debug_assert!(Self::check_valid(&kind));
        Ok(Self { kind })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/box-decoration-break#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
            // Keyword values
            "slice", "clone", // Global values
            "inherit", "initial", "revert", "unset",
        ]);
        set.contains(mode)
    }
}
