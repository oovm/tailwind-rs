use super::*;

impl TailwindFontSmoothing {
    #[inline]
    pub fn new(subpixel: bool) -> Self {
        match subpixel {
            true => Self::Subpixel,
            false => Self::Normal,
        }
    }
}
