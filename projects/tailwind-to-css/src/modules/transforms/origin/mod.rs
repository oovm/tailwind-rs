use crate::AnchorPoint;

use super::*;

pub struct TailwindOrigin {
    kind: AnchorPoint,
}

impl Display for TailwindOrigin {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindOrigin {}
