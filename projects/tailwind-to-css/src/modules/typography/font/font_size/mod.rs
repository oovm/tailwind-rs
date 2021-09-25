use super::*;

// #[doc = include_str!("font-size.md")]
#[derive(Copy, Debug, Clone)]
pub struct TailwindFontSize {
    size: TailwindTracking,
    height: TailwindLeading,
}

impl TailwindFontSize {
    #[inline]
    pub fn new(size: f32, height: f32) -> Self {
        todo!()
    }
}

impl Display for TailwindFontSize {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindFontSize {}
