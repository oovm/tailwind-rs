use crate::Result;
use std::fmt::{Debug, Write};
use text_utils::indent;

pub trait CssInstance: Debug {
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
