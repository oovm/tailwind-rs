pub mod attribute;
pub mod instance;

use crate::Result;
use std::{
    cmp::Ordering,
    fmt::{Debug, Formatter, Write},
    hash::{Hash, Hasher},
};
use std::fmt::Display;
use text_utils::indent;

pub trait TailwindInstance {
    /// used to deduplication and marking
    fn id(&self) -> String;
    // const ID: &'static str;

    fn selectors(&self) -> String {
        format!(".{}", self.id())
    }
    fn attributes(&self) -> Vec<CssAttribute> {
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

#[derive(Debug, Clone)]
pub struct CssAttribute {
    key: String,
    value: String,
}

