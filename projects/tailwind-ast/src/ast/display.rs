use super::*;

impl Display for AstStyle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for v in &self.variants {
            write!(f, "{}", v)?
        }
        write!(f, "{}", self.elements)?;
        if self.arbitrary.is_some() {
            write!(f, "-{}", self.arbitrary)?;
        }
        if self.important {
            write!(f, "!")?
        }
        Ok(())
    }
}

impl Display for TailwindVariant {
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
