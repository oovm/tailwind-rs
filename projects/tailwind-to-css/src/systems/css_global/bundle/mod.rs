use super::*;
mod traits;

/// A collection of css objects
///
/// Separate or merge as needed
#[derive(Debug, Clone, Default, Ord, PartialOrd, Eq, PartialEq)]
pub(crate) struct CssBundle {
    mode: CssInlineMode,
    non_inlined_classes: BTreeSet<String>,
    attribute: CssAttributes,
    addition: BTreeSet<String>,
}

// noinspection DuplicatedCode
impl CssBundle {
    pub fn add_trace(&mut self, item: &CssInstance) {
        self.non_inlined_classes.insert(item.get_class());
    }
    /// insert new css instance to the html tag
    pub fn add_inline(&mut self, item: CssInstance) {
        self.attribute += item.attribute;
        self.addition.insert(item.addition);
    }
    pub fn obfuscate(css: &Self) -> String {
        let mut hasher = Xxh3::new();
        css.attribute.hash(&mut hasher);
        css.addition.hash(&mut hasher);
        base64(hasher.finish())
    }
    /// # Returns
    /// - css classes
    pub fn as_traced(&self) -> String {
        self.non_inlined_classes.iter().join(" ")
    }
    /// # Returns
    /// - `.0`: css classes rest, maybe empty
    /// - `.1`: css style
    pub fn as_inlined(&self) -> (String, String) {
        (self.as_traced(), self.attribute.to_string())
    }
    /// # Returns
    /// - scoped class name
    pub fn as_scope(&self) -> String {
        let id = Self::obfuscate(self);
        self.non_inlined_classes.iter().chain(&[id]).join(" ")
    }
    /// # Returns
    /// - data name without value
    pub fn as_data_key(&self) {
        todo!()
    }
    /// # Returns
    /// - scoped class name
    pub fn as_data_value(&self) {
        todo!()
    }

    pub fn set_mode(&mut self, mode: CssInlineMode) {
        self.mode = mode
    }
    pub fn write_css(&self, f: &mut (dyn Write)) -> Result<()> {
        let id = Self::obfuscate(self);
        match self.mode {
            CssInlineMode::None => unreachable!(),
            CssInlineMode::Inline => return Ok(()),
            CssInlineMode::Scoped => write!(f, ".{}", id)?,
            CssInlineMode::DataKey => write!(f, "[data-tw-{}]", id)?,
            CssInlineMode::DataValue => write!(f, "[data-tw=\"{}\"]", id)?,
        }
        f.write_char('{')?;
        write!(f, "{}", self.attribute)?;
        f.write_char('}')?;
        for item in &self.addition {
            write!(f, "{}", item)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Inlined {
    pub class: String,
    pub style: String,
}
