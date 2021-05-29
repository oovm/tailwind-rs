mod builder;
mod parser;

use super::*;

/// https://tailwindcss.com/docs/box-shadow
#[derive(Copy, Clone, Debug)]
pub struct ShadowSystem {}

/// https://tailwindcss.com/docs/opacity
#[derive(Copy, Clone, Debug)]
pub struct TailwindOpacity {
    opacity: usize,
}

/// https://tailwindcss.com/docs/mix-blend-mode
#[derive(Copy, Clone, Debug)]
pub struct TailwindBlend {
    kind: TailwindBlendKind,
    mode: TailwindBlendMode,
}

#[derive(Copy, Clone, Debug)]
pub enum TailwindBlendKind {
    Normal,
    Background,
}

// https://tailwindcss.com/docs/mix-blend-mode
#[derive(Copy, Clone, Debug)]
pub enum TailwindBlendMode {
    Normal,
    Multiply,
    // TODO
}
