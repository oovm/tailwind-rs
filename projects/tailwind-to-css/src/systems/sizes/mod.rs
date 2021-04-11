use std::fmt::{Display, Formatter};
use css_style::unit::Length;

pub struct SizingSystem {}

/// ## `w`
/// https://www.tailwindcss.cn/docs/width
pub enum TailwindWidth {
    Min,
    Max,
    Fit,
    Screen,
    Full,
    Px(usize),
    Rem(usize),
    Percent(usize, usize),
}


impl TailwindWidth {
    pub fn new() {}
    pub fn px() {}
}


impl Display for TailwindWidth {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}