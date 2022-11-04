use super::*;
use std::fmt::Write;

impl Display for AstStyle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for v in &self.variants {
            write!(f, "{}", v)?
        }
        write!(f, "{}", self.elements)?;
        write!(f, "{}", self.arbitrary)?;
        if self.important {
            write!(f, "!")?
        }
        Ok(())
    }
}

impl Display for ASTVariant {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.not {
            write!(f, "not-")?
        }
        write!(f, "{}", self.names.join("-"))?;
        match self.pseudo {
            true => write!(f, "::"),
            false => write!(f, ":"),
        }
    }
}

impl Display for AstArbitrary {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if !self.item.is_empty() {
            write!(f, "-[{}]", self.item.as_str())?
        }
        Ok(())
    }
}

impl Display for AstElements {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.negative {
            f.write_char('-')?
        }
        write!(f, "{}", self.items.join("-"))
    }
}
