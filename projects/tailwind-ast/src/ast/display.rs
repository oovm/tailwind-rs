use super::*;

impl Display for AstStyle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for v in &self.variants {
            write!(f, "{}", v)?
        }
        if self.negative {
            write!(f, "-")?
        }
        write!(f, "{}", self.elements.join("-"))?;
        if !self.arbitrary.is_empty() {
            write!(f, "-[{}]", self.arbitrary.as_str())?
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
