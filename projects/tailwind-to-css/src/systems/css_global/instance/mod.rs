use std::hash::Hash;

use base64::{encode_config, URL_SAFE_NO_PAD};
use xxhash_rust::xxh3::Xxh3;

use super::*;

mod traits;

#[derive(Debug, Clone)]
pub struct CssInstance {
    pub inlinable: bool,
    pub selector: String,
    pub attribute: BTreeSet<CssAttribute>,
    pub addition: String,
}

impl CssInstance {
    pub fn new(item: &dyn TailwindInstance, ctx: &TailwindBuilder) -> Self {
        let mut selector = item.id();
        let attribute = item.attributes(ctx);
        let addition = item.additional(ctx);
        if ctx.obfuscate {
            let mut hasher = Xxh3::new();
            attribute.hash(&mut hasher);
            addition.hash(&mut hasher);
            selector = Self::base64(hasher.finish())
        }
        Self { inlinable: item.inlineable(), selector, attribute, addition }
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
    pub fn base64(hash: u64) -> String {
        encode_config(hash.to_be_bytes(), URL_SAFE_NO_PAD)
    }
}
