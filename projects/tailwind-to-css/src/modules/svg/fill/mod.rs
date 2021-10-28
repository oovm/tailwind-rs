use super::*;
use crate::TailwindInstance;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct TailwindFillColor {
    color: TailwindColor,
}

impl TailwindFillColor {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        todo!()
    }
}

impl Display for TailwindFillColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindFillColor {}
