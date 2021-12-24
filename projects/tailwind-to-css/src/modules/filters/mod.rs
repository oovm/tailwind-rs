use std::fmt::{Debug, Display, Formatter};

use tailwind_error::Result;

pub use self::{
    blur::TailwindBlur, brightness::TailwindBrightness, contrast::TailwindContrast, grayscale::TailwindGrayscale,
    hue_rotate::TailwindHueRotate, invert::TailwindInvert, saturate::TailwindSaturate, sepia::TailwindSepia,
};
use crate::{css_attributes, CssAttributes, IntegerOnly, TailwindArbitrary, TailwindBuilder, TailwindInstance};

mod blur;
mod brightness;
mod contrast;
mod grayscale;
mod hue_rotate;
mod invert;
mod saturate;
mod sepia;

#[derive(Clone, Debug)]
struct Backdrop(bool);

impl From<bool> for Backdrop {
    fn from(backdrop: bool) -> Self {
        Self(backdrop)
    }
}

impl Backdrop {
    fn write(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            true => {
                write!(f, "backdrop-")
            },
            false => {
                write!(f, "")
            },
        }
    }
    pub fn filter(&self) -> &'static str {
        match self.0 {
            true => "backdrop-filter",
            false => "filter",
        }
    }
}
