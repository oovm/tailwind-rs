use std::fmt::{Debug, Display, Formatter};

use tailwind_error::Result;

use crate::{css_attributes, CssAttributes, NumericValue, TailwindArbitrary, TailwindBuilder, TailwindInstance};

pub use self::{
    blur::TailwindBlur, brightness::TailwindBrightness, contrast::TailwindContrast, grayscale::TailwindGrayscale,
    hue_rotate::TailwindHueRotate, invert::TailwindInvert, saturate::TailwindSaturate, sepia::TailwindSepia,
};

mod blur;
mod brightness;
mod contrast;
mod grayscale;
mod hue_rotate;
mod invert;
mod saturate;
mod sepia;

#[derive(Clone, Debug)]
pub(crate) struct Backdrop(pub(crate) bool);

impl From<bool> for Backdrop {
    fn from(backdrop: bool) -> Self {
        Self(backdrop)
    }
}

impl Backdrop {
    pub fn write(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            true => {
                write!(f, "backdrop-")
            },
            false => {
                write!(f, "")
            },
        }
    }
}
