use super::*;

#[derive(Copy, Debug, Clone)]
pub struct FontSize {
    size: LengthUnit,
    height: LengthUnit,
}

impl FontSize {
    #[inline]
    pub fn new(size: f32, height: f32) -> Self {
        let size = LengthUnit::rem(size);
        let height = if height < 0.0 { LengthUnit::rem(height) } else { LengthUnit::percent(height) };
        Self { size, height }
    }
}
