mod display;
mod parser;
use super::*;

#[doc = include_str!("blur.md")]
#[derive(Clone, Debug)]
pub struct TailwindBlur {
    px: usize,
    backdrop: bool,
}

#[doc = include_str!("brightness.md")]
#[derive(Clone, Debug)]
pub struct TailwindBrightness {
    percent: usize,
    backdrop: bool,
}

#[doc = include_str!("contrast.md")]
#[derive(Clone, Debug)]
pub struct TailwindContrast {
    percent: usize,
    backdrop: bool,
}

#[doc = include_str!("grayscale.md")]
#[derive(Clone, Debug)]
pub struct TailwindGrayscale {
    percent: usize,
    backdrop: bool,
}

#[doc = include_str!("grayscale.md")]
#[derive(Clone, Debug)]
pub struct TailwindHueRotate {
    deg: usize,
    backdrop: bool,
}

#[doc = include_str!("invert.md")]
#[derive(Clone, Debug)]
pub struct TailwindInvert {
    percent: usize,
    backdrop: bool,
}

#[doc = include_str!("saturate.md")]
#[derive(Clone, Debug)]
pub struct TailwindSaturate {
    percent: usize,
    backdrop: bool,
}

#[doc = include_str!("sepia.md")]
#[derive(Clone, Debug)]
pub struct TailwindSepia {
    percent: usize,
    backdrop: bool,
}
