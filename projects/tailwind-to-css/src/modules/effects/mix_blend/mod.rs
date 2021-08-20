/// https://tailwindcss.com/docs/mix-blend-mode
#[derive(Copy, Clone, Debug)]
pub struct TailwindBlend {
    kind: TailwindBlendMode,
}

pub struct TailwindBackgroundBlend {
    blend: TailwindBlend,
}

// https://tailwindcss.com/docs/mix-blend-mode
#[derive(Copy, Clone, Debug)]
pub enum TailwindBlendMode {
    Normal,
    Multiply,
    // TODO
}
