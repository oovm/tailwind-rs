use super::*;

#[derive(Clone, Debug)]
pub struct TailwindLeft {
    negative: bool,
    axis: Option<bool>,
    kind: PlacementSize,
}
