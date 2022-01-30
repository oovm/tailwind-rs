use super::*;

mod traits;

#[allow(clippy::derive_hash_xor_eq)]
#[derive(Debug, Clone, Hash)]
pub(crate) struct CssInstance {
    pub inlineable: bool,
    pub obfuscate: bool,
    pub selector: String,
    pub attribute: CssAttributes,
    pub addition: String,
}

// noinspection DuplicatedCode
impl CssInstance {
    pub fn new(item: &dyn TailwindInstance, ctx: &TailwindBuilder, obfuscate: bool) -> Self {
        Self {
            obfuscate,
            inlineable: item.inlineable(),
            selector: item.id(),
            attribute: item.attributes(ctx),
            addition: item.additional(ctx),
        }
    }

    pub fn obfuscate(css: &Self) -> String {
        let mut hasher = Xxh3::new();
        css.attribute.hash(&mut hasher);
        css.addition.hash(&mut hasher);
        base64(hasher.finish())
    }
    pub fn get_class(&self) -> String {
        match self.obfuscate {
            true => Self::obfuscate(self),
            false => self.selector.to_string(),
        }
    }
    /// write css to buffers
    pub fn write_css(&self, f: &mut (dyn Write)) -> Result<()> {
        f.write_char('.')?;
        normalize_class_name(f, &self.get_class())?;
        f.write_char('{')?;
        write!(f, "{}", self.attribute)?;
        f.write_char('}')?;
        write!(f, "{}", self.addition)?;
        Ok(())
    }
}
