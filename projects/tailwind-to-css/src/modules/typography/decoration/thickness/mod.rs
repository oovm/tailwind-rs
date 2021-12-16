use super::*;

#[doc=include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindDecorationThickness {
    kind: Thickness,
}

#[derive(Debug, Clone)]
enum Thickness {
    Unit(usize),
    Length(LengthUnit),
    Standard(String),
}

impl<T> From<T> for TailwindDecorationThickness
where
    T: Into<String>,
{
    fn from(kind: T) -> Self {
        Self { kind: Thickness::Standard(kind.into()) }
    }
}

impl Display for TailwindDecorationThickness {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "decoration-")?;
        match &self.kind {
            Thickness::Unit(n) => write!(f, "{}", n),
            Thickness::Length(n) => write!(f, "{}", n.get_class_arbitrary()),
            Thickness::Standard(g) => write!(f, "thick-{}", g),
        }
    }
}

impl TailwindInstance for TailwindDecorationThickness {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        let thickness = match &self.kind {
            Thickness::Unit(n) => format!("{}px", n),
            Thickness::Length(n) => n.get_properties(),
            Thickness::Standard(n) => n.to_string(),
        };
        css_attributes! {
            "text-decoration-thickness" => thickness
        }
    }
}

impl TailwindDecorationThickness {
    /// https://tailwindcss.com/docs/text-decoration-thickness
    pub fn parse(input: &str) -> Result<Self> {
        let kind = Thickness::parse(input)?;
        Ok(Self { kind })
    }
}

impl Thickness {
    pub fn parse(input: &str) -> Result<Self> {
        let a = TailwindArbitrary::from(input);
        Self::maybe_length(&a).or_else(|_| Self::maybe_no_unit(&a))
    }
    fn maybe_no_unit(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self::Unit(arbitrary.as_integer()?))
    }
    fn maybe_length(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self::Length(arbitrary.as_length()?))
    }
}
