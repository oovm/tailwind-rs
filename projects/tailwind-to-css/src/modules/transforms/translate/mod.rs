use super::*;

#[derive(Copy, Clone, Debug)]
enum TranslateSize {}

#[doc = include_str!("readme.md")]
#[derive(Copy, Clone, Debug)]
pub struct TailwindTranslate {
    size: TranslateSize,
    axis: Option<bool>,
}
impl Display for TailwindTranslate {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindTranslate {}
