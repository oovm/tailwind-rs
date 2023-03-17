use super::*;
use std::{
    cmp::Ordering,
    fmt::{Display, Formatter},
};

impl Display for TailwindVariant {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.not {
            write!(f, "not-")?
        }
        write!(f, "{}", self.names.join("-"))?;
        match self.pseudo {
            true => {
                write!(f, "::")
            }
            false => {
                write!(f, ":")
            }
        }
    }
}
impl PartialOrd for TailwindVariant {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.names.partial_cmp(&other.names)
    }
}

impl Ord for TailwindVariant {
    fn cmp(&self, other: &Self) -> Ordering {
        self.names.cmp(&other.names)
    }
}
