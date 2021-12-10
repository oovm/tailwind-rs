use super::*;

mod traits;

#[derive(Debug, Clone)]
pub struct CssInstance {
    pub obfuscate: bool,
    pub inlinable: bool,
    pub selector: String,
    pub attribute: BTreeSet<CssAttribute>,
    pub addition: String,
}

impl CssInstance {
    pub fn new(item: &dyn TailwindInstance, ctx: &TailwindBuilder) -> Self {
        Self {
            obfuscate: ctx.obfuscate,
            inlinable: item.inlineable(),
            selector: item.id(),
            attribute: item.attributes(ctx),
            addition: item.additional(ctx),
        }
    }
    pub fn get_class(&self) -> String {
        self.selector.to_string()
    }
    /// write css to buffers
    pub fn write_css(&self, f: &mut (dyn Write)) -> Result<()> {
        for c in self.selector.chars() {
            match c {
                ' ' => write!(f, "_"),
                r @ ('.' | '-' | '_') => write!(f, "{}", r),
                a if a.is_alphanumeric() => write!(f, "{}", a),
                _ => write!(f, "\\{}", c),
            }?
        }
        f.write_char('{')?;
        for item in &self.attribute {
            write!(f, "{}", item)?
        }
        f.write_char('}')?;
        f.write_str(&self.addition)?;
        Ok(())
    }
}
