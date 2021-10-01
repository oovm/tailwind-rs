use super::*;

#[derive(Copy, Clone, Debug)]
enum BoxSizing {
    Border,
    Content,
}

#[doc = include_str!("readme.md")]
#[derive(Copy, Clone, Debug)]
pub struct TailwindBoxSizing {
    kind: BoxSizing,
}
