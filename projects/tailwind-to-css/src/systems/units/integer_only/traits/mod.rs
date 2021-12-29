use std::cmp::Ordering;

use super::*;

impl From<usize> for NumericValue {
    fn from(kind: usize) -> Self {
        Self::Number(kind as f32, Negative::from(false))
    }
}

impl Display for NumericValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(number, _) => write!(f, "{}", number.abs()),
            Self::Standard(value) => write!(f, "{}", value),
            Self::Arbitrary(value) => value.write(f),
        }
    }
}

impl PartialEq<usize> for NumericValue {
    fn eq(&self, other: &usize) -> bool {
        match self {
            Self::Number(n, _) => n.eq(&(*other as f32)),
            _ => false,
        }
    }
}

impl PartialOrd<usize> for NumericValue {
    fn partial_cmp(&self, other: &usize) -> Option<Ordering> {
        match self {
            Self::Number(n, _) => n.partial_cmp(&(*other as f32)),
            _ => None,
        }
    }
}
