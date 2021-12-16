use super::*;

/// 
#[derive(Copy, Clone, Debug)]
pub struct SpacingAxis {
    class: &'static str,
    attributes: &'static [&'static str],
}

impl Display for SpacingAxis {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.class)
    }
}

impl SpacingAxis {
    /// 
    pub fn new(class: &'static str, attributes: &'static [&'static str]) -> Self {
        Self { class, attributes }
    }

    pub fn write_attributes(&self, css: &mut BTreeSet<CssAttribute>, value: String) {
        for attribute in self.attributes {
            let a = CssAttribute::new(attribute.to_string(), value.to_string());
            css.insert(a);
        }
    }
}
