use crate::NumericValue;

use super::*;

#[doc=include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindDecorationThickness {
    px: NumericValue,
}

impl<T> From<T> for TailwindDecorationThickness
where
    T: Into<NumericValue>,
{
    fn from(value: T) -> Self {
        Self { px: value.into() }
    }
}

impl Display for TailwindDecorationThickness {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "decoration-")?;

        match &self.px {
            NumericValue::Number { n, .. } => write!(f, "{}", n),
            NumericValue::Keyword(s) => match s.as_str() {
                "from-font" | "auto" => write!(f, "{}", s),
                _ => write!(f, "thick-{}", s),
            },
            NumericValue::Arbitrary(s) => s.write_class(f, "thick-"),
        }
    }
}

impl TailwindInstance for TailwindDecorationThickness {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        let thick = self.px.get_properties(|f| format!("{}px", f));
        css_attributes! {
            "text-decoration-thickness" => thick
        }
    }
}

impl TailwindDecorationThickness {
    /// <https://tailwindcss.com/docs/text-decoration-thickness>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let kind = NumericValue::positive_parser("decoration-thick", Self::check_valid)(pattern, arbitrary)?;
        Ok(Self { px: kind })
    }
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/text-decoration-thickness#syntax>
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec!["auto", "from-font", "inherit", "initial", "revert", "unset"]);
        set.contains(mode)
    }
}
