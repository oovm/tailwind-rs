use super::*;

#[derive(Debug, Clone)]
enum Basis {
    Number(f32),
    Length(LengthUnit),
    Standard(String),
    Arbitrary(TailwindArbitrary),
}

#[doc=include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindBasis {
    kind: Basis,
}

impl Display for TailwindBasis {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "basis-")?;
        match &self.kind {
            Basis::Number(n) => write!(f, "{}", n),
            Basis::Length(n) if n.is_fraction() => write!(f, "{}", n.get_class()),
            Basis::Length(n) => write!(f, "{}", n.get_class_arbitrary()),
            Basis::Standard(s) => match s.as_str() {
                "fit-content" => write!(f, "fit"),
                "min-content" => write!(f, "min"),
                "max-content" => write!(f, "max"),
                _ => write!(f, "{}", s),
            },
            Basis::Arbitrary(s) => s.write(f),
        }
    }
}

impl TailwindInstance for TailwindBasis {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        let basis = match &self.kind {
            Basis::Number(n) => format!("{}rem", *n as f32 / 4.0),
            Basis::Length(n) => n.get_properties(),
            Basis::Standard(s) => s.to_string(),
            Basis::Arbitrary(s) => s.get_properties(),
        };
        css_attributes! {
            "flex-basis" => basis
        }
    }
}

impl TailwindBasis {
    /// <https://tailwindcss.com/docs/flex-basis>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: Basis::parse(pattern, arbitrary)? })
    }

    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: Basis::parse_arbitrary(arbitrary)? })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/flex-basis#syntax
    pub fn check_valid(mode: &str) -> bool {
        Basis::check_valid(mode)
    }
}

impl Basis {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let out = match pattern {
            ["px"] => Self::Length(LengthUnit::px(1.0)),
            ["full"] => Self::Length(LengthUnit::Fraction(1, 1)),
            ["fit" | "min" | "max", "content"] => Self::Standard(pattern.join("-")),
            [s @ ("fit" | "min" | "max")] => Self::Standard(format!("{}-content", s)),
            [s] if Self::check_valid(s) => Self::Standard(s.to_string()),
            [n] => {
                let a = TailwindArbitrary::from(*n);
                Self::maybe_length(&a).or_else(|_| Self::maybe_float(&a))?
            },
            [] => Self::parse_arbitrary(arbitrary)?,
            _ => return syntax_error!("Unknown basis instructions"),
        };
        Ok(out)
    }
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self::Arbitrary(TailwindArbitrary::new(arbitrary)?))
    }
    fn maybe_float(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self::Number(arbitrary.as_float()?))
    }
    fn maybe_length(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self::Length(arbitrary.as_length_or_fraction()?))
    }
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
            "auto",
            "content",
            "fill",
            "fit-content",
            "inherit",
            "initial",
            "max-content",
            "min-content",
            "revert",
            "unset",
        ]);
        set.contains(mode)
    }
}
