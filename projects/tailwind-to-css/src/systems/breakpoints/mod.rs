use std::collections::HashMap;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub struct BreakPointSystem {
    inner: HashMap<String, BreakPoint>,
}

impl Default for BreakPointSystem {
    fn default() -> Self {
        Self {
            inner: Default::default()
        }
    }
}

impl BreakPointSystem {
    /// Builtin ranges
    /// https://tailwindcss.com/docs/screens
    pub fn builtin() -> Self {
        let mut new = Self::default();
        new.register("sm".to_string(), 640);
        new.register("md".to_string(), 768);
        new.register("lg".to_string(), 1024);
        new.register("xl".to_string(), 1280);
        new.register("2xl".to_string(), 1536);
        return new
    }

    #[inline]
    pub fn register(&mut self, name: String, width: usize) -> Option<BreakPoint> {
        self.inner.insert(name, BreakPoint {
            width
        })
    }
}

#[derive(Clone, Debug)]
pub struct BreakPoint {
    /// min-width
    /// unit: px
    width: usize,
}

impl Display for BreakPoint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f,"@media (min-width: {}px) {{", self.width)?;
        f.write_str("}")
    }
}