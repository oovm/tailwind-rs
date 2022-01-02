use super::*;

#[doc=include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindAlign {
    kind: VerticalAlign,
}

#[derive(Debug, Clone)]
pub enum VerticalAlign {
    Standard(String),
    Length(LengthUnit),
}

impl<T> From<T> for TailwindAlign
where
    T: Into<String>,
{
    fn from(kind: T) -> Self {
        Self { kind: VerticalAlign::Standard(kind.into()) }
    }
}

impl Display for TailwindAlign {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            VerticalAlign::Standard(s) => write!(f, "align-{}", s),
            VerticalAlign::Length(s) => write!(f, "align-[{}]", s),
        }
    }
}

impl TailwindInstance for TailwindAlign {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        let align = match &self.kind {
            VerticalAlign::Standard(s) => s.to_owned(),
            VerticalAlign::Length(s) => s.get_properties(),
        };
        css_attributes! {
            "vertical-align" => align
        }
    }
}

impl TailwindAlign {
    /// https://tailwindcss.com/docs/text-align
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        match pattern {
            [s] if Self::check_valid(s) => Ok(Self { kind: VerticalAlign::Standard(s.to_string()) }),
            [s] => {
                let n = TailwindArbitrary::from(*s).as_length_or_fraction()?;
                Ok(Self { kind: VerticalAlign::Length(n) })
            },
            [] => Self::parse_arbitrary(arbitrary),
            _ => syntax_error!("Unknown align instructions: {}", pattern.join("-")),
        }
    }
    /// https://tailwindcss.com/docs/text-align
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: VerticalAlign::Length(arbitrary.as_length_or_fraction()?) })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/vertical-align#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
            "baseline",
            "sub",
            "super",
            "text-top",
            "text-bottom",
            "middle",
            "top",
            "bottom",
            "inherit",
            "initial",
            "revert",
            "unset",
        ]);
        set.contains(mode)
    }
}
