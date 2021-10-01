use super::*;

#[derive(Copy, Clone, Debug)]
enum BoxDecoration {
    Clone,
    Slice,
}

#[doc = include_str!("readme.md")]
#[derive(Copy, Clone, Debug)]
pub struct TailwindBoxDecoration {
    kind: BoxDecoration,
}
