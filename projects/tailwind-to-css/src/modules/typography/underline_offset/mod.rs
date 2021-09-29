use super::*;

#[doc = include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindUnderlineOffset {
    kind: UnderlineOffset,
}

#[derive(Debug, Clone)]
enum UnderlineOffset {
    Auto,
    Unit(usize),
    Length(LengthUnit),
}

impl Display for UnderlineOffset {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Auto => write!(f, "auto"),
            Self::Unit(n) => write!(f, "{}", n),
            Self::Length(n) => write!(f, "{}", n.get_class_arbitrary()),
        }
    }
}

impl Display for TailwindUnderlineOffset {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "underline-offset-{}", self)
    }
}

impl TailwindInstance for TailwindUnderlineOffset {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let offset = match self.kind {
            UnderlineOffset::Auto => "auto".to_string(),
            UnderlineOffset::Unit(n) => format!("{}px", n),
            UnderlineOffset::Length(n) => n.get_properties(),
        };
        css_attributes! {
            "text-underline-offset" => offset
        }
    }
}

impl TailwindUnderlineOffset {
    /// https://tailwindcss.com/docs/text-underline-offset
    pub fn parse(input: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        match input {
            ["auto"] => Ok(Self { kind: UnderlineOffset::Auto }),
            [] => Self::parse_arbitrary(arbitrary),
            [n] => {
                let a = TailwindArbitrary::from(*n);
                Self::parse_arbitrary(&a)
            },
            _ => syntax_error!("Unknown opacity instructions: {}", input.join("-")),
        }
    }
    /// https://tailwindcss.com/docs/text-underline-offset#arbitrary-values
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Self::maybe_no_unit(arbitrary).or_else(|_| Self::maybe_length(arbitrary))
    }
    fn maybe_length(arbitrary: &TailwindArbitrary) -> Result<Self> {
        let n = arbitrary.as_length()?;
        Ok(Self { kind: UnderlineOffset::Length(n) })
    }
    fn maybe_no_unit(arbitrary: &TailwindArbitrary) -> Result<Self> {
        let n = arbitrary.as_integer()?;
        Ok(Self { kind: UnderlineOffset::Unit(n) })
    }
}
