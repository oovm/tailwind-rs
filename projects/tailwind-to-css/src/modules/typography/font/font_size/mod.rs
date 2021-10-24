use super::*;

// #[doc = include_str!("font-size.md")]
#[derive(Copy, Debug, Clone)]
pub struct TailwindFontSize {
    size: LengthUnit,
    height: Option<LengthUnit>,
}

impl TailwindFontSize {
    #[inline]
    pub fn new(size: f32, height: f32) -> Self {
        let height = if height < 0.0 { None } else { Some(LengthUnit::percent(height)) };
        Self { size: LengthUnit::rem(size), height }
    }
}

impl Display for TailwindFontSize {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindFontSize {}
