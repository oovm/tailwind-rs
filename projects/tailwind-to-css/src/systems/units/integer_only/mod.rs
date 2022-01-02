use super::*;

mod traits;

/// Used to represent those attributes that only have integers
#[derive(Debug, Clone)]
pub enum NumericValue {
    Number(f32, Negative),
    Keyword(String),
    Arbitrary(TailwindArbitrary),
}

impl NumericValue {
    pub fn get_properties(&self, number: impl FnOnce(&f32) -> String) -> String {
        match self {
            Self::Number(n, _) => number(n),
            Self::Keyword(s) => s.to_string(),
            Self::Arbitrary(s) => s.get_properties(),
        }
    }
    pub fn write_negative(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(n, neg) if n.ne(&0.0) && neg.eq(&true) => write!(f, "-"),
            _ => write!(f, ""),
        }
    }
    pub fn write_class(&self, f: &mut Formatter, before: &str) -> std::fmt::Result {
        write!(f, "{}{}", before, self)
    }
}

impl NumericValue {
    pub fn negative_parser(
        id: &'static str,
        checker: impl Fn(&str) -> bool,
    ) -> impl Fn(&[&str], &TailwindArbitrary, Negative) -> Result<Self> {
        move |pattern: &[&str], arbitrary: &TailwindArbitrary, negative: Negative| match pattern {
            [] => Self::parse_arbitrary(arbitrary),
            [n] if checker(n) => Ok(Self::Keyword(n.to_string())),
            [n] => Self::parse_number(n, negative),
            _ => Err(TailwindError::syntax_error(format!("Unknown {} pattern", id))),
        }
    }
    pub fn positive_parser(id: &'static str) -> impl Fn(&[&str], &TailwindArbitrary) -> Result<Self> {
        move |pattern: &[&str], arbitrary: &TailwindArbitrary| match pattern {
            [] => Self::parse_arbitrary(arbitrary),
            [n] => {
                let i = TailwindArbitrary::from(*n).as_integer()?;
                Ok(Self::Number(i as f32, Negative::from(false)))
            },
            _ => Err(TailwindError::syntax_error(format!("Unknown {} pattern", id))),
        }
    }
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self::Arbitrary(TailwindArbitrary::new(arbitrary)?))
    }
    pub fn parse_number(n: &str, negative: Negative) -> Result<Self> {
        let mut i = TailwindArbitrary::from(n).as_float()?;
        if negative.unwrap() {
            i = -i
        }
        Ok(Self::Number(i, negative))
    }
}
