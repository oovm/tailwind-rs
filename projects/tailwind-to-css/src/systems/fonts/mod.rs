use super::*;
mod font_size;
pub use self::font_size::FontSize;

#[derive(Clone, Debug, Default)]
pub struct FontSystem {
    size: HashMap<String, FontSize>,
    family: HashMap<String, Vec<String>>,
}

impl FontSystem {
    /// Builtin fonts' config
    pub fn builtin() -> Self {
        let mut new = Self::default();
        // https://tailwindcss.com/docs/font-size
        new.insert_size("xs", FontSize::new(0.75, 1.0));
        new.insert_size("sm", FontSize::new(0.875, 1.0));
        new.insert_size("md", FontSize::new(1.0, 1.0));
        new.insert_size("lg", FontSize::new(1.125, 1.0));
        new.insert_size("xl", FontSize::new(1.25, 1.0));
        new.insert_size("2xl", FontSize::new(1.5, 1.0));
        new.insert_family("sans", r#"ui-sans-serif"#);
        new.insert_family("serif", r#"ui-serif"#);
        new.insert_family("mono", r#"ui-sans-monospace"#);
        new
    }
    #[inline]
    pub fn get_size(&self, _name: &str) {
        todo!()
    }
    #[inline]
    pub fn insert_size(&mut self, name: impl Into<String>, size: FontSize) -> Option<FontSize> {
        self.size.insert(name.into(), size)
    }
    #[inline]
    pub fn get_family(&self, name: &str) -> String {
        match self.family.get(name) {
            None => String::new(),
            Some(s) => s.join(", "),
        }
    }
    #[inline]
    pub fn insert_family(&mut self, name: impl Into<String>, family: &str) -> Option<Vec<String>> {
        let family = Self::normalize_family(family)?;
        self.family.insert(name.into(), family)
    }
    #[inline]
    fn normalize_family(input: &str) -> Option<Vec<String>> {
        Some(vec![input.to_string()])
    }
}
