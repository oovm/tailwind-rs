use crate::Result;
use std::{
    cmp::Ordering,
    fmt::{Debug, Formatter, Write},
    hash::{Hash, Hasher},
};
use text_utils::indent;

pub trait CssInstance {
    /// used to deduplication and marking
    fn id(&self) -> &'static str;
    // const ID: &'static str;

    fn selectors(&self) -> &'static str {
        "*"
    }
    fn attributes(&self) -> Vec<&'static str> {
        vec![]
    }

    fn write_css(&self, f: &mut (dyn Write)) -> Result<()> {
        writeln!(f, "{} {{", self.selectors())?;
        for item in self.attributes() {
            writeln!(f, "{}", indent(item, 4))?
        }
        writeln!(f, "}}")?;
        Ok(())
    }
}

impl Debug for Box<dyn CssInstance> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.id())
    }
}

impl Hash for Box<dyn CssInstance> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id().hash(state)
    }
}

impl PartialEq for Box<dyn CssInstance> {
    fn eq(&self, other: &Self) -> bool {
        self.id().eq(other.id())
    }
}

impl Eq for Box<dyn CssInstance> {}

impl PartialOrd for Box<dyn CssInstance> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.id().partial_cmp(other.id())
    }
}

impl Ord for Box<dyn CssInstance> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id().cmp(other.id())
    }
}
