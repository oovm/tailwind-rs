use super::*;

use crate::{TailwindBackgroundColor, TailwindRingColor, TailwindTextColor};

impl Default for PaletteSystem {
    fn default() -> Self {
        Self { gradient: false, inner: Default::default() }
    }
}

impl Display for Palette {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Display for ColorResolver {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Inherit => {}
            Self::Current => {}
            Self::Transparent => {}
            Self::Black => {}
            Self::White => {}
            Self::Themed { name, weight } => {}
        }
    }
}

macro_rules! color_wrapper {
    ($t:ty) => {
        impl $t {
            pub const INHERIT: Self = Self { color: ColorResolver::Inherit };
            pub const CURRENT: Self = Self { color: ColorResolver::Current };
            pub const TRANSPARENT: Self = Self { color: ColorResolver::Transparent };
            pub const BLACK: Self = Self { color: ColorResolver::Black };
            pub const WHITE: Self = Self { color: ColorResolver::White };
        }

        impl $t {
            #[inline]
            pub fn parse(name: &str, weight: &str) -> Result<Self> {
                let w = parse_integer(weight)?;
                Ok(Self { color: ColorResolver::new(name, w.1) })
            }
            #[inline]
            pub fn new(name: String, weight: usize) -> Result<Self> {
                Ok(Self { color: ColorResolver::new(name, weight) })
            }
        }
    };
    ($($t:ty),+ $(,)?) => {
        $(color_wrapper!($t);)+
    };
}

color_wrapper![TailwindTextColor, TailwindRingColor, TailwindBackgroundColor];
