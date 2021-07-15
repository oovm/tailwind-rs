use super::*;

#[derive(Clone, Debug)]
pub struct FontSystem {
    size: HashMap<String, Font>,
    family: HashMap<String, Vec<String>>,
}

impl Default for FontSystem {
    fn default() -> Self {
        Self { size: Default::default(), family: Default::default() }
    }
}

impl FontSystem {
    /// Builtin ranges
    /// https://tailwindcss.com/docs/screens
    pub fn builtin() -> Self {
        let mut new = Self::default();
        new.insert_size("sm", 640);
        new.insert_size("md", 768);
        new.insert_size("lg", 1024);
        new.insert_size("xl", 1280);
        new.insert_size("2xl", 1536);
        new.insert_family("sans", r#"ui-sans-serif"#);
        new.insert_family("serif", r#"ui-serif"#);
        new.insert_family("mono", r#"ui-sans-monospace"#);
        return new;
    }
    #[inline]
    pub fn get_size(&self, name: &str) {
        todo!()
    }
    #[inline]
    pub fn insert_size(&mut self, name: impl Into<String>, width: usize) -> Option<Font> {
        self.size.insert(name.into(), Font { width })
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
        todo!()
    }
}

#[derive(Clone, Debug)]
pub struct Font {
    /// min-width
    /// unit: px
    width: usize,
}

impl Display for Font {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "@media (min-width: {}px) {{", self.width)?;
        f.write_str("}")
    }
}
