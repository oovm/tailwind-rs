use crate::{CssAttribute, TailwindInstance};
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Debug)]
enum BackgroundOrigin {
    Border,
    Padding,
    Content,
}

// https://tailwindcss.com/docs/background-origin
#[derive(Copy, Clone, Debug)]
pub struct TailwindBackgroundOrigin {
    kind: BackgroundOrigin,
}

// bg-origin-border	background-origin: border-box;
// bg-origin-padding	background-origin: padding-box;
// bg-origin-content	background-origin: content-box;
impl Display for BackgroundOrigin {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Display for TailwindBackgroundOrigin {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindBackgroundOrigin {}

impl TailwindBackgroundOrigin {
    /// `bg-clip-border`
    pub const Border: Self = Self { kind: BackgroundOrigin::Border };
    /// `bg-clip-padding`
    pub const Padding: Self = Self { kind: BackgroundOrigin::Padding };
    /// `bg-clip-content`
    pub const Content: Self = Self { kind: BackgroundOrigin::Content };
}
