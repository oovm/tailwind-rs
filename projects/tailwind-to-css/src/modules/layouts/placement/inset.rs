use super::*;

#[derive(Clone, Debug)]
pub struct TailwindInset {
    negative: bool,
    axis: Option<bool>,
    kind: PlacementSize,
}
