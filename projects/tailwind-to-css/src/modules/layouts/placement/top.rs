use super::*;

#[derive(Clone, Debug)]
pub struct TailwindTop {
    negative: bool,
    axis: Option<bool>,
    kind: PlacementSize,
}
