use std::fmt::{Display, Formatter};

pub struct BreakPointSystem {

}

/// sm	640px	@media (min-width: 640px) { ... }
pub struct BreakPoint {
    name: String,
    /// min-width
    /// unit: px
    width: usize
}

impl Display for BreakPoint {

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}