use super::*;
use std::fmt::{Display, Formatter};

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
