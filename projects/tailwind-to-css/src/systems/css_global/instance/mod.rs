use super::*;

mod traits;

#[derive(Debug, Clone)]
pub struct CssInstance {
    pub obfuscate: bool,
    pub selector: String,
    pub attribute: BTreeSet<CssAttribute>,
}

impl CssInstance {
    pub fn from_trace(item: &Box<dyn TailwindInstance>, ctx: &TailwindBuilder) -> Self {
        let obfuscate = ctx.obfuscate;
        let mut selector = item.id();
        let attribute = item.attributes(ctx);
        Self { obfuscate, selector, attribute }
    }
    pub fn from_inline(item: &Box<dyn TailwindInstance>, ctx: &TailwindBuilder) -> Self {
        todo!()
    }
    pub fn from_scoped(item: &Box<dyn TailwindInstance>, obfuscate: bool) -> Self {
        todo!()
    }

    pub fn get_class(&self) -> String {
        todo!()
    }
    pub fn get_style(&self) -> String {
        todo!()
    }

    /// write css to buffers
    fn write_css(&self, f: &mut (dyn Write), _: &TailwindBuilder) -> Result<()> {
        for c in self.selector.chars() {
            match c {
                ' ' => write!(f, "_"),
                r @ ('.' | '-' | '_') => write!(f, "{}", r),
                a if a.is_alphanumeric() => write!(f, "{}", a),
                _ => write!(f, "\\{}", c),
            }?
        }
        f.write_char('{')?;
        for item in self.attribute {
            write!(f, "{}", item)?
        }
        f.write_char('}')?;
        Ok(())
    }
}
