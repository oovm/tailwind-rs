pub mod attribute;
pub mod instance;

use crate::Result;
use std::{
    cmp::Ordering,
    collections::BTreeSet,
    fmt::{Debug, Display, Formatter, Write},
    hash::{Hash, Hasher},
};
use text_utils::indent;

pub trait TailwindInstance {
    /// used to deduplication and marking
    fn id(&self) -> String;
    // const ID: &'static str;

    fn selectors(&self) -> String {
        format!(".{}", self.id())
    }
    fn attributes(&self) -> BTreeSet<CssAttribute> {
        BTreeSet::default()
    }

    fn write_css(&self, f: &mut (dyn Write)) -> Result<()> {
        writeln!(f, "{} {{", self.selectors())?;
        for item in self.attributes() {
            writeln!(f, "{}", indent(item.to_string(), 4))?
        }
        writeln!(f, "}}")?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct CssAttribute {
    key: String,
    value: String,
}
