use super::*;

#[doc = include_str!("text-decoration-color.md")]
#[derive(Debug, Clone)]
pub struct TailwindDecorationColor {
    color: TailwindColor,
}

impl From<TailwindColor> for TailwindDecorationColor {
    fn from(color: TailwindColor) -> Self {
        Self { color }
    }
}

impl Display for TailwindDecorationColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindDecorationColor {}
