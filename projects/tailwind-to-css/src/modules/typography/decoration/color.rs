use super::*;

#[doc = include_str!("text-decoration-color.md")]
#[derive(Debug, Clone)]
pub struct TailwindDecorationColor {
    color: TailwindColor,
}

impl Display for TailwindDecorationColor {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
impl TailwindInstance for TailwindDecorationColor {}
