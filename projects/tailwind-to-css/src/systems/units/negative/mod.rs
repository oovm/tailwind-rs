use std::fmt::Write;

use super::*;

#[derive(Copy, Clone, Debug)]
pub struct Negative(bool);

impl From<bool> for Negative {
    fn from(n: bool) -> Self {
        Self(n)
    }
}

impl Negative {
    pub fn write(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            true => write!(f, "-"),
            false => write!(f, ""),
        }
    }
}
