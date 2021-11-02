use super::*;

#[derive(Debug, Clone)]
pub(super) enum SpaceSize {
    Unit(f32),
    Standard(String),
    Arbitrary(String),
}

impl Display for SpaceSize {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unit(s) => write!(f, "{}", s),
            Self::Standard(s) => write!(f, "{}", s),
            Self::Arbitrary(s) => write!(f, "[{}]", s),
        }
    }
}

impl SpaceSize {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        match pattern {
            [] => Self::parse_arbitrary(arbitrary),
            ["px"] => Ok(Self::Arbitrary("1px".to_string())),
            [n] if Self::check_valid(n) => Ok(Self::Standard(n.to_string())),
            [n] => Ok(Self::Unit(TailwindArbitrary::from(*n).as_float()?)),
            _ => syntax_error!("Unknown padding instructions: {}", pattern.join("-")),
        }
    }

    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        debug_assert!(arbitrary.is_some());
        Ok(Self::Arbitrary(arbitrary.to_string()))
    }
}
impl SpaceSize {
    pub fn get_properties(&self) -> String {
        match self {
            Self::Unit(x) => format!("{}rem", x / 4.0),
            Self::Standard(x) => x.to_string(),
            Self::Arbitrary(x) => x.to_string(),
        }
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/padding#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec!["auto", "inherit", "initial", "revert", "unset"]);
        set.contains(mode)
    }
}
