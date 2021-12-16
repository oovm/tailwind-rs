use super::*;

pub use self::font_size::FontSize;

mod font_size;

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
        new.insert_size("sm", FontSize::new(0.875, 1.25));
        new.insert_size("md", FontSize::new(1.0, 1.5));
        new.insert_size("lg", FontSize::new(1.125, 1.75));
        new.insert_size("xl", FontSize::new(1.25, 1.75));
        new.insert_size("2xl", FontSize::new(1.5, 2.0));
        new.insert_size("3xl", FontSize::new(1.875, 2.25));
        new.insert_size("4xl", FontSize::new(2.25, 2.5));
        new.insert_size("5xl", FontSize::new(3.0, -1.0));
        new.insert_size("6xl", FontSize::new(3.75, -1.0));
        new.insert_size("7xl", FontSize::new(4.5, -1.0));
        new.insert_size("8xl", FontSize::new(6.0, -1.0));
        new.insert_size("9xl", FontSize::new(8.0, -1.0));
        new.insert_family("sans", r#"ui-sans-serif"#);
        new.insert_family("serif", r#"ui-serif"#);
        new.insert_family("mono", r#"ui-sans-monospace"#);
        new
    }
    #[inline]
    pub fn get_size(&self, name: &str) -> FontSize {
        match self.size.get(name).cloned() {
            None => FontSize::new(1.0, -1.0),
            Some(s) => s,
        }
    }
    /// Insert a new font size
    #[inline]
    pub fn insert_size(&mut self, name: impl Into<String>, size: FontSize) -> Option<FontSize> {
        self.size.insert(name.into(), size)
    }
    /// Get the named font family, 
    /// 
    /// never fail, fallback to the `serif, sans-serif, monospace`
    #[inline]
    pub fn get_family(&self, name: &str) -> String {
        match self.family.get(name) {
            None => String::new(),
            Some(s) => s.join(", "),
        }
    }
    /// Insert a new font family
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
