mod display;
mod parser;

use super::*;

#[derive(Copy, Clone, Debug)]
enum ShadowKind {
    None,
    Small,
    Normal,
    Medium,
    Large,
    ExtraLarge,
    UltraLarge,
    Custom { x: usize, y: usize, alpha: usize },
}

/// https://tailwindcss.com/docs/box-shadow
#[derive(Copy, Clone, Debug)]
pub struct TailwindShadow {
    kind: ShadowKind,
    is_drop: bool,
}

/// https://tailwindcss.com/docs/box-shadow
#[derive(Copy, Clone, Debug)]
pub struct TailwindShadowColor {}

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
