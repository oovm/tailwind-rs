use super::*;

impl Display for TailwindVariant {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.not {
            f.write_str("not-")?
        }
        for (i, s) in self.names.iter().enumerate() {
            f.write_str(s)?;
            if i + 1 != self.names.len() {
                f.write_char('-')?
            }
        }
        Ok(())
    }
}

impl Debug for TailwindInstruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let w = &mut f.debug_struct("TailwindStyle");
        w.field("negative", &self.negative);
        let variants: Vec<_> = self.variants.iter().map(|e| e.to_string()).collect();
        w.field("variants", &variants);
        let elements: Vec<_> = self.elements.iter().map(|e| &e.0).collect();
        w.field("elements", &elements);
        if let Some(s) = &self.arbitrary {
            w.field("arbitrary", s);
        }
        w.finish()
    }
}
