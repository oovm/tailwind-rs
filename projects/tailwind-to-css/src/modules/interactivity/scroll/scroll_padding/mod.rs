use super::*;
use crate::{SpacingAxis, TailwindPadding};

#[doc=include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindScrollPadding {
    negative: bool,
    axis: SpacingAxis,
    size: Spacing,
}
