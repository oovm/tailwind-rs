/// https://tailwindcss.com/docs/box-shadow
#[derive(Copy, Clone, Debug)]
pub struct ShadowSystem {}
/// https://tailwindcss.com/docs/opacity
#[derive(Copy, Clone, Debug)]
pub struct TailwindOpacity(usize);

/// https://tailwindcss.com/docs/mix-blend-mode
#[derive(Copy, Clone, Debug)]
pub struct TailwindBlender {
    mode: TailwindBlendMode,
}

#[derive(Copy, Clone, Debug)]
pub enum TailwindBlendKind {
    Background,
    Mixed,
}

#[derive(Copy, Clone, Debug)]
pub enum TailwindBlendMode {}
