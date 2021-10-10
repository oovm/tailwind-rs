use super::*;

#[derive(Clone, Debug)]
pub struct TailwindRight {
    negative: bool,
    axis: Option<bool>,
    kind: PlacementSize,
}
