use super::*;

#[derive(Debug, Clone)]
pub enum MaybeArbitrary {
    Standard(String),
    Arbitrary(String),
}

impl Display for MaybeArbitrary {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Standard(s) => write!(f, "{}", s),
            Self::Arbitrary(s) => write!(f, "[{}]", s),
        }
    }
}

impl MaybeArbitrary {
    pub fn parser(
        id: &'static str,
        check_valid: &'static impl Fn(&str) -> bool,
    ) -> impl Fn(&[&str], &TailwindArbitrary) -> Result<Self> {
        move |pattern: &[&str], arbitrary: &TailwindArbitrary| match pattern {
            [] => Self::parse_arbitrary(arbitrary),
            _ => Self::parse_standard(pattern, id, check_valid),
        }
    }
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        debug_assert!(arbitrary.is_some());
        Ok(Self::Arbitrary(arbitrary.to_string()))
    }
    pub fn parse_standard(pattern: &[&str], id: &str, checker: &'static impl Fn(&str) -> bool) -> Result<Self> {
        let keyword = pattern.join("-");
        debug_assert!(checker(&keyword), "{} does not a valid value of {}", keyword, id);
        Ok(Self::Standard(keyword))
    }
    pub fn get_properties(&self) -> &str {
        match self {
            Self::Standard(s) => s.as_str(),
            Self::Arbitrary(s) => s.as_str(),
        }
    }
}
