use super::*;

#[derive(Debug, Copy, Clone)]
enum DecorationStyle {
    Solid,
    Double,
    Dotted,
    Dashed,
    Wavy,
}

#[doc = include_str!("text-decoration-style.md")]
#[derive(Debug, Copy, Clone)]
pub struct TailwindDecorationStyle {
    kind: &'static str,
}
// decoration-solid	text-decoration-style: solid;
// decoration-double	text-decoration-style: double;
// decoration-dotted	text-decoration-style: dotted;
// decoration-dashed	text-decoration-style: dashed;
// decoration-wavy	text-decoration-style: wavy;

impl Display for DecorationStyle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Solid => f.write_str(),
            DecorationStyle::Double => {},
            DecorationStyle::Dotted => {},
            DecorationStyle::Dashed => {},
            DecorationStyle::Wavy => {},
        }
    }
}

impl Display for TailwindDecorationStyle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindDecorationStyle {}

impl Display for TailwindDecorationThickness {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
