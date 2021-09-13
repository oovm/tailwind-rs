use super::*;

#[derive(Debug, Copy, Clone)]
pub(super) enum DecorationStyle {
    Solid,
    Double,
    Dotted,
    Dashed,
    Wavy,
}

#[doc = include_str!("text-decoration-style.md")]
#[derive(Debug, Copy, Clone)]
pub struct TailwindDecorationStyle {
    kind: DecorationStyle,
}

impl Display for TailwindDecorationStyle {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindDecorationStyle {}

impl Display for TailwindDecorationThickness {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
