use std::cmp::Ordering;

use super::*;

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
            Self::Arbitrary(s) => write!(f, "[{}]", s),
        }
    }
}

impl PartialEq<usize> for IntegerOnly {
    fn eq(&self, other: &usize) -> bool {
        match self {
            Self::Number(s) => s.eq(other),
            Self::Arbitrary(_) => false,
        }
    }
}

impl PartialOrd<usize> for IntegerOnly {
    fn partial_cmp(&self, other: &usize) -> Option<Ordering> {
        match self {
            Self::Number(s) => s.partial_cmp(other),
            Self::Arbitrary(_) => None,
        }
    }
}
