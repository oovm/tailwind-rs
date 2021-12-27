use super::*;

#[derive(Debug, Clone)]
pub(super) enum SpacingSize {
    Unit(f32),
    Standard(String),
    Arbitrary(TailwindArbitrary),
}

impl Display for SpacingSize {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unit(s) => write!(f, "{}", s),
            Self::Standard(s) => write!(f, "{}", s),
            Self::Arbitrary(s) => s.write(f),
        }
    }
}

impl SpacingSize {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary, check_valid: &'static impl Fn(&str) -> bool) -> Result<Self> {
        match pattern {
            [] => Self::parse_arbitrary(arbitrary),
            ["px"] => Ok(Self::Arbitrary(TailwindArbitrary::from("1px"))),
            [n] if check_valid(n) => Ok(Self::Standard(n.to_string())),
            [n] => Ok(Self::Unit(TailwindArbitrary::from(*n).as_float()?)),
            _ => syntax_error!("Unknown padding instructions: {}", pattern.join("-")),
        }
    }

    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self::Arbitrary(TailwindArbitrary::new(arbitrary)?))
    }
}
impl SpacingSize {
    pub fn get_properties(&self) -> String {
        match self {
            Self::Unit(x) => format!("{}rem", x / 4.0),
            Self::Standard(x) => x.to_string(),
            Self::Arbitrary(x) => x.get_properties(),
        }
    }
}
