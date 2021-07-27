use super::*;

impl Display for TailwindInstruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for v in &self.variants {
            write!(f, "{}", v)?
        }
        if self.negative {
            write!(f, "-")?
        }
        write!(f, "{}{}", self.elements, self.arbitrary)
    }
}

impl Display for TailwindVariant {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.not {
            write!(f, "not-")?
        }
        write!(f, "{}", self.names.join("-"))?;
        match self.pseudo {
            true => {write!(f, "::")}
            false => {write!(f, ":")}
        }
    }
}

impl Display for TailwindElements {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner.join("-"))
    }
}

impl Display for TailwindArbitrary {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.inner {
            None => {
                write!(f, "")
            }
            Some(s) => {
                write!(f, "-[{}]", s)
            }
        }
    }
}
