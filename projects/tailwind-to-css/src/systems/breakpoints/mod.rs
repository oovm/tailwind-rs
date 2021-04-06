use std::collections::HashMap;
use std::fmt::{Display, Formatter};

pub struct BreakPointSystem {
    inner: HashMap<String, BreakPoint>,
}

impl BreakPointSystem {
    #[inline]
    pub fn register(&mut self, name: String, width: usize) -> Option<BreakPoint> {
        self.inner.insert(name, BreakPoint {
            width
        })
    }
}

/// sm	640px	@media (min-width: 640px) { ... }
pub struct BreakPoint {
    /// min-width
    /// unit: px
    width: usize,
}

impl Display for BreakPoint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}