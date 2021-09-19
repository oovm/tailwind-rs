use super::*;

// https://developer.mozilla.org/en-US/docs/Web/CSS/text-align#values
#[derive(Copy, Debug, Clone)]
enum TextAlignment {
    Left,
    Center,
    Right,
    Justify,
}

#[doc = include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindTextAlignment {
    kind: TextAlignment,
}
