use super::*;

/// Used to represent those attributes that only have integers
#[derive(Debug, Clone)]
pub enum IntegerOnly {
    Number(usize),
    Keywords(String),
    Arbitrary(String),
}

impl<T> From<T> for IntegerOnly
where
    T: Into<usize>,
{
    fn from(kind: T) -> Self {
        Self::Number(kind.into())
    }
}

impl Display for IntegerOnly {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(s) => write!(f, "{}", s),
            Self::Keywords(s) => write!(f, "{}", s),
            Self::Arbitrary(s) => write!(f, "[{}]", s),
        }
    }
}

impl IntegerOnly {
    pub fn parser(
        id: &'static str,
        check_valid: &'static impl Fn(&str) -> bool,
    ) -> impl Fn(&[&str], &TailwindArbitrary) -> Result<Self> {
        move |pattern: &[&str], arbitrary: &TailwindArbitrary| match pattern {
            [] => Self::parse_arbitrary(arbitrary),
            [n] if check_valid(n) => Ok(Self::Keywords(n.to_string())),
            [n] => Ok(Self::Number(TailwindArbitrary::from(*n).as_integer()?)),
            _ => syntax_error!("Unknown {id} pattern"),
        }
    }
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        debug_assert!(arbitrary.is_some());
        Ok(Self::Arbitrary(arbitrary.to_string()))
    }
}
