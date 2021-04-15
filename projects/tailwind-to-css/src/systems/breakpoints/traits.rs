use super::*;

impl Default for BreakPointSystem {
    fn default() -> Self {
        Self { inner: Default::default() }
    }
}

impl Display for BreakPoint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "@media (min-width: {}px) {{", self.width)?;
        f.write_str("}")
    }
}
