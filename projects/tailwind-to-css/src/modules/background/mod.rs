use super::*;
mod attachment;
mod builder;
mod clip;
mod display;
mod origin;
#[cfg(test)]
mod test;

pub use self::{attachment::TailwindBackgroundAttachment, clip::TailwindBackgroundClip, origin::TailwindBackgroundOrigin};

#[doc = include_str ! ("aspect-ratio.md")]
#[derive(Clone, Debug)]
pub struct TailwindBackgroundColor {
    pub(crate) color: ColorResolver,
}

// https://tailwindcss.com/docs/background-origin
#[derive(Clone, Debug)]
pub struct TailwindBackgroundPosition {
    position: TailwindObjectPosition,
}

#[doc = include_str ! ("aspect-ratio.md")]
#[derive(Clone, Debug)]
pub struct TailwindBackgroundBrightness {
    brightness: TailwindBrightness,
}
