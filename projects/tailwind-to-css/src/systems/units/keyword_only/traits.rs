use super::*;

impl From<&str> for StandardValue {
    fn from(kind: &str) -> Self {
        Self::Keyword(kind.into())
    }
}

impl From<String> for StandardValue {
    fn from(kind: String) -> Self {
        Self::Keyword(kind)
    }
}

impl From<TailwindArbitrary> for StandardValue {
    fn from(kind: TailwindArbitrary) -> Self {
        Self::Arbitrary(kind)
    }
}

impl Display for StandardValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Keyword(s) => write!(f, "{}", s),
            Self::Arbitrary(s) => s.write(f),
        }
    }
}
