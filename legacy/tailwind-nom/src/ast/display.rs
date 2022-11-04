use super::*;

impl<'a> Display for AstGroup<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let children: Vec<_> = self.children.iter().map(|s| s.to_string()).collect();
        write!(f, "{}({})", self.head, children.join(" "))
    }
}

impl<'a> Display for AstGroupItem<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Grouped(g) => Display::fmt(g, f),
            Self::Styled(g) => Display::fmt(g, f),
        }
    }
}

impl<'a> Display for AstStyle<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for v in &self.variants {
            write!(f, "{}", v)?
        }
        if self.negative {
            write!(f, "-")?
        }
        write!(f, "{}", self.elements.join("-"))?;
        match self.arbitrary {
            None => {}
            Some(s) => write!(f, "-[{}]", s)?,
        }
        Ok(())
    }
}

impl<'a> Display for ASTVariant<'a> {
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
